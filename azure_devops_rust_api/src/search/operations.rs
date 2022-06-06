// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::{models, API_VERSION};
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self
            .scopes
            .unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(
        &self,
        request: impl Into<azure_core::Request>,
    ) -> Result<azure_core::Response, azure_core::Error> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn code_search_results(&self) -> code_search_results::Client {
        code_search_results::Client(self.clone())
    }
    pub fn package_search_results(&self) -> package_search_results::Client {
        package_search_results::Client(self.clone())
    }
    pub fn repositories(&self) -> repositories::Client {
        repositories::Client(self.clone())
    }
    pub fn tfvc(&self) -> tfvc::Client {
        tfvc::Client(self.clone())
    }
    pub fn wiki_search_results(&self) -> wiki_search_results::Client {
        wiki_search_results::Client(self.clone())
    }
    pub fn work_item_search_results(&self) -> work_item_search_results::Client {
        work_item_search_results::Client(self.clone())
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    PackageSearchResults_FetchPackageSearchResults(
        #[from] package_search_results::fetch_package_search_results::Error,
    ),
    #[error(transparent)]
    CodeSearchResults_FetchCodeSearchResults(
        #[from] code_search_results::fetch_code_search_results::Error,
    ),
    #[error(transparent)]
    Repositories_Get(#[from] repositories::get::Error),
    #[error(transparent)]
    Tfvc_Get(#[from] tfvc::get::Error),
    #[error(transparent)]
    WikiSearchResults_FetchWikiSearchResults(
        #[from] wiki_search_results::fetch_wiki_search_results::Error,
    ),
    #[error(transparent)]
    WorkItemSearchResults_FetchWorkItemSearchResults(
        #[from] work_item_search_results::fetch_work_item_search_results::Error,
    ),
}
pub mod package_search_results {
    use super::{models, API_VERSION};
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn fetch_package_search_results(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::PackageSearchRequest>,
        ) -> fetch_package_search_results::Builder {
            fetch_package_search_results::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
    }
    pub mod fetch_package_search_results {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::PackageSearchRequest,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::PackageSearchResponseContent, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/_apis/search/packagesearchresults",
                        self.client.endpoint(),
                        &self.organization
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::POST);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    req_builder = req_builder.header("content-type", "application/json");
                    let req_body = azure_core::to_json(&self.body).map_err(Error::Serialize)?;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::PackageSearchResponseContent =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod code_search_results {
    use super::{models, API_VERSION};
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn fetch_code_search_results(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::CodeSearchRequest>,
            project: impl Into<String>,
        ) -> fetch_code_search_results::Builder {
            fetch_code_search_results::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod fetch_code_search_results {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::CodeSearchRequest,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::CodeSearchResponse, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/{}/_apis/search/codesearchresults",
                        self.client.endpoint(),
                        &self.organization,
                        &self.project
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::POST);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    req_builder = req_builder.header("content-type", "application/json");
                    let req_body = azure_core::to_json(&self.body).map_err(Error::Serialize)?;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::CodeSearchResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod repositories {
    use super::{models, API_VERSION};
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
            repository: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
                repository: repository.into(),
            }
        }
    }
    pub mod get {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
            pub(crate) repository: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::RepositoryStatusResponse, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/{}/_apis/search/status/repositories/{}",
                        self.client.endpoint(),
                        &self.organization,
                        &self.project,
                        &self.repository
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::RepositoryStatusResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod tfvc {
    use super::{models, API_VERSION};
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn get(
            &self,
            organization: impl Into<String>,
            project: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                project: project.into(),
            }
        }
    }
    pub mod get {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::TfvcRepositoryStatusResponse, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/{}/_apis/search/status/tfvc",
                        self.client.endpoint(),
                        &self.organization,
                        &self.project
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::TfvcRepositoryStatusResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod wiki_search_results {
    use super::{models, API_VERSION};
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn fetch_wiki_search_results(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WikiSearchRequest>,
            project: impl Into<String>,
        ) -> fetch_wiki_search_results::Builder {
            fetch_wiki_search_results::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod fetch_wiki_search_results {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WikiSearchRequest,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::WikiSearchResponse, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/{}/_apis/search/wikisearchresults",
                        self.client.endpoint(),
                        &self.organization,
                        &self.project
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::POST);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    req_builder = req_builder.header("content-type", "application/json");
                    let req_body = azure_core::to_json(&self.body).map_err(Error::Serialize)?;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::WikiSearchResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
}
pub mod work_item_search_results {
    use super::{models, API_VERSION};
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn fetch_work_item_search_results(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::WorkItemSearchRequest>,
            project: impl Into<String>,
        ) -> fetch_work_item_search_results::Builder {
            fetch_work_item_search_results::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
                project: project.into(),
            }
        }
    }
    pub mod fetch_work_item_search_results {
        use super::{models, API_VERSION};
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse {
                status_code: http::StatusCode,
                body: bytes::Bytes,
            },
            #[error("Failed to parse request URL: {0}")]
            ParseUrl(url::ParseError),
            #[error("Failed to build request: {0}")]
            BuildRequest(http::Error),
            #[error("Failed to serialize request body: {0}")]
            Serialize(serde_json::Error),
            #[error("Failed to get access token: {0}")]
            GetToken(azure_core::Error),
            #[error("Failed to execute request: {0}")]
            SendRequest(azure_core::Error),
            #[error("Failed to get response bytes: {0}")]
            ResponseBytes(azure_core::StreamError),
            #[error("Failed to deserialize response: {0}, body: {1:?}")]
            Deserialize(serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::WorkItemSearchRequest,
            pub(crate) project: String,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<
                'static,
                std::result::Result<models::WorkItemSearchResponse, Error>,
            > {
                Box::pin(async move {
                    let url_str = &format!(
                        "{}/{}/{}/_apis/search/workitemsearchresults",
                        self.client.endpoint(),
                        &self.organization,
                        &self.project
                    );
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::POST);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(
                        http::header::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    url.query_pairs_mut()
                        .append_pair("api-version", super::API_VERSION);
                    req_builder = req_builder.header("content-type", "application/json");
                    let req_body = azure_core::to_json(&self.body).map_err(Error::Serialize)?;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            let rsp_value: models::WorkItemSearchResponse =
                                serde_json::from_slice(&rsp_body).map_err(|source| {
                                    Error::Deserialize(source, rsp_body.clone())
                                })?;
                            Ok(rsp_value)
                        }
                        status_code => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream)
                                .await
                                .map_err(Error::ResponseBytes)?;
                            Err(Error::UnexpectedResponse {
                                status_code,
                                body: rsp_body,
                            })
                        }
                    }
                })
            }
        }
    }
}