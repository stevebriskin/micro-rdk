// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartBuildRequest {
    /// repo to build
    #[prost(string, tag="1")]
    pub repo: ::prost::alloc::string::String,
    /// optional git ref; defaults to 'main'
    #[prost(string, optional, tag="2")]
    pub r#ref: ::core::option::Option<::prost::alloc::string::String>,
    /// list of platforms to build
    #[prost(string, repeated, tag="3")]
    pub platforms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// module_id as prefix:name pair
    #[prost(string, tag="4")]
    pub module_id: ::prost::alloc::string::String,
    /// version of the module to publish
    /// must be valid semver2.0 string (ex: 1.2.3-rc0)
    #[prost(string, tag="5")]
    pub module_version: ::prost::alloc::string::String,
    /// checkout token. provide this for private repos
    #[prost(string, optional, tag="6")]
    pub token: ::core::option::Option<::prost::alloc::string::String>,
    /// optional working directory. defaults to repo root.
    #[prost(string, optional, tag="7")]
    pub workdir: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartBuildResponse {
    #[prost(string, tag="1")]
    pub build_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogsRequest {
    #[prost(string, tag="1")]
    pub build_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub platform: ::prost::alloc::string::String,
}
/// GetLogsResponse is a streaming endpoint that may have multiple messages that belong
/// to the same build_step if there are too many bytes to fit into a single gRPC
/// response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLogsResponse {
    #[prost(string, tag="1")]
    pub build_step: ::prost::alloc::string::String,
    /// includes multiple lines delimited by \n\r
    #[prost(string, tag="2")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobInfo {
    #[prost(string, tag="1")]
    pub build_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub platform: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    #[prost(enumeration="JobStatus", tag="4")]
    pub status: i32,
    #[prost(message, optional, tag="5")]
    pub start_time: ::core::option::Option<super::super::super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub end_time: ::core::option::Option<super::super::super::super::google::protobuf::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// module_id as prefix:name pair
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// don't return more than max_jobs_length jobs
    /// if not present, return all jobs.
    #[prost(int32, optional, tag="2")]
    pub max_jobs_length: ::core::option::Option<i32>,
    /// only return jobs that match this build id
    /// if not present, return all jobs.
    #[prost(string, optional, tag="3")]
    pub build_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// jobs is ordered by (build start time, alphabetical platform).
    #[prost(message, repeated, tag="1")]
    pub jobs: ::prost::alloc::vec::Vec<JobInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepoLink {
    /// viam internal ID which maps to an external oauth app
    #[prost(string, tag="1")]
    pub oauth_app_link_id: ::prost::alloc::string::String,
    /// OrgID of the module
    #[prost(string, tag="2")]
    pub org_id: ::prost::alloc::string::String,
    /// Public namespace of the module. This is for user display in ListRepoLinksResponse.
    /// It is ignored in LinkRepoRequest.
    #[prost(string, optional, tag="3")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// name of the module (just the name, not the dotted org:name form)
    #[prost(string, tag="4")]
    pub module_name: ::prost::alloc::string::String,
    /// git repo in owner/repository form
    #[prost(string, tag="5")]
    pub repo: ::prost::alloc::string::String,
    /// email of the viam user who created this
    #[prost(string, tag="6")]
    pub viam_user: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkRepoRequest {
    #[prost(message, optional, tag="1")]
    pub link: ::core::option::Option<RepoLink>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkRepoResponse {
    #[prost(string, tag="1")]
    pub repo_link_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlinkRepoRequest {
    #[prost(string, tag="1")]
    pub repo_link_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlinkRepoResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepoLinksRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepoLinksResponse {
    #[prost(message, repeated, tag="1")]
    pub links: ::prost::alloc::vec::Vec<RepoLink>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppLinksRequest {
}
/// represents a link between viam users / orgs and an external oauth app
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppLink {
    /// viam internal ID which maps to an external oauth app
    #[prost(string, tag="1")]
    pub oauth_app_link_id: ::prost::alloc::string::String,
    /// email of the viam user that owns this
    #[prost(string, tag="2")]
    pub viam_user: ::prost::alloc::string::String,
    /// username of the external user who created this
    #[prost(string, tag="3")]
    pub external_user: ::prost::alloc::string::String,
    /// list of org public namespace (where available) or org UUIDs attached to the external app
    #[prost(string, repeated, tag="4")]
    pub org_id_or_ns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppLinksResponse {
    #[prost(message, repeated, tag="1")]
    pub links: ::prost::alloc::vec::Vec<AppLink>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAppLinkRequest {
    /// viam internal ID which maps to an external oauth app
    #[prost(string, tag="1")]
    pub oauth_app_link_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAppLinkResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkOrgRequest {
    /// viam internal ID which maps to an external oauth app
    #[prost(string, tag="1")]
    pub oauth_app_link_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkOrgResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlinkOrgRequest {
    /// viam internal ID which maps to an external oauth app
    #[prost(string, tag="1")]
    pub oauth_app_link_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlinkOrgResponse {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobStatus {
    Unspecified = 0,
    /// IN_PROGRESS = pending or executing on cloud infra. Artifact has not been uploaded.
    InProgress = 1,
    Failed = 2,
    Done = 3,
}
impl JobStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobStatus::Unspecified => "JOB_STATUS_UNSPECIFIED",
            JobStatus::InProgress => "JOB_STATUS_IN_PROGRESS",
            JobStatus::Failed => "JOB_STATUS_FAILED",
            JobStatus::Done => "JOB_STATUS_DONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JOB_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "JOB_STATUS_IN_PROGRESS" => Some(Self::InProgress),
            "JOB_STATUS_FAILED" => Some(Self::Failed),
            "JOB_STATUS_DONE" => Some(Self::Done),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
