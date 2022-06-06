// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionsReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "reportName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub report_name: Option<String>,
    #[serde(
        rename = "reportStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub report_status: Option<permissions_report::ReportStatus>,
    #[serde(
        rename = "reportStatusLastUpdatedTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub report_status_last_updated_time: Option<String>,
    #[serde(
        rename = "requestedTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requestor: Option<String>,
}
pub mod permissions_report {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReportStatus {
        #[serde(rename = "created")]
        Created,
        #[serde(rename = "inProgress")]
        InProgress,
        #[serde(rename = "completedWithErrors")]
        CompletedWithErrors,
        #[serde(rename = "completedSuccessfully")]
        CompletedSuccessfully,
        #[serde(rename = "deleted")]
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionsReportRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub descriptors: Vec<String>,
    #[serde(
        rename = "reportName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub report_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<PermissionsReportResource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionsReportResource {
    #[serde(
        rename = "resourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_id: Option<String>,
    #[serde(
        rename = "resourceName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_name: Option<String>,
    #[serde(
        rename = "resourceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type: Option<permissions_report_resource::ResourceType>,
}
pub mod permissions_report_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceType {
        #[serde(rename = "repo")]
        Repo,
        #[serde(rename = "ref")]
        Ref,
        #[serde(rename = "projectGit")]
        ProjectGit,
        #[serde(rename = "release")]
        Release,
        #[serde(rename = "tfvc")]
        Tfvc,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
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
pub struct PermissionsReportList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PermissionsReport>,
}