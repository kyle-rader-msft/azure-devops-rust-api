use crate::path;
use autorust_openapi::{
    AdditionalProperties, CollectionFormat, DataType, MsExamples, OpenAPI, Operation, Parameter,
    ParameterType, PathItem, Reference, ReferenceOr, Response, Schema, SchemaCommon, StatusCode,
};
use indexmap::{IndexMap, IndexSet};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

/// An API specification
#[derive(Clone, Debug)]
pub struct Spec {
    /// A store of all the documents for an API specification keyed on their file paths where the first one is the root document
    docs: IndexMap<PathBuf, OpenAPI>,
    schemas: IndexMap<RefKey, Schema>,
    parameters: IndexMap<RefKey, Parameter>,
    input_files_paths: IndexSet<PathBuf>,
}

impl Spec {
    /// Read a list of input files as OpenApi docs with the first being the root doc
    ///
    /// This eagerly collects all the schemas and parametes for the docs
    pub fn read_files<P: AsRef<Path>>(input_files_paths: &[P]) -> Result<Self> {
        let mut docs: IndexMap<PathBuf, OpenAPI> = IndexMap::new();
        for file_path in input_files_paths {
            Spec::read_file(&mut docs, file_path)?;
        }

        let mut schemas: IndexMap<RefKey, Schema> = IndexMap::new();
        let mut parameters: IndexMap<RefKey, Parameter> = IndexMap::new();
        for (path, doc) in &docs {
            for (name, schema) in &doc.definitions {
                if let ReferenceOr::Item(schema) = schema {
                    let ref_key = RefKey {
                        file_path: path.clone(),
                        name: name.clone(),
                    };
                    schemas.insert(ref_key, schema.clone());
                }
            }

            for (name, param) in &doc.parameters {
                parameters.insert(
                    RefKey {
                        file_path: path.clone(),
                        name: name.clone(),
                    },
                    param.clone(),
                );
            }
        }

        Ok(Self {
            docs,
            schemas,
            parameters,
            input_files_paths: input_files_paths
                .iter()
                .map(|f| f.as_ref().to_owned())
                .collect(),
        })
    }

    /// Read a file and references too, recursively into the map
    fn read_file<P: AsRef<Path>>(
        docs: &mut IndexMap<PathBuf, OpenAPI>,
        file_path: P,
    ) -> Result<()> {
        let file_path = file_path.as_ref();
        if !docs.contains_key(file_path) {
            let doc = openapi::parse(&file_path)?;
            let ref_files = openapi::get_reference_file_paths(&file_path.to_path_buf(), &doc);
            docs.insert(PathBuf::from(file_path), doc);
            for ref_file in ref_files {
                let child_path = path::join(&file_path, &ref_file)
                    .map_err(|source| Error::PathJoin { source })?;
                Spec::read_file(docs, &child_path)?;
            }
        }
        Ok(())
    }

    pub fn docs(&self) -> &IndexMap<PathBuf, OpenAPI> {
        &self.docs
    }

    pub fn title(&self) -> Option<&str> {
        let mut titles: Vec<_> = self
            .docs
            .values()
            .map(|doc| &doc.info.title)
            .filter(|t| t.is_some())
            .flatten()
            .collect();
        titles.sort_unstable();

        titles.get(0).map(|t| t.as_str())
    }

    pub fn consumes(&self) -> Vec<&String> {
        let mut versions: Vec<_> = self
            .docs()
            .values()
            .filter(|doc| !doc.all_paths().is_empty())
            .map(|api| &api.consumes)
            .flatten()
            .collect();

        versions.sort_unstable();
        versions
    }

    /// Look for specs with operations and return the last one sorted alphabetically
    pub fn api_version(&self) -> Option<String> {
        let mut versions: Vec<&str> = self
            .docs()
            .values()
            .filter(|doc| !doc.all_paths().is_empty())
            .filter_map(|api| api.info.version.as_deref())
            .collect();
        versions.sort_unstable();
        versions.last().map(|version| version.to_string())
    }

    pub fn input_docs(&self) -> impl Iterator<Item = (&PathBuf, &OpenAPI)> {
        self.docs.iter().filter(move |(p, _)| self.is_input_file(p))
    }

    pub fn is_input_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.input_files_paths.contains(path.as_ref())
    }

    /// Find the schema for a given doc path and reference
    pub fn resolve_schema_ref<P: AsRef<Path>>(
        &self,
        doc_file: P,
        reference: Reference,
    ) -> Result<ResolvedSchema> {
        let doc_file = doc_file.as_ref();
        let full_path = match reference.file {
            None => doc_file.to_owned(),
            Some(file) => {
                path::join(doc_file, &file).map_err(|source| Error::PathJoin { source })?
            }
        };

        let name = reference.name.ok_or(Error::NoNameInReference)?;
        let ref_key = RefKey {
            file_path: full_path,
            name,
        };
        let schema = self
            .schemas
            .get(&ref_key)
            .ok_or_else(|| Error::SchemaNotFound {
                ref_key: ref_key.clone(),
            })?
            .clone();
        Ok(ResolvedSchema {
            ref_key: Some(ref_key),
            schema,
        })
    }

    /// Find the parameter for a given doc path and reference
    pub fn resolve_parameter_ref<P: AsRef<Path>>(
        &self,
        doc_file: P,
        reference: Reference,
    ) -> Result<Parameter> {
        let doc_file = doc_file.as_ref();
        let full_path = match reference.file {
            None => doc_file.to_owned(),
            Some(file) => {
                path::join(doc_file, &file).map_err(|source| Error::PathJoin { source })?
            }
        };
        let name = reference.name.ok_or(Error::NoNameInReference)?;
        let ref_key = RefKey {
            file_path: full_path,
            name,
        };
        Ok(self
            .parameters
            .get(&ref_key)
            .ok_or(Error::ParameterNotFound { ref_key })?
            .clone())
    }

    /// Resolve a reference or schema to a resolved schema
    fn resolve_schema<P: AsRef<Path>>(
        &self,
        doc_file: P,
        ref_or_schema: &ReferenceOr<Schema>,
    ) -> Result<ResolvedSchema> {
        match ref_or_schema {
            ReferenceOr::Item(schema) => Ok(ResolvedSchema {
                ref_key: None,
                schema: schema.clone(),
            }),
            ReferenceOr::Reference { reference, .. } => {
                self.resolve_schema_ref(doc_file.as_ref(), reference.clone())
            }
        }
    }

    /// Resolve a collection of references or schemas to a collection of resolved schemas
    pub fn resolve_schema_map<P: AsRef<Path>>(
        &self,
        doc_file: P,
        ref_or_schemas: &IndexMap<String, ReferenceOr<Schema>>,
    ) -> Result<IndexMap<String, ResolvedSchema>> {
        let mut resolved = IndexMap::new();
        for (name, schema) in ref_or_schemas {
            resolved.insert(name.clone(), self.resolve_schema(&doc_file, schema)?);
        }
        Ok(resolved)
    }

    pub fn resolve_path<P: AsRef<Path>>(
        &self,
        _doc_file: P,
        path: &ReferenceOr<PathItem>,
    ) -> Result<PathItem> {
        match path {
            ReferenceOr::Item(path) => Ok(path.clone()),
            ReferenceOr::Reference { .. } => {
                // self.resolve_path_ref(doc_file, reference),
                // TODO
                Err(Error::NotImplemented)
            }
        }
    }

    pub fn resolve_path_map(
        &self,
        doc_file: &Path,
        paths: &IndexMap<String, ReferenceOr<PathItem>>,
    ) -> Result<IndexMap<String, PathItem>> {
        let mut resolved = IndexMap::new();
        for (name, path) in paths {
            resolved.insert(name.clone(), self.resolve_path(doc_file, path)?);
        }
        Ok(resolved)
    }

    fn resolve_parameter(
        &self,
        doc_file: &Path,
        parameter: &ReferenceOr<Parameter>,
    ) -> Result<Parameter> {
        match parameter {
            ReferenceOr::Item(param) => Ok(param.clone()),
            ReferenceOr::Reference { reference, .. } => {
                self.resolve_parameter_ref(doc_file, reference.clone())
            }
        }
    }

    fn resolve_parameters(
        &self,
        doc_file: &Path,
        parameters: &[ReferenceOr<Parameter>],
    ) -> Result<Vec<WebParameter>> {
        let mut resolved = Vec::new();
        for param in parameters {
            resolved.push(WebParameter(self.resolve_parameter(doc_file, param)?));
        }
        Ok(resolved)
    }

    // only operations from listed input files
    fn operations_unresolved(&self) -> Result<Vec<WebOperationUnresolved>> {
        let mut operations: Vec<WebOperationUnresolved> = Vec::new();
        for (doc_file, doc) in self.docs() {
            if self.is_input_file(&doc_file) {
                let paths = self.resolve_path_map(doc_file, &doc.all_paths())?;
                for (path, item) in paths {
                    operations.extend(path_operations_unresolved(doc_file, &path, &item))
                }
            }
        }
        Ok(operations)
    }

    // only operations from listed input files
    pub fn operations(&self) -> Result<Vec<WebOperation>> {
        self.operations_unresolved()?
            .into_iter()
            .map({
                |op| {
                    Ok(WebOperation {
                        id: op.id,
                        path: op.path,
                        verb: op.verb,
                        parameters: self.resolve_parameters(&op.doc_file, &op.parameters)?,
                        responses: op.responses,
                        examples: op.examples,
                        summary: op.summary,
                    })
                }
            })
            .collect()
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("PathJoin")]
    PathJoin { source: path::Error },
    #[error("SchemaNotFound {} {}", ref_key.file_path.display(), ref_key.name)]
    SchemaNotFound { ref_key: RefKey },
    #[error("NoNameInReference")]
    NoNameInReference,
    #[error("ParameterNotFound")]
    ParameterNotFound { ref_key: RefKey },
    #[error("NotImplemented")]
    NotImplemented,
    #[error("ReadFile")]
    ReadFile {
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("DeserializeYaml")]
    DeserializeYaml {
        source: serde_yaml::Error,
        path: PathBuf,
    },
    #[error("DeserializeJson")]
    DeserializeJson {
        source: serde_json::Error,
        path: PathBuf,
    },
    #[error("TypeName {0}")]
    TypeName(#[source] Box<crate::codegen::Error>),
    #[error("creating function name: {0}")]
    FunctionName(#[source] crate::identifier::Error),
    #[error("ArrayExpectedToHaveItems")]
    ArrayExpectedToHaveItems,
    #[error("NoNameForRef")]
    NoNameForRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RefKey {
    pub file_path: PathBuf,
    pub name: String,
}

pub struct ResolvedSchema {
    pub ref_key: Option<RefKey>,
    pub schema: Schema,
}

/// Functionality related to Open API definitions
pub mod openapi {
    use super::*;

    /// Parse an OpenAPI object from a file located at `path`
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<OpenAPI> {
        let path = path.as_ref();
        let bytes = fs::read(path).map_err(|source| Error::ReadFile {
            source,
            path: PathBuf::from(path),
        })?;
        // Strip BOM (Byte Order Mark) if present
        let bytes = match bytes[..] {
            [0xEF, 0xBB, 0xBF, ..] => &bytes[3..],
            _ => &bytes,
        };
        let api = if path.extension() == Some(OsStr::new("yaml"))
            || path.extension() == Some(OsStr::new("yml"))
        {
            serde_yaml::from_slice(&bytes).map_err(|source| Error::DeserializeYaml {
                source,
                path: PathBuf::from(path),
            })?
        } else {
            serde_json::from_slice(&bytes).map_err(|source| Error::DeserializeJson {
                source,
                path: PathBuf::from(path),
            })?
        };

        Ok(api)
    }

    /// Returns a set of referenced relative file paths from an OpenAPI specficiation
    pub fn get_reference_file_paths<P: AsRef<Path>>(
        doc_file: P,
        api: &OpenAPI,
    ) -> IndexSet<String> {
        get_references(doc_file, api)
            .into_iter()
            .filter_map(|reference| match reference {
                TypedReference::Example(_) => None,
                reference => {
                    let reference: Reference = reference.into();
                    reference.file
                }
            })
            .collect()
    }

    /// Returns the list of all references contained in an OpenAPI schema
    pub fn get_references<P: AsRef<Path>>(doc_file: P, api: &OpenAPI) -> Vec<TypedReference> {
        let mut list = Vec::new();

        // paths and operations
        for (path, item) in &api.all_paths() {
            match item {
                ReferenceOr::Reference { reference, .. } => {
                    list.push(TypedReference::PathItem(reference.clone()))
                }
                ReferenceOr::Item(item) => {
                    for operation in path_operations_unresolved(&doc_file, path, item) {
                        // parameters
                        for param in &operation.parameters {
                            match param {
                                ReferenceOr::Reference { reference, .. } => {
                                    list.push(TypedReference::Parameter(reference.clone()))
                                }
                                ReferenceOr::Item(parameter) => match &parameter.schema {
                                    Some(ReferenceOr::Reference { reference, .. }) => {
                                        list.push(TypedReference::Schema(reference.clone()))
                                    }
                                    Some(ReferenceOr::Item(schema)) => {
                                        add_references_for_schema(&mut list, schema)
                                    }
                                    None => {}
                                },
                            }
                        }

                        // responses
                        for (_code, rsp) in &operation.responses {
                            match &rsp.schema {
                                Some(ReferenceOr::Reference { reference, .. }) => {
                                    list.push(TypedReference::Schema(reference.clone()))
                                }
                                Some(ReferenceOr::Item(schema)) => {
                                    add_references_for_schema(&mut list, schema)
                                }
                                None => {}
                            }
                        }

                        // examples
                        for (_name, example) in &operation.examples {
                            if let ReferenceOr::Reference { reference, .. } = example {
                                list.push(TypedReference::Example(reference.clone()));
                            }
                        }
                    }
                }
            }
        }

        // definitions
        for (_name, schema) in &api.definitions {
            match schema {
                ReferenceOr::Reference { reference, .. } => {
                    list.push(TypedReference::Schema(reference.clone()))
                }
                ReferenceOr::Item(schema) => add_references_for_schema(&mut list, schema),
            }
        }

        list
    }

    /// Get all references related to schemas for an Open API specification
    pub fn get_api_schema_references(doc_file: &Path, api: &OpenAPI) -> Vec<Reference> {
        get_references(doc_file, api)
            .into_iter()
            .filter_map(|rf| match rf {
                TypedReference::Schema(rs) => Some(rs),
                _ => None,
            })
            .collect()
    }
}

// contains unresolved parameters
struct WebOperationUnresolved {
    pub doc_file: PathBuf,
    pub id: Option<String>,
    pub path: String,
    pub verb: WebVerb,
    pub parameters: Vec<ReferenceOr<Parameter>>,
    pub responses: IndexMap<StatusCode, Response>,
    pub examples: MsExamples,
    pub summary: Option<String>,
}

// contains resolved parameters
pub struct WebOperation {
    pub id: Option<String>,
    pub path: String,
    pub verb: WebVerb,
    pub parameters: Vec<WebParameter>,
    pub responses: IndexMap<StatusCode, Response>,
    pub examples: MsExamples,
    pub summary: Option<String>,
}

impl Default for WebOperation {
    fn default() -> Self {
        Self {
            id: Default::default(),
            path: Default::default(),
            verb: WebVerb::Get,
            parameters: Default::default(),
            responses: Default::default(),
            examples: Default::default(),
            summary: Default::default(),
        }
    }
}

pub struct WebParameter(Parameter);

impl WebParameter {
    pub fn name(&self) -> &str {
        self.0.name.as_str()
    }

    pub fn required(&self) -> bool {
        self.0.required.unwrap_or(false)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self.0.common.type_, Some(DataType::Boolean))
    }

    pub fn collection_format(&self) -> &CollectionFormat {
        self.0
            .collection_format
            .as_ref()
            .unwrap_or(&CollectionFormat::Csv)
    }

    pub fn type_(&self) -> &ParameterType {
        &self.0.in_
    }

    pub fn data_type(&self) -> &Option<DataType> {
        &self.0.common.type_
    }

    pub fn is_array(&self) -> bool {
        matches!(self.data_type(), Some(DataType::Array))
    }

    pub fn is_string(&self) -> bool {
        matches!(self.data_type(), Some(DataType::String))
    }

    pub fn type_name(&self) -> Result<TypeName, Error> {
        Ok(if let Some(_data_type) = self.data_type() {
            get_type_name_for_schema(&self.0.common)?
        } else if let Some(schema) = &self.0.schema {
            get_type_name_for_schema_ref(schema)?
        } else {
            // eprintln!("WARN unknown param type name for {}", self.name());
            TypeName::Value
        })
    }

    pub fn type_is_ref(&self) -> Result<bool, Error> {
        Ok(if let Some(data_type) = self.data_type() {
            matches!(
                data_type,
                DataType::String | DataType::Object | DataType::File
            )
        } else {
            true
        })
    }
}

impl WebOperation {
    pub fn parameters(&self) -> Vec<&WebParameter> {
        self.parameters.iter().collect()
    }

    pub fn id_parts(&self) -> Vec<&str> {
        match &self.id {
            Some(id) => id.splitn(2, '_').collect(),
            None => Vec::new(),
        }
    }

    pub fn in_group(&self) -> bool {
        self.id_parts().len() == 2
    }

    pub fn has_body_parameter(&self) -> bool {
        self.parameters
            .iter()
            .any(|p| p.type_() == &ParameterType::Body)
    }
}

pub enum WebVerb {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Head,
}

impl<'a> WebVerb {
    pub fn as_str(&self) -> &'static str {
        match self {
            WebVerb::Get => "get",
            WebVerb::Post => "post",
            WebVerb::Put => "put",
            WebVerb::Patch => "patch",
            WebVerb::Delete => "delete",
            WebVerb::Options => "options",
            WebVerb::Head => "head",
        }
    }
}

struct OperationVerb<'a> {
    pub operation: Option<&'a Operation>,
    pub verb: WebVerb,
}

fn path_operations_unresolved<P: AsRef<Path>>(
    doc_file: P,
    path: &str,
    item: &PathItem,
) -> Vec<WebOperationUnresolved> {
    vec![
        OperationVerb {
            operation: item.get.as_ref(),
            verb: WebVerb::Get,
        },
        OperationVerb {
            operation: item.post.as_ref(),
            verb: WebVerb::Post,
        },
        OperationVerb {
            operation: item.put.as_ref(),
            verb: WebVerb::Put,
        },
        OperationVerb {
            operation: item.patch.as_ref(),
            verb: WebVerb::Patch,
        },
        OperationVerb {
            operation: item.delete.as_ref(),
            verb: WebVerb::Delete,
        },
        OperationVerb {
            operation: item.options.as_ref(),
            verb: WebVerb::Options,
        },
        OperationVerb {
            operation: item.head.as_ref(),
            verb: WebVerb::Head,
        },
    ]
    .into_iter()
    .filter_map(|op_verb| match op_verb.operation {
        Some(op) => {
            let mut parameters = item.parameters.clone();
            parameters.append(&mut op.parameters.clone());
            Some(WebOperationUnresolved {
                doc_file: doc_file.as_ref().to_path_buf(),
                id: op.operation_id.clone(),
                path: path.to_string(),
                verb: op_verb.verb,
                parameters,
                responses: op.responses.clone(),
                examples: op.x_ms_examples.clone(),
                summary: op.summary.clone(),
            })
        }
        None => None,
    })
    .collect()
}

/// A $ref reference type that knows what type of reference it is
#[derive(Clone, Debug, PartialEq)]
pub enum TypedReference {
    PathItem(Reference),
    Parameter(Reference),
    Schema(Reference),
    Example(Reference),
}

impl From<TypedReference> for Reference {
    fn from(s: TypedReference) -> Reference {
        match s {
            TypedReference::PathItem(r) => r,
            TypedReference::Parameter(r) => r,
            TypedReference::Schema(r) => r,
            TypedReference::Example(r) => r,
        }
    }
}

/// Get all schema references for a given schema
pub fn get_schema_schema_references(schema: &Schema) -> Vec<Reference> {
    let mut refs = Vec::new();
    add_references_for_schema(&mut refs, schema);
    refs.into_iter()
        .filter_map(|rf| match rf {
            TypedReference::Schema(rs) => Some(rs),
            _ => None,
        })
        .collect()
}

fn add_references_for_schema(list: &mut Vec<TypedReference>, schema: &Schema) {
    for (_, schema) in &schema.properties {
        match schema {
            ReferenceOr::Reference { reference, .. } => {
                list.push(TypedReference::Schema(reference.clone()))
            }
            ReferenceOr::Item(schema) => add_references_for_schema(list, schema),
        }
    }

    if let Some(ap) = schema.additional_properties.as_ref() {
        match ap {
            AdditionalProperties::Boolean(_) => {}
            AdditionalProperties::Schema(schema) => match schema {
                ReferenceOr::Reference { reference, .. } => {
                    list.push(TypedReference::Schema(reference.clone()))
                }
                ReferenceOr::Item(schema) => add_references_for_schema(list, schema),
            },
        }
    }
    if let Some(schema) = schema.common.items.as_ref() {
        match schema {
            ReferenceOr::Reference { reference, .. } => {
                list.push(TypedReference::Schema(reference.clone()))
            }
            ReferenceOr::Item(schema) => add_references_for_schema(list, schema),
        }
    }
    for schema in &schema.all_of {
        match schema {
            ReferenceOr::Reference { reference, .. } => {
                list.push(TypedReference::Schema(reference.clone()))
            }
            ReferenceOr::Item(schema) => add_references_for_schema(list, schema),
        }
    }
}

pub enum TypeName {
    Reference(String),
    Array(Box<TypeName>),
    Value,
    Bytes,
    Int32,
    Int64,
    Float32,
    Float64,
    Boolean,
    String,
}

pub fn get_type_name_for_schema(schema: &SchemaCommon) -> Result<TypeName, Error> {
    Ok(if let Some(schema_type) = &schema.type_ {
        let format = schema.format.as_deref();
        match schema_type {
            DataType::Array => {
                let items = get_schema_array_items(schema)?;
                let vec_items_typ = get_type_name_for_schema_ref(items)?;
                TypeName::Array(Box::new(vec_items_typ))
            }
            DataType::Integer => {
                if format == Some("int32") {
                    TypeName::Int32
                } else {
                    TypeName::Int64
                }
            }
            DataType::Number => {
                if format == Some("float") {
                    TypeName::Float32
                } else {
                    TypeName::Float64
                }
            }
            DataType::String => TypeName::String,
            DataType::Boolean => TypeName::Boolean,
            DataType::Object => TypeName::Value,
            DataType::File => TypeName::Bytes,
        }
    } else {
        // eprintln!(
        //     "WARN unknown type in get_type_name_for_schema, description {:?}",
        //     schema.description
        // );
        TypeName::Value
    })
}

pub fn get_type_name_for_schema_ref(schema: &ReferenceOr<Schema>) -> Result<TypeName, Error> {
    Ok(match schema {
        ReferenceOr::Reference { reference, .. } => {
            let name = reference.name.as_ref().ok_or(Error::NoNameForRef)?;
            TypeName::Reference(name.to_owned())
        }
        ReferenceOr::Item(schema) => get_type_name_for_schema(&schema.common)?,
    })
}

pub fn get_schema_array_items(schema: &SchemaCommon) -> Result<&ReferenceOr<Schema>, Error> {
    //println!("get_schema_array_items: {:?}", schema);
    schema
        .items
        .as_ref()
        .as_ref()
        .ok_or(Error::ArrayExpectedToHaveItems)
}