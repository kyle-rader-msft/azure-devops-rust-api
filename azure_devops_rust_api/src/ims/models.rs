// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessTokenResult {
    #[serde(
        rename = "accessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<JsonWebToken>,
    #[serde(
        rename = "accessTokenError",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token_error: Option<access_token_result::AccessTokenError>,
    #[serde(
        rename = "authorizationId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_id: Option<String>,
    #[serde(
        rename = "errorDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub error_description: Option<String>,
    #[serde(rename = "hasError", default, skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(
        rename = "isFirstPartyClient",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_first_party_client: Option<bool>,
    #[serde(
        rename = "refreshToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refresh_token: Option<RefreshTokenGrant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "tokenType", default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
    #[serde(rename = "validTo", default, skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<String>,
}
pub mod access_token_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AccessTokenError {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "grantTypeRequired")]
        GrantTypeRequired,
        #[serde(rename = "authorizationGrantRequired")]
        AuthorizationGrantRequired,
        #[serde(rename = "clientSecretRequired")]
        ClientSecretRequired,
        #[serde(rename = "redirectUriRequired")]
        RedirectUriRequired,
        #[serde(rename = "invalidAuthorizationGrant")]
        InvalidAuthorizationGrant,
        #[serde(rename = "invalidAuthorizationScopes")]
        InvalidAuthorizationScopes,
        #[serde(rename = "invalidRefreshToken")]
        InvalidRefreshToken,
        #[serde(rename = "authorizationNotFound")]
        AuthorizationNotFound,
        #[serde(rename = "authorizationGrantExpired")]
        AuthorizationGrantExpired,
        #[serde(rename = "accessAlreadyIssued")]
        AccessAlreadyIssued,
        #[serde(rename = "invalidRedirectUri")]
        InvalidRedirectUri,
        #[serde(rename = "accessTokenNotFound")]
        AccessTokenNotFound,
        #[serde(rename = "invalidAccessToken")]
        InvalidAccessToken,
        #[serde(rename = "accessTokenAlreadyRefreshed")]
        AccessTokenAlreadyRefreshed,
        #[serde(rename = "invalidClientSecret")]
        InvalidClientSecret,
        #[serde(rename = "clientSecretExpired")]
        ClientSecretExpired,
        #[serde(rename = "serverError")]
        ServerError,
        #[serde(rename = "accessDenied")]
        AccessDenied,
        #[serde(rename = "accessTokenKeyRequired")]
        AccessTokenKeyRequired,
        #[serde(rename = "invalidAccessTokenKey")]
        InvalidAccessTokenKey,
        #[serde(rename = "failedToGetAccessToken")]
        FailedToGetAccessToken,
        #[serde(rename = "invalidClientId")]
        InvalidClientId,
        #[serde(rename = "invalidClient")]
        InvalidClient,
        #[serde(rename = "invalidValidTo")]
        InvalidValidTo,
        #[serde(rename = "invalidUserId")]
        InvalidUserId,
        #[serde(rename = "failedToIssueAccessToken")]
        FailedToIssueAccessToken,
        #[serde(rename = "authorizationGrantScopeMissing")]
        AuthorizationGrantScopeMissing,
        #[serde(rename = "invalidPublicAccessTokenKey")]
        InvalidPublicAccessTokenKey,
        #[serde(rename = "invalidPublicAccessToken")]
        InvalidPublicAccessToken,
        #[serde(rename = "publicFeatureFlagNotEnabled")]
        PublicFeatureFlagNotEnabled,
        #[serde(rename = "sshPolicyDisabled")]
        SshPolicyDisabled,
        #[serde(rename = "hostAuthorizationNotFound")]
        HostAuthorizationNotFound,
        #[serde(rename = "hostAuthorizationIsNotValid")]
        HostAuthorizationIsNotValid,
        #[serde(rename = "invalidScope")]
        InvalidScope,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationGrant {
    #[serde(rename = "grantType", default, skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<authorization_grant::GrantType>,
}
pub mod authorization_grant {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum GrantType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "jwtBearer")]
        JwtBearer,
        #[serde(rename = "refreshToken")]
        RefreshToken,
        #[serde(rename = "implicit")]
        Implicit,
        #[serde(rename = "clientCredentials")]
        ClientCredentials,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangedIdentities {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identities: Vec<Identity>,
    #[serde(rename = "moreData", default, skip_serializing_if = "Option::is_none")]
    pub more_data: Option<bool>,
    #[serde(
        rename = "sequenceContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sequence_context: Option<ChangedIdentitiesContext>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangedIdentitiesContext {
    #[serde(
        rename = "groupSequenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub group_sequence_id: Option<i32>,
    #[serde(
        rename = "identitySequenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_sequence_id: Option<i32>,
    #[serde(
        rename = "organizationIdentitySequenceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_identity_sequence_id: Option<i32>,
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateScopeInfo {
    #[serde(
        rename = "adminGroupDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_group_description: Option<String>,
    #[serde(
        rename = "adminGroupName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_group_name: Option<String>,
    #[serde(rename = "creatorId", default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[serde(
        rename = "parentScopeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_scope_id: Option<String>,
    #[serde(rename = "scopeName", default, skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<String>,
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<create_scope_info::ScopeType>,
}
pub mod create_scope_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "generic")]
        Generic,
        #[serde(rename = "serviceHost")]
        ServiceHost,
        #[serde(rename = "teamProject")]
        TeamProject,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FrameworkIdentityInfo {
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(
        rename = "identityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_type: Option<framework_identity_info::IdentityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
pub mod framework_identity_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum IdentityType {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "serviceIdentity")]
        ServiceIdentity,
        #[serde(rename = "aggregateIdentity")]
        AggregateIdentity,
        #[serde(rename = "importedIdentity")]
        ImportedIdentity,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMembership {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<IdentityDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "queriedId", default, skip_serializing_if = "Option::is_none")]
    pub queried_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    #[serde(flatten)]
    pub identity_base: IdentityBase,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityBase {
    #[serde(
        rename = "customDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<IdentityDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(
        rename = "isContainer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_container: Option<bool>,
    #[serde(rename = "masterId", default, skip_serializing_if = "Option::is_none")]
    pub master_id: Option<String>,
    #[serde(rename = "memberIds", default, skip_serializing_if = "Vec::is_empty")]
    pub member_ids: Vec<String>,
    #[serde(rename = "memberOf", default, skip_serializing_if = "Vec::is_empty")]
    pub member_of: Vec<IdentityDescriptor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<IdentityDescriptor>,
    #[serde(
        rename = "metaTypeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub meta_type_id: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PropertiesCollection>,
    #[serde(
        rename = "providerDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_display_name: Option<String>,
    #[serde(
        rename = "resourceVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_version: Option<i32>,
    #[serde(
        rename = "socialDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub social_descriptor: Option<String>,
    #[serde(
        rename = "subjectDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_descriptor: Option<String>,
    #[serde(
        rename = "uniqueUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_user_id: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityBatchInfo {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub descriptors: Vec<IdentityDescriptor>,
    #[serde(rename = "identityIds", default, skip_serializing_if = "Vec::is_empty")]
    pub identity_ids: Vec<String>,
    #[serde(
        rename = "includeRestrictedVisibility",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_restricted_visibility: Option<bool>,
    #[serde(
        rename = "propertyNames",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_names: Vec<String>,
    #[serde(
        rename = "queryMembership",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub query_membership: Option<identity_batch_info::QueryMembership>,
    #[serde(
        rename = "socialDescriptors",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub social_descriptors: Vec<String>,
    #[serde(
        rename = "subjectDescriptors",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subject_descriptors: Vec<String>,
}
pub mod identity_batch_info {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum QueryMembership {
        #[serde(rename = "none")]
        None,
        #[serde(rename = "direct")]
        Direct,
        #[serde(rename = "expanded")]
        Expanded,
        #[serde(rename = "expandedUp")]
        ExpandedUp,
        #[serde(rename = "expandedDown")]
        ExpandedDown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityDescriptor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(
        rename = "identityType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityRightsTransferData {
    #[serde(
        rename = "userPrincipalNameMappings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub user_principal_name_mappings: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityScope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrators: Option<IdentityDescriptor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(rename = "isGlobal", default, skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    #[serde(
        rename = "localScopeId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_scope_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<identity_scope::ScopeType>,
    #[serde(
        rename = "securingHostId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub securing_host_id: Option<String>,
    #[serde(
        rename = "subjectDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_descriptor: Option<String>,
}
pub mod identity_scope {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "generic")]
        Generic,
        #[serde(rename = "serviceHost")]
        ServiceHost,
        #[serde(rename = "teamProject")]
        TeamProject,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitySelf {
    #[serde(
        rename = "accountName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub account_name: Option<String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "originId", default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tenants: Vec<TenantInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentitySnapshot {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Identity>,
    #[serde(rename = "identityIds", default, skip_serializing_if = "Vec::is_empty")]
    pub identity_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memberships: Vec<GroupMembership>,
    #[serde(rename = "scopeId", default, skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<IdentityScope>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityUpdateData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuedToken {
    #[serde(
        rename = "isAuthenticated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_authenticated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonPatchDocument {
    #[serde(flatten)]
    pub vec_json_patch_operation: Vec<JsonPatchOperation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonPatchOperation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<json_patch_operation::Op>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
pub mod json_patch_operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Op {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "remove")]
        Remove,
        #[serde(rename = "replace")]
        Replace,
        #[serde(rename = "move")]
        Move,
        #[serde(rename = "copy")]
        Copy,
        #[serde(rename = "test")]
        Test,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonWebToken {
    #[serde(flatten)]
    pub issued_token: IssuedToken,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedIdentities {
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub continuation_token: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub identities: Vec<Identity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertiesCollection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RefreshTokenGrant {
    #[serde(flatten)]
    pub authorization_grant: AuthorizationGrant,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwt: Option<JsonWebToken>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwapIdentityInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id2: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantInfo {
    #[serde(
        rename = "homeTenant",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub home_tenant: Option<bool>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(
        rename = "tenantName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tenant_name: Option<String>,
    #[serde(
        rename = "verifiedDomains",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub verified_domains: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VssJsonCollectionWrapper {
    #[serde(flatten)]
    pub vss_json_collection_wrapper_base: VssJsonCollectionWrapperBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VssJsonCollectionWrapperBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Identity>,
}