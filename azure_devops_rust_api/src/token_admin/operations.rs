// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: crate::auth::Credential,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: crate::auth::Credential,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = "https://vssps.dev.azure.com";
impl ClientBuilder {
    pub fn new(credential: crate::auth::Credential) -> Self {
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
    pub(crate) fn credential(&self) -> &crate::auth::Credential {
        &self.credential
    }
    pub(crate) async fn send(
        &self,
        request: impl Into<azure_core::Request>,
    ) -> azure_core::error::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: crate::auth::Credential,
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
    pub fn personal_access_tokens(&self) -> personal_access_tokens::Client {
        personal_access_tokens::Client(self.clone())
    }
    pub fn revocation_rules(&self) -> revocation_rules::Client {
        revocation_rules::Client(self.clone())
    }
    pub fn revocations(&self) -> revocations::Client {
        revocations::Client(self.clone())
    }
}
pub mod personal_access_tokens {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn list(
            &self,
            organization: impl Into<String>,
            subject_descriptor: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                subject_descriptor: subject_descriptor.into(),
                page_size: None,
                continuation_token: None,
                is_public: None,
            }
        }
    }
    pub mod list {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = models::TokenAdminPagedSessionTokens;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) subject_descriptor: String,
            pub(crate) page_size: Option<i32>,
            pub(crate) continuation_token: Option<String>,
            pub(crate) is_public: Option<bool>,
        }
        impl Builder {
            pub fn page_size(mut self, page_size: i32) -> Self {
                self.page_size = Some(page_size);
                self
            }
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            pub fn is_public(mut self, is_public: bool) -> Self {
                self.is_public = Some(is_public);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/{}/_apis/tokenadmin/personalaccesstokens/{}",
                            this.client.endpoint(),
                            &this.organization,
                            &this.subject_descriptor
                        );
                        let mut url = url::Url::parse(url_str)
                            .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::GET);
                        req_builder = req_builder.header(
                            http::header::AUTHORIZATION,
                            &this
                                .client
                                .credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        url.query_pairs_mut()
                            .append_pair("api-version", "7.1-preview");
                        if let Some(page_size) = &this.page_size {
                            url.query_pairs_mut()
                                .append_pair("pageSize", &page_size.to_string());
                        }
                        if let Some(continuation_token) = &this.continuation_token {
                            url.query_pairs_mut()
                                .append_pair("continuationToken", continuation_token);
                        }
                        if let Some(is_public) = &this.is_public {
                            url.query_pairs_mut()
                                .append_pair("isPublic", &is_public.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder
                            .body(req_body)
                            .context(azure_core::error::ErrorKind::Other, "build request")?;
                        let rsp = this
                            .client
                            .send(req)
                            .await
                            .context(azure_core::error::ErrorKind::Io, "execute request")?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::OK => {
                                let rsp_body =
                                    azure_core::collect_pinned_stream(rsp_stream).await?;
                                let rsp_value: models::TokenAdminPagedSessionTokens =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code.as_u16(),
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod revocation_rules {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn create(
            &self,
            organization: impl Into<String>,
            body: impl Into<models::TokenAdminRevocationRule>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body: body.into(),
            }
        }
    }
    pub mod create {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: models::TokenAdminRevocationRule,
        }
        impl Builder {
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/{}/_apis/tokenadmin/revocationrules",
                            this.client.endpoint(),
                            &this.organization
                        );
                        let mut url = url::Url::parse(url_str)
                            .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::POST);
                        req_builder = req_builder.header(
                            http::header::AUTHORIZATION,
                            &this
                                .client
                                .credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        url.query_pairs_mut()
                            .append_pair("api-version", "7.1-preview");
                        req_builder = req_builder.header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder
                            .body(req_body)
                            .context(azure_core::error::ErrorKind::Other, "build request")?;
                        let rsp = this
                            .client
                            .send(req)
                            .await
                            .context(azure_core::error::ErrorKind::Io, "execute request")?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::CREATED => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code.as_u16(),
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
pub mod revocations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        pub fn revoke_authorizations(
            &self,
            organization: impl Into<String>,
            body: Vec<models::TokenAdminRevocation>,
        ) -> revoke_authorizations::Builder {
            revoke_authorizations::Builder {
                client: self.0.clone(),
                organization: organization.into(),
                body,
                is_public: None,
            }
        }
    }
    pub mod revoke_authorizations {
        use super::models;
        use azure_core::error::ResultExt;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) organization: String,
            pub(crate) body: Vec<models::TokenAdminRevocation>,
            pub(crate) is_public: Option<bool>,
        }
        impl Builder {
            pub fn is_public(mut self, is_public: bool) -> Self {
                self.is_public = Some(is_public);
                self
            }
            pub fn into_future(
                self,
            ) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>>
            {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url_str = &format!(
                            "{}/{}/_apis/tokenadmin/revocations",
                            this.client.endpoint(),
                            &this.organization
                        );
                        let mut url = url::Url::parse(url_str)
                            .context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                        let mut req_builder = http::request::Builder::new();
                        req_builder = req_builder.method(http::Method::POST);
                        req_builder = req_builder.header(
                            http::header::AUTHORIZATION,
                            &this
                                .client
                                .credential()
                                .http_authorization_header(&this.client.scopes)
                                .await?,
                        );
                        url.query_pairs_mut()
                            .append_pair("api-version", "7.1-preview");
                        req_builder = req_builder.header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(is_public) = &this.is_public {
                            url.query_pairs_mut()
                                .append_pair("isPublic", &is_public.to_string());
                        }
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder
                            .body(req_body)
                            .context(azure_core::error::ErrorKind::Other, "build request")?;
                        let rsp = this
                            .client
                            .send(req)
                            .await
                            .context(azure_core::error::ErrorKind::Io, "execute request")?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            http::StatusCode::NO_CONTENT => Ok(()),
                            status_code => Err(azure_core::error::Error::from(
                                azure_core::error::ErrorKind::HttpResponse {
                                    status: status_code.as_u16(),
                                    error_code: None,
                                },
                            )),
                        }
                    }
                })
            }
        }
    }
}
