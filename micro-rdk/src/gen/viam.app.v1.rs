// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Robot {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub location: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub last_access: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub created_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotPart {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// dns_name part name used for fqdn and local fqdn. Anytime the Name is updated this should be sanitized and updated as well.
    #[prost(string, tag="10")]
    pub dns_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub secret: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub robot: ::prost::alloc::string::String,
    /// Store the location_id to allow for unique indexes across parts and locations. This filed MUST be updated each time the robots location
    /// changes.
    #[prost(string, tag="12")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub robot_config: ::core::option::Option<super::super::super::google::protobuf::Struct>,
    #[prost(message, optional, tag="6")]
    pub last_access: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag="7")]
    pub user_supplied_info: ::core::option::Option<super::super::super::google::protobuf::Struct>,
    #[prost(bool, tag="8")]
    pub main_part: bool,
    #[prost(string, tag="9")]
    pub fqdn: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub local_fqdn: ::prost::alloc::string::String,
    #[prost(message, optional, tag="13")]
    pub created_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    /// List of secrets allowed for authentication.
    #[prost(message, repeated, tag="14")]
    pub secrets: ::prost::alloc::vec::Vec<SharedSecret>,
    /// latest timestamp when a robot part was updated
    #[prost(message, optional, tag="15")]
    pub last_updated: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotPartHistoryEntry {
    #[prost(string, tag="1")]
    pub part: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub robot: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub when: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub old: ::core::option::Option<RobotPart>,
    #[prost(message, optional, tag="5")]
    pub edited_by: ::core::option::Option<AuthenticatorInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticatorInfo {
    #[prost(enumeration="AuthenticationType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_deactivated: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Organization {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(string, tag="4")]
    pub public_namespace: ::prost::alloc::string::String,
    /// GCS region of the organization. Locations created under this org will have their GCS region set to this by default and packages
    /// associated with this org will be stored in this region.
    #[prost(string, tag="5")]
    pub default_region: ::prost::alloc::string::String,
    #[prost(string, optional, tag="6")]
    pub cid: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationMember {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub emails: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub date_added: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub last_login: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsResponse {
    #[prost(message, repeated, tag="1")]
    pub organizations: ::prost::alloc::vec::Vec<Organization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationInvite {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, repeated, tag="4")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationResponse {
    #[prost(message, optional, tag="1")]
    pub organization: ::core::option::Option<Organization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationResponse {
    #[prost(message, optional, tag="1")]
    pub organization: ::core::option::Option<Organization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationNamespaceAvailabilityRequest {
    #[prost(string, tag="1")]
    pub public_namespace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationNamespaceAvailabilityResponse {
    #[prost(bool, tag="1")]
    pub available: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub public_namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// The new GCS region to associate the org with.
    #[prost(string, optional, tag="4")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="5")]
    pub cid: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationResponse {
    #[prost(message, optional, tag="1")]
    pub organization: ::core::option::Option<Organization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationMetadataRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationMetadataResponse {
    #[prost(map="string, message", tag="1")]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::google::protobuf::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationMetadataRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="2")]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::google::protobuf::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationMetadataResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationMembersRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationMembersResponse {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub members: ::prost::alloc::vec::Vec<OrganizationMember>,
    #[prost(message, repeated, tag="3")]
    pub invites: ::prost::alloc::vec::Vec<OrganizationInvite>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationInviteRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorization>,
    /// Set to true (the default) to send an email to the recipient of an invite. The user must accept the email to be added to the associated authorizations.
    /// When set to false, the user automatically receives the associated authorization on the next login of the user with the associated email address.
    #[prost(bool, optional, tag="4")]
    pub send_email_invite: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationInviteResponse {
    #[prost(message, optional, tag="1")]
    pub invite: ::core::option::Option<OrganizationInvite>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationInviteAuthorizationsRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub add_authorizations: ::prost::alloc::vec::Vec<Authorization>,
    #[prost(message, repeated, tag="4")]
    pub remove_authorizations: ::prost::alloc::vec::Vec<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationInviteAuthorizationsResponse {
    #[prost(message, optional, tag="1")]
    pub invite: ::core::option::Option<OrganizationInvite>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationInviteRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationInviteResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendOrganizationInviteRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendOrganizationInviteResponse {
    #[prost(message, optional, tag="1")]
    pub invite: ::core::option::Option<OrganizationInvite>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationMemberRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationMemberResponse {
}
// Third Party Org Services

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingAddress {
    #[prost(string, tag="1")]
    pub address_line_1: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub address_line_2: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub city: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub zipcode: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub country: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableBillingServiceRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub billing_address: ::core::option::Option<BillingAddress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableBillingServiceResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBillingServiceRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub billing_address: ::core::option::Option<BillingAddress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBillingServiceResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBillingServiceConfigRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBillingServiceConfigResponse {
    #[prost(message, optional, tag="1")]
    pub billing_address: ::core::option::Option<BillingAddress>,
    #[prost(string, tag="2")]
    pub support_email: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub logo_url: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub billing_dashboard_url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableBillingServiceRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableBillingServiceResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationSetSupportEmailRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationSetSupportEmailResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationGetSupportEmailRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationGetSupportEmailResponse {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
// Location
//

/// Used for rendering an organization's information on the frontend (limited
/// to id, name, or both).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationIdentity {
    /// Organization ID.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Organization name.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationOrganization {
    /// Organization ID the location is shared with.
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    /// Whether the organization is the primary owner or not.
    #[prost(bool, tag="2")]
    pub primary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationAuth {
    /// Deprecated: use secrets field.
    #[deprecated]
    #[prost(string, tag="1")]
    pub secret: ::prost::alloc::string::String,
    /// Location ID containing this LocationAuth.
    #[prost(string, tag="2")]
    pub location_id: ::prost::alloc::string::String,
    /// List of secrets used to authenticate to the Location.
    #[prost(message, repeated, tag="3")]
    pub secrets: ::prost::alloc::vec::Vec<SharedSecret>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageConfig {
    /// GCS region that data is stored in.
    #[prost(string, tag="1")]
    pub region: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Location ID.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Location name.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Location ID of the parent location.
    #[prost(string, tag="4")]
    pub parent_location_id: ::prost::alloc::string::String,
    /// Location authentication secrets.
    #[prost(message, optional, tag="5")]
    pub auth: ::core::option::Option<LocationAuth>,
    /// Organizations that the location is shared with.
    #[prost(message, repeated, tag="6")]
    pub organizations: ::prost::alloc::vec::Vec<LocationOrganization>,
    /// Location creation timestamp.
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    ///
    #[prost(int32, tag="7")]
    pub robot_count: i32,
    /// Config for how data in this location is stored.
    #[prost(message, optional, tag="8")]
    pub config: ::core::option::Option<StorageConfig>,
}
/// SharedSecret is a secret used for LocationAuth and RobotParts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSecret {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The payload of the secret. Used during authentication to the rpc framework.
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
    /// Date/time the secret was first created.
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    /// State of the shared secret. In most cases it should be enabled. We may support
    /// disabling a specific secret while keeping it in the database.
    #[prost(enumeration="shared_secret::State", tag="4")]
    pub state: i32,
}
/// Nested message and enum types in `SharedSecret`.
pub mod shared_secret {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Unspecified = 0,
        /// Secret is enabled and can be used in authentication.
        Enabled = 1,
        /// Secret is disabled and must not be used to authenticate to rpc.
        Disabled = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Enabled => "STATE_ENABLED",
                State::Disabled => "STATE_DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_ENABLED" => Some(Self::Enabled),
                "STATE_DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocationRequest {
    /// Organization ID to create the location under.
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    /// Name of the location.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The new parent location to move the location under.
    #[prost(string, optional, tag="3")]
    pub parent_location_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocationResponse {
    /// Location object is returned.
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationRequest {
    /// Location ID of location to get.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationResponse {
    /// Location object is returned.
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocationRequest {
    /// Location ID of location to update.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    /// The new name to be updated on location.
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The new parent location to move the location under.
    #[prost(string, optional, tag="3")]
    pub parent_location_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The new GCS region to associate the location with.
    #[prost(string, optional, tag="4")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocationResponse {
    /// Location object is returned.
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocationRequest {
    /// Location ID of location to delete.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationMetadataRequest {
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationMetadataResponse {
    #[prost(map="string, message", tag="1")]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::google::protobuf::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocationMetadataRequest {
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="2")]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::google::protobuf::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocationMetadataResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationsWithAccessToLocationRequest {
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationsWithAccessToLocationResponse {
    #[prost(message, repeated, tag="1")]
    pub organization_identities: ::prost::alloc::vec::Vec<OrganizationIdentity>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsRequest {
    /// Organization ID under which to list all locations.
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareLocationRequest {
    /// Location ID to be shared.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    /// Organization ID to share the location with.
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareLocationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnshareLocationRequest {
    /// Location ID to be unshared.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    /// Organization ID to unshare the location with.
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnshareLocationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsResponse {
    #[prost(message, repeated, tag="1")]
    pub locations: ::prost::alloc::vec::Vec<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocationSecretRequest {
    /// Location ID to create the secret in.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocationSecretResponse {
    /// Location's auth after updates.
    #[prost(message, optional, tag="1")]
    pub auth: ::core::option::Option<LocationAuth>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocationSecretRequest {
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub secret_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocationSecretResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationAuthRequest {
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationAuthResponse {
    #[prost(message, optional, tag="1")]
    pub auth: ::core::option::Option<LocationAuth>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoverRentalRobotsRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoverRentalRobot {
    #[prost(string, tag="1")]
    pub robot_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub robot_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub robot_main_part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoverRentalRobotsResponse {
    #[prost(message, repeated, tag="1")]
    pub robots: ::prost::alloc::vec::Vec<RoverRentalRobot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotResponse {
    #[prost(message, optional, tag="1")]
    pub robot: ::core::option::Option<Robot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartsRequest {
    #[prost(string, tag="1")]
    pub robot_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartsResponse {
    #[prost(message, repeated, tag="1")]
    pub parts: ::prost::alloc::vec::Vec<RobotPart>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartResponse {
    #[prost(message, optional, tag="1")]
    pub part: ::core::option::Option<RobotPart>,
    #[prost(string, tag="2")]
    pub config_json: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartLogsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// TODO(<https://viam.atlassian.net/browse/APP-3877>): Remove this field
    #[deprecated]
    #[prost(bool, tag="2")]
    pub errors_only: bool,
    #[prost(string, optional, tag="3")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// logs of all levels are returned when the levels field is empty
    #[prost(string, repeated, tag="5")]
    pub levels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="6")]
    pub start: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag="7")]
    pub end: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(int64, optional, tag="8")]
    pub limit: ::core::option::Option<i64>,
    #[prost(string, optional, tag="9")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartLogsResponse {
    #[prost(message, repeated, tag="1")]
    pub logs: ::prost::alloc::vec::Vec<super::super::common::v1::LogEntry>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailRobotPartLogsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub errors_only: bool,
    #[prost(string, optional, tag="3")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailRobotPartLogsResponse {
    #[prost(message, repeated, tag="1")]
    pub logs: ::prost::alloc::vec::Vec<super::super::common::v1::LogEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartHistoryRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartHistoryResponse {
    #[prost(message, repeated, tag="1")]
    pub history: ::prost::alloc::vec::Vec<RobotPartHistoryEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotPartRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub robot_config: ::core::option::Option<super::super::super::google::protobuf::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotPartResponse {
    #[prost(message, optional, tag="1")]
    pub part: ::core::option::Option<RobotPart>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRobotPartRequest {
    #[prost(string, tag="1")]
    pub robot_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub part_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRobotPartResponse {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotPartRequest {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartMetadataResponse {
    #[prost(map="string, message", tag="1")]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::google::protobuf::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotPartMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="2")]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::google::protobuf::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotPartMetadataResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotApiKeysRequest {
    #[prost(string, tag="1")]
    pub robot_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiKey {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub created_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotApiKeysResponse {
    #[prost(message, repeated, tag="1")]
    pub api_keys: ::prost::alloc::vec::Vec<ApiKeyWithAuthorizations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotPartResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fragment {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub fragment: ::core::option::Option<super::super::super::google::protobuf::Struct>,
    #[prost(string, tag="4")]
    pub organization_owner: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub public: bool,
    #[prost(message, optional, tag="6")]
    pub created_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(string, tag="7")]
    pub organization_name: ::prost::alloc::string::String,
    /// number of robot parts using this fragment
    #[prost(int32, tag="9")]
    pub robot_part_count: i32,
    /// number of organizations using this fragment
    #[prost(int32, tag="10")]
    pub organization_count: i32,
    /// whether the organization(s) using this fragment is the same as the fragment org
    #[prost(bool, tag="11")]
    pub only_used_by_owner: bool,
    /// the visibility of a fragment; public, private or unlisted
    #[prost(enumeration="FragmentVisibility", tag="12")]
    pub visibility: i32,
    /// latest timestamp when fragment was updated
    #[prost(message, optional, tag="13")]
    pub last_updated: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(string, tag="14")]
    pub revision: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FragmentHistoryEntry {
    #[prost(string, tag="1")]
    pub fragment: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub edited_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag="3")]
    pub old: ::core::option::Option<Fragment>,
    #[prost(message, optional, tag="4")]
    pub edited_by: ::core::option::Option<AuthenticatorInfo>,
    #[prost(string, tag="5")]
    pub revision: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub config: ::core::option::Option<super::super::super::google::protobuf::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FragmentRevision {
    #[prost(string, tag="1")]
    pub revision: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub created_at: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FragmentTag {
    #[prost(string, tag="1")]
    pub tag: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub revision: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FragmentError {
    #[prost(enumeration="FragmentErrorType", tag="1")]
    pub error_type: i32,
    #[prost(string, tag="2")]
    pub fragment_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub detail: ::prost::alloc::string::String,
}
/// Cached fragment usage statistics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FragmentUsage {
    #[prost(string, tag="1")]
    pub fragment_id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub organizations: i32,
    #[prost(int32, tag="3")]
    pub machines: i32,
    #[prost(int32, tag="4")]
    pub machines_in_current_org: i32,
    /// revision or tag
    #[prost(string, optional, tag="5")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolvedFragment {
    #[prost(string, tag="1")]
    pub fragment_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub resolved_config: ::core::option::Option<super::super::super::google::protobuf::Struct>,
    #[prost(message, optional, tag="3")]
    pub error: ::core::option::Option<FragmentError>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFragmentsRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub show_public: bool,
    #[prost(enumeration="FragmentVisibility", repeated, tag="3")]
    pub fragment_visibility: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFragmentsResponse {
    #[prost(message, repeated, tag="1")]
    pub fragments: ::prost::alloc::vec::Vec<Fragment>,
    #[prost(message, repeated, tag="2")]
    pub fragment_usages: ::prost::alloc::vec::Vec<FragmentUsage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFragmentRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub current_organization_id: ::prost::alloc::string::String,
    /// revision or tag
    #[prost(string, optional, tag="3")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFragmentResponse {
    #[prost(message, optional, tag="1")]
    pub fragment: ::core::option::Option<Fragment>,
    #[prost(message, optional, tag="2")]
    pub fragment_usage: ::core::option::Option<FragmentUsage>,
    #[prost(message, repeated, tag="3")]
    pub revisions: ::prost::alloc::vec::Vec<FragmentRevision>,
    #[prost(message, repeated, tag="4")]
    pub tags: ::prost::alloc::vec::Vec<FragmentTag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFragmentRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<super::super::super::google::protobuf::Struct>,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(enumeration="FragmentVisibility", optional, tag="4")]
    pub visibility: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFragmentResponse {
    #[prost(message, optional, tag="1")]
    pub fragment: ::core::option::Option<Fragment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFragmentRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub config: ::core::option::Option<super::super::super::google::protobuf::Struct>,
    #[prost(bool, optional, tag="4")]
    pub public: ::core::option::Option<bool>,
    #[prost(enumeration="FragmentVisibility", optional, tag="5")]
    pub visibility: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFragmentResponse {
    #[prost(message, optional, tag="1")]
    pub fragment: ::core::option::Option<Fragment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFragmentRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFragmentResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFragmentHistoryRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag="3")]
    pub page_limit: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFragmentHistoryResponse {
    #[prost(message, repeated, tag="1")]
    pub history: ::prost::alloc::vec::Vec<FragmentHistoryEntry>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFragmentUsageRequest {
    #[prost(string, tag="1")]
    pub fragment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFragmentUsageResponse {
    #[prost(message, repeated, tag="1")]
    pub version_usages: ::prost::alloc::vec::Vec<FragmentUsage>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFragmentTagRequest {
    #[prost(string, tag="1")]
    pub fragment_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tag: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub revision: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetFragmentTagResponse {
    #[prost(message, repeated, tag="1")]
    pub tags: ::prost::alloc::vec::Vec<FragmentTag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFragmentTagRequest {
    #[prost(string, tag="1")]
    pub fragment_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tag: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFragmentTagResponse {
    #[prost(message, repeated, tag="1")]
    pub tags: ::prost::alloc::vec::Vec<FragmentTag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRobotsRequest {
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMachineFragmentsRequest {
    /// the machine_id used to filter fragments defined in a machine's parts.
    /// Also returns any fragments nested within the fragments defined in parts.
    #[prost(string, tag="1")]
    pub machine_id: ::prost::alloc::string::String,
    /// additional fragment_ids to append to the response. useful when needing to view fragments that will be
    /// provisionally added to the machine alongside existing fragments.
    #[prost(string, repeated, tag="2")]
    pub additional_fragment_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMachineFragmentsResponse {
    #[prost(message, repeated, tag="1")]
    pub fragments: ::prost::alloc::vec::Vec<Fragment>,
    #[prost(message, repeated, tag="2")]
    pub resolved_fragments: ::prost::alloc::vec::Vec<ResolvedFragment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRobotsResponse {
    #[prost(message, repeated, tag="1")]
    pub robots: ::prost::alloc::vec::Vec<Robot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRobotRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub location: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRobotResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub location: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotResponse {
    #[prost(message, optional, tag="1")]
    pub robot: ::core::option::Option<Robot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotMetadataResponse {
    #[prost(map="string, message", tag="1")]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::google::protobuf::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotMetadataRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="2")]
    pub data: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::google::protobuf::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotMetadataResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPartAsMainRequest {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPartAsMainResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPartForRestartRequest {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPartForRestartResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRobotPartSecretRequest {
    /// Robot Part ID to create the secret in.
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRobotPartSecretResponse {
    /// Location's auth after updates.
    #[prost(message, optional, tag="1")]
    pub part: ::core::option::Option<RobotPart>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotPartSecretRequest {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub secret_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotPartSecretResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorization {
    #[prost(string, tag="1")]
    pub authorization_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub identity_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub identity_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRoleRequest {
    #[prost(message, optional, tag="1")]
    pub authorization: ::core::option::Option<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRoleResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRoleRequest {
    #[prost(message, optional, tag="1")]
    pub authorization: ::core::option::Option<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRoleResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeRoleRequest {
    #[prost(message, optional, tag="1")]
    pub old_authorization: ::core::option::Option<Authorization>,
    #[prost(message, optional, tag="2")]
    pub new_authorization: ::core::option::Option<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeRoleResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizationsRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    /// optional filter
    #[prost(string, repeated, tag="2")]
    pub resource_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizationsResponse {
    #[prost(message, repeated, tag="1")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPermissionsRequest {
    #[prost(message, repeated, tag="1")]
    pub permissions: ::prost::alloc::vec::Vec<AuthorizedPermissions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedPermissions {
    #[prost(string, tag="1")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPermissionsResponse {
    #[prost(message, repeated, tag="1")]
    pub authorized_permissions: ::prost::alloc::vec::Vec<AuthorizedPermissions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleVersion {
    /// The semver string that represents the major/minor/patch version of the module
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    /// The uploads that are available for this module version
    #[prost(message, repeated, tag="2")]
    pub files: ::prost::alloc::vec::Vec<Uploads>,
    /// The models that this verion of the module provides
    #[prost(message, repeated, tag="3")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// The entrypoint for this version of the module
    #[prost(string, tag="4")]
    pub entrypoint: ::prost::alloc::string::String,
    /// The path to a setup script that is run before a newly downloaded module starts.
    #[prost(string, optional, tag="5")]
    pub first_run: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleMetadata {
    /// A list of models that are available in the module
    #[prost(message, repeated, tag="1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// A list of versions of the module that are available
    /// When this is returned from the backend, the versions are sorted in ascending order by the semver version
    #[prost(message, repeated, tag="2")]
    pub versions: ::prost::alloc::vec::Vec<ModuleVersion>,
    /// The executable to run to start the module program
    #[prost(string, tag="3")]
    pub entrypoint: ::prost::alloc::string::String,
    /// The path to a setup script that is run before a newly downloaded module starts.
    #[prost(string, optional, tag="4")]
    pub first_run: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MlModelMetadata {
    /// A list of package versions for a ML model
    #[prost(string, repeated, tag="1")]
    pub versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="super::mltraining::v1::ModelType", tag="2")]
    pub model_type: i32,
    #[prost(enumeration="super::mltraining::v1::ModelFramework", tag="3")]
    pub model_framework: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MlTrainingVersion {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub created_on: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MlTrainingMetadata {
    /// A list of package versions for ML training source distribution
    #[prost(message, repeated, tag="5")]
    pub versions: ::prost::alloc::vec::Vec<MlTrainingVersion>,
    #[prost(enumeration="super::mltraining::v1::ModelType", tag="2")]
    pub model_type: i32,
    #[prost(enumeration="super::mltraining::v1::ModelFramework", tag="3")]
    pub model_framework: i32,
    #[prost(bool, tag="4")]
    pub draft: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryItem {
    /// The id of the item, containing either:
    /// namespace:item_name when a namespace exists on the org.
    /// org_id:item_name when a namespace does not exist.
    #[prost(string, tag="1")]
    pub item_id: ::prost::alloc::string::String,
    /// The id of the organization that owns the item
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    /// The public namespace of the organization that owns the module
    /// This is empty if no public namespace is set
    #[prost(string, tag="3")]
    pub public_namespace: ::prost::alloc::string::String,
    /// The name of the registry item
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// The type of the item in the registry
    #[prost(enumeration="super::packages::v1::PackageType", tag="5")]
    pub r#type: i32,
    /// The visibility of the registry item
    #[prost(enumeration="Visibility", tag="6")]
    pub visibility: i32,
    /// The url to reference for documentation, code, etc.
    #[prost(string, tag="7")]
    pub url: ::prost::alloc::string::String,
    /// A short description of the item that explains its purpose
    #[prost(string, tag="8")]
    pub description: ::prost::alloc::string::String,
    /// The total number of robots using this item
    #[prost(int64, tag="9")]
    pub total_robot_usage: i64,
    /// The total number of robots using this item outside of the owning org
    #[prost(int64, tag="13")]
    pub total_external_robot_usage: i64,
    /// The total number of organizations using this item
    #[prost(int64, tag="10")]
    pub total_organization_usage: i64,
    /// The total number of organizations using this item outside of the owning org
    #[prost(int64, tag="14")]
    pub total_external_organization_usage: i64,
    /// When the item was created
    #[prost(message, optional, tag="15")]
    pub created_at: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    /// When the item was last updated, either through an update or upload.
    #[prost(message, optional, tag="16")]
    pub updated_at: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    /// Type-specific metadata
    #[prost(oneof="registry_item::Metadata", tags="11, 12, 18")]
    pub metadata: ::core::option::Option<registry_item::Metadata>,
}
/// Nested message and enum types in `RegistryItem`.
pub mod registry_item {
    /// Type-specific metadata
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(message, tag="11")]
        ModuleMetadata(super::ModuleMetadata),
        #[prost(message, tag="12")]
        MlModelMetadata(super::MlModelMetadata),
        #[prost(message, tag="18")]
        MlTrainingMetadata(super::MlTrainingMetadata),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegistryItemRequest {
    #[prost(string, tag="1")]
    pub item_id: ::prost::alloc::string::String,
    #[prost(bool, optional, tag="2")]
    pub include_markdown_documentation: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegistryItemResponse {
    #[prost(message, optional, tag="1")]
    pub item: ::core::option::Option<RegistryItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRegistryItemRequest {
    /// The organization to create the registry item under
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    /// The name of the registry item, which must be unique within your org
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The type of the item in the registry
    #[prost(enumeration="super::packages::v1::PackageType", tag="3")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRegistryItemResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRegistryItemRequest {
    #[prost(string, tag="1")]
    pub item_id: ::prost::alloc::string::String,
    #[prost(enumeration="super::packages::v1::PackageType", tag="2")]
    pub r#type: i32,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration="Visibility", tag="4")]
    pub visibility: i32,
    #[prost(string, optional, tag="5")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(oneof="update_registry_item_request::Metadata", tags="6, 7, 8")]
    pub metadata: ::core::option::Option<update_registry_item_request::Metadata>,
}
/// Nested message and enum types in `UpdateRegistryItemRequest`.
pub mod update_registry_item_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        #[prost(message, tag="6")]
        ModuleUpdateMetadata(super::UpdateModuleMetadata),
        #[prost(message, tag="7")]
        MlModelUpdateMetadata(super::UpdateMlModelMetadata),
        #[prost(message, tag="8")]
        MlTrainingUpdateMetadata(super::UpdateMlTrainingMetadata),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRegistryItemResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRegistryItemsRequest {
    /// The id of the organization to return registry items for.
    #[prost(string, optional, tag="1")]
    pub organization_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="super::packages::v1::PackageType", repeated, tag="2")]
    pub types: ::prost::alloc::vec::Vec<i32>,
    #[prost(enumeration="Visibility", repeated, tag="3")]
    pub visibilities: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, repeated, tag="4")]
    pub platforms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="RegistryItemStatus", repeated, tag="5")]
    pub statuses: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, optional, tag="6")]
    pub search_term: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
    /// One or more public namespaces to return results for.
    #[prost(string, repeated, tag="8")]
    pub public_namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="9")]
    pub include_markdown_documentation: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRegistryItemsResponse {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<RegistryItem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRegistryItemRequest {
    /// The id of the item (formatted as prefix:name where prefix is the owner's orgid or namespace)
    #[prost(string, tag="1")]
    pub item_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRegistryItemResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferRegistryItemRequest {
    #[prost(string, tag="1")]
    pub item_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_public_namespace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferRegistryItemResponse {
}
/// Modules
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModuleRequest {
    /// The organization to create the module under
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    /// The name of the module, which must be unique within your org
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModuleResponse {
    /// The id of the module (formatted as prefix:name where prefix is the module owner's orgid or namespace)
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// The detail page of the module
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModuleRequest {
    /// The id of the module (formatted as prefix:name where prefix is the module owner's orgid or namespace)
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// The visibility that should be set for the module
    #[prost(enumeration="Visibility", tag="2")]
    pub visibility: i32,
    /// The url to reference for documentation, code, etc.
    #[prost(string, tag="3")]
    pub url: ::prost::alloc::string::String,
    /// A short description of the module that explains its purpose
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// A list of models that are available in the module
    #[prost(message, repeated, tag="5")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// The executable to run to start the module program
    #[prost(string, tag="6")]
    pub entrypoint: ::prost::alloc::string::String,
    /// The path to a setup script that is run before a newly downloaded module starts.
    #[prost(string, optional, tag="7")]
    pub first_run: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModuleResponse {
    /// The detail page of the module
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModuleMetadata {
    /// A list of models that are available in the module
    #[prost(message, repeated, tag="1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// The executable to run to start the module program
    #[prost(string, tag="2")]
    pub entrypoint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMlModelMetadata {
    #[prost(enumeration="super::mltraining::v1::ModelType", tag="1")]
    pub model_type: i32,
    #[prost(enumeration="super::mltraining::v1::ModelFramework", tag="2")]
    pub model_framework: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMlTrainingMetadata {
    #[prost(enumeration="super::mltraining::v1::ModelType", tag="1")]
    pub model_type: i32,
    #[prost(enumeration="super::mltraining::v1::ModelFramework", tag="2")]
    pub model_framework: i32,
    #[prost(bool, tag="3")]
    pub draft: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// The colon-delimited-triplet of the api implemented by the model
    #[prost(string, tag="1")]
    pub api: ::prost::alloc::string::String,
    /// The colon-delimited-triplet of the model
    #[prost(string, tag="2")]
    pub model: ::prost::alloc::string::String,
    /// The markdown content describing the usage of the model
    #[prost(string, optional, tag="3")]
    pub markdown_documentation: ::core::option::Option<::prost::alloc::string::String>,
    /// A short description of the model that explains its purpose
    #[prost(string, optional, tag="4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleFileInfo {
    /// The id of the module (formatted as prefix:name where prefix is the module owner's orgid or namespace)
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// The semver string that represents the new major/minor/patch version of the module
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// The platform that the file is built to run on
    #[prost(string, tag="3")]
    pub platform: ::prost::alloc::string::String,
    /// Platform tag constraints. When a robot requests its config, it uploads a platform and a list of
    /// platform tags. The platform is checked against `platform` above, and the tags are checked against
    /// this list.
    #[prost(string, repeated, tag="5")]
    pub platform_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadModuleFileRequest {
    #[prost(oneof="upload_module_file_request::ModuleFile", tags="1, 2")]
    pub module_file: ::core::option::Option<upload_module_file_request::ModuleFile>,
}
/// Nested message and enum types in `UploadModuleFileRequest`.
pub mod upload_module_file_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModuleFile {
        /// The information about the module file being uploaded
        #[prost(message, tag="1")]
        ModuleFileInfo(super::ModuleFileInfo),
        /// The file contents to be uploaded
        #[prost(bytes, tag="2")]
        File(::prost::alloc::vec::Vec<u8>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadModuleFileResponse {
    /// The detail page of the module
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModuleRequest {
    /// The id of the module (formatted as prefix:name where prefix is the module owner's orgid or namespace)
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    #[prost(bool, optional, tag="2")]
    pub include_markdown_documentation: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModuleResponse {
    /// The module object
    #[prost(message, optional, tag="1")]
    pub module: ::core::option::Option<Module>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// The id of the module (formatted as prefix:name where prefix is the module owner's orgid or namespace)
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// The name of the module
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The visibility of the module
    #[prost(enumeration="Visibility", tag="3")]
    pub visibility: i32,
    /// The versions of the module that are available
    /// When this is returned from the backend, the versions are sorted in ascending order by the semver version
    #[prost(message, repeated, tag="4")]
    pub versions: ::prost::alloc::vec::Vec<VersionHistory>,
    /// The url to reference for documentation, code, etc.
    #[prost(string, tag="5")]
    pub url: ::prost::alloc::string::String,
    /// A short description of the module that explains its purpose
    #[prost(string, tag="6")]
    pub description: ::prost::alloc::string::String,
    /// A list of models that are available in the module
    #[prost(message, repeated, tag="7")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// The total number of robots using this module
    #[prost(int64, tag="8")]
    pub total_robot_usage: i64,
    /// The total number of organizations using this module
    #[prost(int64, tag="9")]
    pub total_organization_usage: i64,
    /// The id of the organization that owns the module
    #[prost(string, tag="10")]
    pub organization_id: ::prost::alloc::string::String,
    /// The executable to run to start the module program
    #[prost(string, tag="11")]
    pub entrypoint: ::prost::alloc::string::String,
    /// The public namespace of the organization that owns the module
    /// This is empty if no public namespace is set
    #[prost(string, tag="12")]
    pub public_namespace: ::prost::alloc::string::String,
    /// The path to a setup script that is run before a newly downloaded module starts.
    #[prost(string, optional, tag="13")]
    pub first_run: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionHistory {
    /// The semver string that represents the major/minor/patch version of the module
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    /// The uploads that are available for this module version
    #[prost(message, repeated, tag="2")]
    pub files: ::prost::alloc::vec::Vec<Uploads>,
    /// The models that this verion of the module provides
    #[prost(message, repeated, tag="3")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// The entrypoint for this version of the module
    #[prost(string, tag="4")]
    pub entrypoint: ::prost::alloc::string::String,
    /// The path to a setup script that is run before a newly downloaded module starts.
    #[prost(string, optional, tag="5")]
    pub first_run: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uploads {
    /// The OS and architecture the module is built to run on
    #[prost(string, tag="1")]
    pub platform: ::prost::alloc::string::String,
    /// The time when the file was uploaded
    #[prost(message, optional, tag="2")]
    pub uploaded_at: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModulesRequest {
    /// The id of the organization to return private modules for.
    #[prost(string, optional, tag="1")]
    pub organization_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="2")]
    pub include_markdown_documentation: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModulesResponse {
    /// A listed of modules. When authenticated, this API will return modules that are private for this org. Public modules are always returned.
    #[prost(message, repeated, tag="1")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserIdByEmailRequest {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserIdByEmailResponse {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsByUserRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgDetails {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub org_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub org_cid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub public_namespace: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsByUserResponse {
    #[prost(message, repeated, tag="1")]
    pub orgs: ::prost::alloc::vec::Vec<OrgDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOrganizationsRequest {
    #[prost(string, optional, tag="1")]
    pub org_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub org_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub cid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub public_namespace: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOrganizationsResponse {
    #[prost(message, repeated, tag="1")]
    pub organizations: ::prost::alloc::vec::Vec<OrgDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyRequest {
    #[prost(message, repeated, tag="1")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorization>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyResponse {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameKeyRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameKeyResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationDetails {
    #[prost(string, tag="1")]
    pub authorization_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiKeyWithAuthorizations {
    #[prost(message, optional, tag="1")]
    pub api_key: ::core::option::Option<ApiKey>,
    #[prost(message, repeated, tag="2")]
    pub authorizations: ::prost::alloc::vec::Vec<AuthorizationDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysResponse {
    #[prost(message, repeated, tag="1")]
    pub api_keys: ::prost::alloc::vec::Vec<ApiKeyWithAuthorizations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RotateKeyRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RotateKeyResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyFromExistingKeyAuthorizationsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyFromExistingKeyAuthorizationsResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationSetLogoRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub logo: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationSetLogoResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationGetLogoRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationGetLogoResponse {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableAuthServiceRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableAuthServiceResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableAuthServiceRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableAuthServiceResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOAuthAppRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub oauth_config: ::core::option::Option<OAuthConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOAuthAppResponse {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_secret: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadOAuthAppRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadOAuthAppResponse {
    #[prost(string, tag="1")]
    pub client_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub oauth_config: ::core::option::Option<OAuthConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOAuthAppRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub oauth_config: ::core::option::Option<OAuthConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOAuthAppResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOAuthAppRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub client_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOAuthAppResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOAuthAppsRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOAuthAppsResponse {
    #[prost(string, repeated, tag="1")]
    pub client_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuthConfig {
    #[prost(enumeration="ClientAuthentication", tag="1")]
    pub client_authentication: i32,
    #[prost(enumeration="Pkce", tag="2")]
    pub pkce: i32,
    #[prost(enumeration="UrlValidation", tag="3")]
    pub url_validation: i32,
    #[prost(string, repeated, tag="4")]
    pub origin_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub logout_uri: ::prost::alloc::string::String,
    #[prost(enumeration="EnabledGrant", repeated, tag="7")]
    pub enabled_grants: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthenticationType {
    Unspecified = 0,
    WebOauth = 1,
    ApiKey = 2,
    RobotPartSecret = 3,
    LocationSecret = 4,
}
impl AuthenticationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthenticationType::Unspecified => "AUTHENTICATION_TYPE_UNSPECIFIED",
            AuthenticationType::WebOauth => "AUTHENTICATION_TYPE_WEB_OAUTH",
            AuthenticationType::ApiKey => "AUTHENTICATION_TYPE_API_KEY",
            AuthenticationType::RobotPartSecret => "AUTHENTICATION_TYPE_ROBOT_PART_SECRET",
            AuthenticationType::LocationSecret => "AUTHENTICATION_TYPE_LOCATION_SECRET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTHENTICATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTHENTICATION_TYPE_WEB_OAUTH" => Some(Self::WebOauth),
            "AUTHENTICATION_TYPE_API_KEY" => Some(Self::ApiKey),
            "AUTHENTICATION_TYPE_ROBOT_PART_SECRET" => Some(Self::RobotPartSecret),
            "AUTHENTICATION_TYPE_LOCATION_SECRET" => Some(Self::LocationSecret),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FragmentVisibility {
    Unspecified = 0,
    Private = 1,
    Public = 2,
    PublicUnlisted = 3,
}
impl FragmentVisibility {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FragmentVisibility::Unspecified => "FRAGMENT_VISIBILITY_UNSPECIFIED",
            FragmentVisibility::Private => "FRAGMENT_VISIBILITY_PRIVATE",
            FragmentVisibility::Public => "FRAGMENT_VISIBILITY_PUBLIC",
            FragmentVisibility::PublicUnlisted => "FRAGMENT_VISIBILITY_PUBLIC_UNLISTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FRAGMENT_VISIBILITY_UNSPECIFIED" => Some(Self::Unspecified),
            "FRAGMENT_VISIBILITY_PRIVATE" => Some(Self::Private),
            "FRAGMENT_VISIBILITY_PUBLIC" => Some(Self::Public),
            "FRAGMENT_VISIBILITY_PUBLIC_UNLISTED" => Some(Self::PublicUnlisted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FragmentErrorType {
    Unspecified = 0,
    NoAccess = 1,
    NestingLimitExceeded = 2,
    ChildIdInvalid = 3,
    CycleDetected = 4,
}
impl FragmentErrorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FragmentErrorType::Unspecified => "FRAGMENT_ERROR_TYPE_UNSPECIFIED",
            FragmentErrorType::NoAccess => "FRAGMENT_ERROR_TYPE_NO_ACCESS",
            FragmentErrorType::NestingLimitExceeded => "FRAGMENT_ERROR_TYPE_NESTING_LIMIT_EXCEEDED",
            FragmentErrorType::ChildIdInvalid => "FRAGMENT_ERROR_TYPE_CHILD_ID_INVALID",
            FragmentErrorType::CycleDetected => "FRAGMENT_ERROR_TYPE_CYCLE_DETECTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FRAGMENT_ERROR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "FRAGMENT_ERROR_TYPE_NO_ACCESS" => Some(Self::NoAccess),
            "FRAGMENT_ERROR_TYPE_NESTING_LIMIT_EXCEEDED" => Some(Self::NestingLimitExceeded),
            "FRAGMENT_ERROR_TYPE_CHILD_ID_INVALID" => Some(Self::ChildIdInvalid),
            "FRAGMENT_ERROR_TYPE_CYCLE_DETECTED" => Some(Self::CycleDetected),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RegistryItemStatus {
    Unspecified = 0,
    Published = 1,
    InDevelopment = 2,
}
impl RegistryItemStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RegistryItemStatus::Unspecified => "REGISTRY_ITEM_STATUS_UNSPECIFIED",
            RegistryItemStatus::Published => "REGISTRY_ITEM_STATUS_PUBLISHED",
            RegistryItemStatus::InDevelopment => "REGISTRY_ITEM_STATUS_IN_DEVELOPMENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REGISTRY_ITEM_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "REGISTRY_ITEM_STATUS_PUBLISHED" => Some(Self::Published),
            "REGISTRY_ITEM_STATUS_IN_DEVELOPMENT" => Some(Self::InDevelopment),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Visibility {
    Unspecified = 0,
    /// Private registry items are visible only within the owning org
    Private = 1,
    /// Public registry items are visible to everyone
    Public = 2,
    /// Public Unlisted registry items are usable in everyone's robot but are hidden from the registry page as if they are private
    PublicUnlisted = 3,
}
impl Visibility {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Visibility::Unspecified => "VISIBILITY_UNSPECIFIED",
            Visibility::Private => "VISIBILITY_PRIVATE",
            Visibility::Public => "VISIBILITY_PUBLIC",
            Visibility::PublicUnlisted => "VISIBILITY_PUBLIC_UNLISTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VISIBILITY_UNSPECIFIED" => Some(Self::Unspecified),
            "VISIBILITY_PRIVATE" => Some(Self::Private),
            "VISIBILITY_PUBLIC" => Some(Self::Public),
            "VISIBILITY_PUBLIC_UNLISTED" => Some(Self::PublicUnlisted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientAuthentication {
    Unspecified = 0,
    Required = 1,
    NotRequired = 2,
    NotRequiredWhenUsingPkce = 3,
}
impl ClientAuthentication {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClientAuthentication::Unspecified => "CLIENT_AUTHENTICATION_UNSPECIFIED",
            ClientAuthentication::Required => "CLIENT_AUTHENTICATION_REQUIRED",
            ClientAuthentication::NotRequired => "CLIENT_AUTHENTICATION_NOT_REQUIRED",
            ClientAuthentication::NotRequiredWhenUsingPkce => "CLIENT_AUTHENTICATION_NOT_REQUIRED_WHEN_USING_PKCE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CLIENT_AUTHENTICATION_UNSPECIFIED" => Some(Self::Unspecified),
            "CLIENT_AUTHENTICATION_REQUIRED" => Some(Self::Required),
            "CLIENT_AUTHENTICATION_NOT_REQUIRED" => Some(Self::NotRequired),
            "CLIENT_AUTHENTICATION_NOT_REQUIRED_WHEN_USING_PKCE" => Some(Self::NotRequiredWhenUsingPkce),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Pkce {
    Unspecified = 0,
    Required = 1,
    NotRequired = 2,
    NotRequiredWhenUsingClientAuthentication = 3,
}
impl Pkce {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Pkce::Unspecified => "PKCE_UNSPECIFIED",
            Pkce::Required => "PKCE_REQUIRED",
            Pkce::NotRequired => "PKCE_NOT_REQUIRED",
            Pkce::NotRequiredWhenUsingClientAuthentication => "PKCE_NOT_REQUIRED_WHEN_USING_CLIENT_AUTHENTICATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PKCE_UNSPECIFIED" => Some(Self::Unspecified),
            "PKCE_REQUIRED" => Some(Self::Required),
            "PKCE_NOT_REQUIRED" => Some(Self::NotRequired),
            "PKCE_NOT_REQUIRED_WHEN_USING_CLIENT_AUTHENTICATION" => Some(Self::NotRequiredWhenUsingClientAuthentication),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UrlValidation {
    Unspecified = 0,
    ExactMatch = 1,
    AllowWildcards = 2,
}
impl UrlValidation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UrlValidation::Unspecified => "URL_VALIDATION_UNSPECIFIED",
            UrlValidation::ExactMatch => "URL_VALIDATION_EXACT_MATCH",
            UrlValidation::AllowWildcards => "URL_VALIDATION_ALLOW_WILDCARDS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "URL_VALIDATION_UNSPECIFIED" => Some(Self::Unspecified),
            "URL_VALIDATION_EXACT_MATCH" => Some(Self::ExactMatch),
            "URL_VALIDATION_ALLOW_WILDCARDS" => Some(Self::AllowWildcards),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnabledGrant {
    Unspecified = 0,
    AuthorizationCode = 1,
    Implicit = 2,
    Password = 3,
    RefreshToken = 4,
    DeviceCode = 5,
}
impl EnabledGrant {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EnabledGrant::Unspecified => "ENABLED_GRANT_UNSPECIFIED",
            EnabledGrant::AuthorizationCode => "ENABLED_GRANT_AUTHORIZATION_CODE",
            EnabledGrant::Implicit => "ENABLED_GRANT_IMPLICIT",
            EnabledGrant::Password => "ENABLED_GRANT_PASSWORD",
            EnabledGrant::RefreshToken => "ENABLED_GRANT_REFRESH_TOKEN",
            EnabledGrant::DeviceCode => "ENABLED_GRANT_DEVICE_CODE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENABLED_GRANT_UNSPECIFIED" => Some(Self::Unspecified),
            "ENABLED_GRANT_AUTHORIZATION_CODE" => Some(Self::AuthorizationCode),
            "ENABLED_GRANT_IMPLICIT" => Some(Self::Implicit),
            "ENABLED_GRANT_PASSWORD" => Some(Self::Password),
            "ENABLED_GRANT_REFRESH_TOKEN" => Some(Self::RefreshToken),
            "ENABLED_GRANT_DEVICE_CODE" => Some(Self::DeviceCode),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceSummary {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub invoice_date: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(double, tag="3")]
    pub invoice_amount: f64,
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub due_date: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub paid_date: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentMethodCard {
    #[prost(string, tag="1")]
    pub brand: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub last_four_digits: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentMonthUsageRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageCost {
    #[prost(enumeration="UsageCostType", tag="1")]
    pub resource_type: i32,
    #[prost(double, tag="2")]
    pub cost: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUsageCostsBySource {
    #[prost(enumeration="SourceType", tag="1")]
    pub source_type: i32,
    #[prost(message, optional, tag="2")]
    pub resource_usage_costs: ::core::option::Option<ResourceUsageCosts>,
    #[prost(string, tag="3")]
    pub tier_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUsageCosts {
    #[prost(message, repeated, tag="1")]
    pub usage_costs: ::prost::alloc::vec::Vec<UsageCost>,
    #[prost(double, tag="2")]
    pub discount: f64,
    #[prost(double, tag="3")]
    pub total_with_discount: f64,
    #[prost(double, tag="4")]
    pub total_without_discount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentMonthUsageResponse {
    #[prost(message, optional, tag="1")]
    pub start_date: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end_date: ::core::option::Option<super::super::super::google::protobuf::Timestamp>,
    #[prost(message, repeated, tag="14")]
    pub resource_usage_costs_by_source: ::prost::alloc::vec::Vec<ResourceUsageCostsBySource>,
    #[prost(double, tag="15")]
    pub subtotal: f64,
    /// all fields below are deprecated
    #[deprecated]
    #[prost(double, tag="3")]
    pub cloud_storage_usage_cost: f64,
    #[deprecated]
    #[prost(double, tag="4")]
    pub data_upload_usage_cost: f64,
    #[deprecated]
    #[prost(double, tag="5")]
    pub data_egres_usage_cost: f64,
    #[deprecated]
    #[prost(double, tag="6")]
    pub remote_control_usage_cost: f64,
    #[deprecated]
    #[prost(double, tag="7")]
    pub standard_compute_usage_cost: f64,
    #[deprecated]
    #[prost(double, tag="8")]
    pub discount_amount: f64,
    #[deprecated]
    #[prost(double, tag="9")]
    pub total_usage_with_discount: f64,
    #[deprecated]
    #[prost(double, tag="10")]
    pub total_usage_without_discount: f64,
    #[deprecated]
    #[prost(double, tag="11")]
    pub per_machine_usage_cost: f64,
    #[deprecated]
    #[prost(double, tag="12")]
    pub binary_data_cloud_storage_usage_cost: f64,
    #[deprecated]
    #[prost(double, tag="13")]
    pub other_cloud_storage_usage_cost: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgBillingInformationRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgBillingInformationResponse {
    #[prost(enumeration="PaymentMethodType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub billing_email: ::prost::alloc::string::String,
    /// defined if type is PAYMENT_METHOD_TYPE_CARD
    #[prost(message, optional, tag="3")]
    pub method: ::core::option::Option<PaymentMethodCard>,
    /// Only return billing_tier for billing dashboard admin users
    #[prost(string, optional, tag="4")]
    pub billing_tier: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoicesSummaryRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoicesSummaryResponse {
    /// all unpaid balances at the end of the last billing cycle
    #[prost(double, tag="1")]
    pub outstanding_balance: f64,
    /// all previous invoices
    #[prost(message, repeated, tag="2")]
    pub invoices: ::prost::alloc::vec::Vec<InvoiceSummary>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoicePdfRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoicePdfResponse {
    #[prost(bytes="vec", tag="1")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentRequiredEmailRequest {
    #[prost(string, tag="1")]
    pub customer_org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub billing_owner_org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendPaymentRequiredEmailResponse {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentMethodType {
    Unspecified = 0,
    Card = 1,
}
impl PaymentMethodType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentMethodType::Unspecified => "PAYMENT_METHOD_TYPE_UNSPECIFIED",
            PaymentMethodType::Card => "PAYMENT_METHOD_TYPE_CARD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PAYMENT_METHOD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PAYMENT_METHOD_TYPE_CARD" => Some(Self::Card),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UsageCostType {
    Unspecified = 0,
    DataUpload = 1,
    DataEgress = 2,
    RemoteControl = 3,
    StandardCompute = 4,
    CloudStorage = 5,
    BinaryDataCloudStorage = 6,
    OtherCloudStorage = 7,
    PerMachine = 8,
    TriggerNotification = 9,
    TabularDataCloudStorage = 10,
    ConfigHistoryCloudStorage = 11,
    LogsCloudStorage = 12,
    TrainingLogsCloudStorage = 13,
    PackagesCloudStorage = 14,
    BinaryDataUpload = 15,
    TabularDataUpload = 16,
    LogsUpload = 17,
    BinaryDataEgress = 18,
    TabularDataEgress = 19,
    LogsEgress = 20,
    TrainingLogsEgress = 21,
    TabularDataDatabaseCloudStorage = 22,
    TabularDataDatabaseCompute = 23,
}
impl UsageCostType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UsageCostType::Unspecified => "USAGE_COST_TYPE_UNSPECIFIED",
            UsageCostType::DataUpload => "USAGE_COST_TYPE_DATA_UPLOAD",
            UsageCostType::DataEgress => "USAGE_COST_TYPE_DATA_EGRESS",
            UsageCostType::RemoteControl => "USAGE_COST_TYPE_REMOTE_CONTROL",
            UsageCostType::StandardCompute => "USAGE_COST_TYPE_STANDARD_COMPUTE",
            UsageCostType::CloudStorage => "USAGE_COST_TYPE_CLOUD_STORAGE",
            UsageCostType::BinaryDataCloudStorage => "USAGE_COST_TYPE_BINARY_DATA_CLOUD_STORAGE",
            UsageCostType::OtherCloudStorage => "USAGE_COST_TYPE_OTHER_CLOUD_STORAGE",
            UsageCostType::PerMachine => "USAGE_COST_TYPE_PER_MACHINE",
            UsageCostType::TriggerNotification => "USAGE_COST_TYPE_TRIGGER_NOTIFICATION",
            UsageCostType::TabularDataCloudStorage => "USAGE_COST_TYPE_TABULAR_DATA_CLOUD_STORAGE",
            UsageCostType::ConfigHistoryCloudStorage => "USAGE_COST_TYPE_CONFIG_HISTORY_CLOUD_STORAGE",
            UsageCostType::LogsCloudStorage => "USAGE_COST_TYPE_LOGS_CLOUD_STORAGE",
            UsageCostType::TrainingLogsCloudStorage => "USAGE_COST_TYPE_TRAINING_LOGS_CLOUD_STORAGE",
            UsageCostType::PackagesCloudStorage => "USAGE_COST_TYPE_PACKAGES_CLOUD_STORAGE",
            UsageCostType::BinaryDataUpload => "USAGE_COST_TYPE_BINARY_DATA_UPLOAD",
            UsageCostType::TabularDataUpload => "USAGE_COST_TYPE_TABULAR_DATA_UPLOAD",
            UsageCostType::LogsUpload => "USAGE_COST_TYPE_LOGS_UPLOAD",
            UsageCostType::BinaryDataEgress => "USAGE_COST_TYPE_BINARY_DATA_EGRESS",
            UsageCostType::TabularDataEgress => "USAGE_COST_TYPE_TABULAR_DATA_EGRESS",
            UsageCostType::LogsEgress => "USAGE_COST_TYPE_LOGS_EGRESS",
            UsageCostType::TrainingLogsEgress => "USAGE_COST_TYPE_TRAINING_LOGS_EGRESS",
            UsageCostType::TabularDataDatabaseCloudStorage => "USAGE_COST_TYPE_TABULAR_DATA_DATABASE_CLOUD_STORAGE",
            UsageCostType::TabularDataDatabaseCompute => "USAGE_COST_TYPE_TABULAR_DATA_DATABASE_COMPUTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USAGE_COST_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "USAGE_COST_TYPE_DATA_UPLOAD" => Some(Self::DataUpload),
            "USAGE_COST_TYPE_DATA_EGRESS" => Some(Self::DataEgress),
            "USAGE_COST_TYPE_REMOTE_CONTROL" => Some(Self::RemoteControl),
            "USAGE_COST_TYPE_STANDARD_COMPUTE" => Some(Self::StandardCompute),
            "USAGE_COST_TYPE_CLOUD_STORAGE" => Some(Self::CloudStorage),
            "USAGE_COST_TYPE_BINARY_DATA_CLOUD_STORAGE" => Some(Self::BinaryDataCloudStorage),
            "USAGE_COST_TYPE_OTHER_CLOUD_STORAGE" => Some(Self::OtherCloudStorage),
            "USAGE_COST_TYPE_PER_MACHINE" => Some(Self::PerMachine),
            "USAGE_COST_TYPE_TRIGGER_NOTIFICATION" => Some(Self::TriggerNotification),
            "USAGE_COST_TYPE_TABULAR_DATA_CLOUD_STORAGE" => Some(Self::TabularDataCloudStorage),
            "USAGE_COST_TYPE_CONFIG_HISTORY_CLOUD_STORAGE" => Some(Self::ConfigHistoryCloudStorage),
            "USAGE_COST_TYPE_LOGS_CLOUD_STORAGE" => Some(Self::LogsCloudStorage),
            "USAGE_COST_TYPE_TRAINING_LOGS_CLOUD_STORAGE" => Some(Self::TrainingLogsCloudStorage),
            "USAGE_COST_TYPE_PACKAGES_CLOUD_STORAGE" => Some(Self::PackagesCloudStorage),
            "USAGE_COST_TYPE_BINARY_DATA_UPLOAD" => Some(Self::BinaryDataUpload),
            "USAGE_COST_TYPE_TABULAR_DATA_UPLOAD" => Some(Self::TabularDataUpload),
            "USAGE_COST_TYPE_LOGS_UPLOAD" => Some(Self::LogsUpload),
            "USAGE_COST_TYPE_BINARY_DATA_EGRESS" => Some(Self::BinaryDataEgress),
            "USAGE_COST_TYPE_TABULAR_DATA_EGRESS" => Some(Self::TabularDataEgress),
            "USAGE_COST_TYPE_LOGS_EGRESS" => Some(Self::LogsEgress),
            "USAGE_COST_TYPE_TRAINING_LOGS_EGRESS" => Some(Self::TrainingLogsEgress),
            "USAGE_COST_TYPE_TABULAR_DATA_DATABASE_CLOUD_STORAGE" => Some(Self::TabularDataDatabaseCloudStorage),
            "USAGE_COST_TYPE_TABULAR_DATA_DATABASE_COMPUTE" => Some(Self::TabularDataDatabaseCompute),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SourceType {
    Unspecified = 0,
    Org = 1,
    Fragment = 2,
}
impl SourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SourceType::Unspecified => "SOURCE_TYPE_UNSPECIFIED",
            SourceType::Org => "SOURCE_TYPE_ORG",
            SourceType::Fragment => "SOURCE_TYPE_FRAGMENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SOURCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SOURCE_TYPE_ORG" => Some(Self::Org),
            "SOURCE_TYPE_FRAGMENT" => Some(Self::Fragment),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsLegalAcceptedRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsLegalAcceptedResponse {
    /// If false, the user should not be able to use the application.
    #[prost(bool, tag="1")]
    pub accepted_legal: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptLegalRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptLegalResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAuthApplicationRequest {
    #[prost(string, tag="1")]
    pub application_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub origin_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub logout_uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAuthApplicationResponse {
    #[prost(string, tag="1")]
    pub application_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub application_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_secret: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthApplicationRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub application_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub application_name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub origin_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub logout_uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAuthApplicationResponse {
    #[prost(string, tag="1")]
    pub application_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub application_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthApplicationRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub application_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthApplicationResponse {
    #[prost(string, tag="1")]
    pub application_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub application_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub client_secret: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub origin_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="5")]
    pub redirect_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub logout_uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotConfig {
    #[prost(message, optional, tag="1")]
    pub cloud: ::core::option::Option<CloudConfig>,
    #[prost(message, repeated, tag="2")]
    pub remotes: ::prost::alloc::vec::Vec<RemoteConfig>,
    #[prost(message, repeated, tag="3")]
    pub components: ::prost::alloc::vec::Vec<ComponentConfig>,
    #[prost(message, repeated, tag="4")]
    pub processes: ::prost::alloc::vec::Vec<ProcessConfig>,
    #[prost(message, repeated, tag="5")]
    pub services: ::prost::alloc::vec::Vec<ServiceConfig>,
    #[prost(message, optional, tag="6")]
    pub network: ::core::option::Option<NetworkConfig>,
    #[prost(message, optional, tag="7")]
    pub auth: ::core::option::Option<AuthConfig>,
    /// Turns on debug mode for robot, adding an echo server and more logging and tracing. Only works after restart
    #[prost(bool, optional, tag="8")]
    pub debug: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="9")]
    pub modules: ::prost::alloc::vec::Vec<ModuleConfig>,
    #[prost(bool, optional, tag="10")]
    pub disable_partial_start: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="11")]
    pub packages: ::prost::alloc::vec::Vec<PackageConfig>,
    #[prost(message, repeated, tag="12")]
    pub overwrite_fragment_status: ::prost::alloc::vec::Vec<AppValidationStatus>,
    /// Turns on pprof http server on localhost. By default false.
    #[prost(bool, tag="13")]
    pub enable_web_profile: bool,
    #[prost(message, repeated, tag="14")]
    pub log: ::prost::alloc::vec::Vec<LogPatternConfig>,
    /// Attributes a particular revision to the config.
    #[prost(string, tag="15")]
    pub revision: ::prost::alloc::string::String,
    #[prost(message, optional, tag="16")]
    pub maintenance: ::core::option::Option<MaintenanceConfig>,
    /// Disables the robot's log deduplication (identical, noisy logs will still
    /// be output individually instead of aggregated.)
    #[prost(bool, tag="17")]
    pub disable_log_deduplication: bool,
}
/// LogPatternConfig allows you to specify a 2-tuple consisting
/// of a logger name and its corresponding log level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogPatternConfig {
    #[prost(string, tag="1")]
    pub pattern: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub level: ::prost::alloc::string::String,
}
/// Valid location secret that can be used for authentication to the robot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationSecret {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// secret payload
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppValidationStatus {
    #[prost(string, tag="1")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudConfig {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub fqdn: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub local_fqdn: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub managed_by: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub signaling_address: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub signaling_insecure: bool,
    /// Deprecated use location_secrets
    #[deprecated]
    #[prost(string, tag="7")]
    pub location_secret: ::prost::alloc::string::String,
    /// Robot part secret
    #[prost(string, tag="8")]
    pub secret: ::prost::alloc::string::String,
    /// All valid location secrets.
    #[prost(message, repeated, tag="9")]
    pub location_secrets: ::prost::alloc::vec::Vec<LocationSecret>,
    #[prost(string, tag="10")]
    pub primary_org_id: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub machine_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// deprecated; use api
    #[prost(string, tag="2")]
    pub namespace: ::prost::alloc::string::String,
    /// deprecated; use api
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub model: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub frame: ::core::option::Option<Frame>,
    #[prost(string, repeated, tag="6")]
    pub depends_on: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="7")]
    pub service_configs: ::prost::alloc::vec::Vec<ResourceLevelServiceConfig>,
    #[prost(message, optional, tag="8")]
    pub attributes: ::core::option::Option<super::super::super::google::protobuf::Struct>,
    #[prost(string, tag="9")]
    pub api: ::prost::alloc::string::String,
    #[prost(message, optional, tag="10")]
    pub log_configuration: ::core::option::Option<LogConfiguration>,
}
/// A ResourceLevelServiceConfig describes component or remote configuration for a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLevelServiceConfig {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// TODO(adam): Should this be move to a structured type as defined in the typescript frontend.
    #[prost(message, optional, tag="2")]
    pub attributes: ::core::option::Option<super::super::super::google::protobuf::Struct>,
}
/// A ProcessConfig describes how to manage a system process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessConfig {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub cwd: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub one_shot: bool,
    #[prost(bool, tag="6")]
    pub log: bool,
    #[prost(int32, tag="7")]
    pub stop_signal: i32,
    #[prost(message, optional, tag="8")]
    pub stop_timeout: ::core::option::Option<super::super::super::google::protobuf::Duration>,
    /// additional environment variables passed to the process
    #[prost(map="string, string", tag="9")]
    pub env: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(string, tag="10")]
    pub username: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// deprecated; use api
    #[prost(string, tag="2")]
    pub namespace: ::prost::alloc::string::String,
    /// deprecated; use api
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub attributes: ::core::option::Option<super::super::super::google::protobuf::Struct>,
    #[prost(string, repeated, tag="5")]
    pub depends_on: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub model: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub api: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="10")]
    pub service_configs: ::prost::alloc::vec::Vec<ResourceLevelServiceConfig>,
    #[prost(message, optional, tag="11")]
    pub log_configuration: ::core::option::Option<LogConfiguration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    #[prost(string, tag="1")]
    pub fqdn: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bind_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tls_cert_file: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub tls_key_file: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub sessions: ::core::option::Option<SessionsConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionsConfig {
    #[prost(message, optional, tag="1")]
    pub heartbeat_window: ::core::option::Option<super::super::super::google::protobuf::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthConfig {
    #[prost(message, repeated, tag="1")]
    pub handlers: ::prost::alloc::vec::Vec<AuthHandlerConfig>,
    #[prost(string, repeated, tag="2")]
    pub tls_auth_entities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub external_auth_config: ::core::option::Option<ExternalAuthConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwksFile {
    /// JSON Web Keys (JWKS) file as arbitary json.
    /// See <https://www.rfc-editor.org/rfc/rfc7517>
    #[prost(message, optional, tag="1")]
    pub json: ::core::option::Option<super::super::super::google::protobuf::Struct>,
}
/// ExternalAuthConfig describes how a viam managed robot can accept
/// credentials signed by the cloud app.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAuthConfig {
    #[prost(message, optional, tag="1")]
    pub jwks: ::core::option::Option<JwksFile>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthHandlerConfig {
    #[prost(enumeration="CredentialsType", tag="1")]
    pub r#type: i32,
    #[prost(message, optional, tag="5")]
    pub config: ::core::option::Option<super::super::super::google::protobuf::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Frame {
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub translation: ::core::option::Option<Translation>,
    #[prost(message, optional, tag="3")]
    pub orientation: ::core::option::Option<Orientation>,
    #[prost(message, optional, tag="4")]
    pub geometry: ::core::option::Option<super::super::common::v1::Geometry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogConfiguration {
    #[prost(string, tag="1")]
    pub level: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Translation {
    #[prost(double, tag="1")]
    pub x: f64,
    #[prost(double, tag="2")]
    pub y: f64,
    #[prost(double, tag="3")]
    pub z: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Orientation {
    #[prost(oneof="orientation::Type", tags="1, 2, 3, 4, 5, 6")]
    pub r#type: ::core::option::Option<orientation::Type>,
}
/// Nested message and enum types in `Orientation`.
pub mod orientation {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoOrientation {
    }
    /// OrientationVector containing ox, oy, oz, theta represents an orientation vector
    /// Structured similarly to an angle axis, an orientation vector works differently. Rather than representing an orientation
    /// with an arbitrary axis and a rotation around it from an origin, an orientation vector represents orientation
    /// such that the ox/oy/oz components represent the point on the cartesian unit sphere at which your end effector is pointing
    /// from the origin, and that unit vector forms an axis around which theta rotates. This means that incrementing/decrementing
    /// theta will perform an in-line rotation of the end effector.
    /// Theta is defined as rotation between two planes: the plane defined by the origin, the point (0,0,1), and the rx,ry,rz
    /// point, and the plane defined by the origin, the rx,ry,rz point, and the new local Z axis. So if theta is kept at
    /// zero as the north/south pole is circled, the Roll will correct itself to remain in-line.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OrientationVectorRadians {
        #[prost(double, tag="1")]
        pub theta: f64,
        #[prost(double, tag="2")]
        pub x: f64,
        #[prost(double, tag="3")]
        pub y: f64,
        #[prost(double, tag="4")]
        pub z: f64,
    }
    /// OrientationVectorDegrees is the orientation vector between two objects, but expressed in degrees rather than radians.
    /// Because protobuf Pose is in degrees, this is necessary.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OrientationVectorDegrees {
        #[prost(double, tag="1")]
        pub theta: f64,
        #[prost(double, tag="2")]
        pub x: f64,
        #[prost(double, tag="3")]
        pub y: f64,
        #[prost(double, tag="4")]
        pub z: f64,
    }
    /// EulerAngles are three angles (in radians) used to represent the rotation of an object in 3D Euclidean space
    /// The Tait–Bryan angle formalism is used, with rotations around three distinct axes in the z-y′-x″ sequence.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EulerAngles {
        #[prost(double, tag="1")]
        pub roll: f64,
        #[prost(double, tag="2")]
        pub pitch: f64,
        #[prost(double, tag="3")]
        pub yaw: f64,
    }
    /// See here for a thorough explanation: <https://en.wikipedia.org/wiki/Axis%E2%80%93angle_representation>
    /// Basic explanation: Imagine a 3d cartesian grid centered at 0,0,0, and a sphere of radius 1 centered at
    /// that same point. An orientation can be expressed by first specifying an axis, i.e. a line from the origin
    /// to a point on that sphere, represented by (rx, ry, rz), and a rotation around that axis, theta.
    /// These four numbers can be used as-is (R4), or they can be converted to R3, where theta is multiplied by each of
    /// the unit sphere components to give a vector whose length is theta and whose direction is the original axis.
    /// AxisAngles represents an R4 axis angle.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AxisAngles {
        #[prost(double, tag="1")]
        pub theta: f64,
        #[prost(double, tag="2")]
        pub x: f64,
        #[prost(double, tag="3")]
        pub y: f64,
        #[prost(double, tag="4")]
        pub z: f64,
    }
    /// Quaternion is a float64 precision quaternion.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Quaternion {
        #[prost(double, tag="1")]
        pub w: f64,
        #[prost(double, tag="2")]
        pub x: f64,
        #[prost(double, tag="3")]
        pub y: f64,
        #[prost(double, tag="4")]
        pub z: f64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        NoOrientation(NoOrientation),
        #[prost(message, tag="2")]
        VectorRadians(OrientationVectorRadians),
        #[prost(message, tag="3")]
        VectorDegrees(OrientationVectorDegrees),
        #[prost(message, tag="4")]
        EulerAngles(EulerAngles),
        #[prost(message, tag="5")]
        AxisAngles(AxisAngles),
        #[prost(message, tag="6")]
        Quaternion(Quaternion),
    }
}
/// A RemoteConfig describes a remote robot that should be integrated.
/// The Frame field defines how the "world" node of the remote robot should be reconciled with the "world" node of the
/// the current robot. All components of the remote robot who have Parent as "world" will be attached to the parent defined
/// in Frame, and with the given offset as well.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub frame: ::core::option::Option<Frame>,
    #[prost(message, optional, tag="4")]
    pub auth: ::core::option::Option<RemoteAuth>,
    #[prost(string, tag="5")]
    pub managed_by: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub insecure: bool,
    #[prost(message, optional, tag="7")]
    pub connection_check_interval: ::core::option::Option<super::super::super::google::protobuf::Duration>,
    #[prost(message, optional, tag="8")]
    pub reconnect_interval: ::core::option::Option<super::super::super::google::protobuf::Duration>,
    #[prost(message, repeated, tag="9")]
    pub service_configs: ::prost::alloc::vec::Vec<ResourceLevelServiceConfig>,
    /// Secret is a helper for a robot location secret.
    #[prost(string, tag="10")]
    pub secret: ::prost::alloc::string::String,
}
/// RemoteAuth specifies how to authenticate against a remote. If no credentials are
/// specified, authentication does not happen. If an entity is specified, the
/// authentication request will specify it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteAuth {
    #[prost(message, optional, tag="1")]
    pub credentials: ::core::option::Option<remote_auth::Credentials>,
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RemoteAuth`.
pub mod remote_auth {
    /// Credentials packages up both a type of credential along with its payload which
    /// is formatted specific to the type.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Credentials {
        #[prost(enumeration="super::CredentialsType", tag="1")]
        pub r#type: i32,
        #[prost(string, tag="2")]
        pub payload: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentInfo {
    #[prost(string, tag="1")]
    pub host: ::prost::alloc::string::String,
    /// Will soon be deprecated, use platform instead
    #[prost(string, tag="2")]
    pub os: ::prost::alloc::string::String,
    /// list of all ipv4 ips.
    #[prost(string, repeated, tag="3")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// RDK version
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub git_revision: ::prost::alloc::string::String,
    /// The platform the RDK is running on. For example linux/amd64
    #[prost(string, optional, tag="6")]
    pub platform: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional tags to further constrain which artifact is returned for modules.
    #[prost(string, repeated, tag="7")]
    pub platform_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRequest {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Details about the RDK (os, version) are updated during this request.
    #[prost(message, optional, tag="2")]
    pub agent_info: ::core::option::Option<AgentInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigResponse {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<RobotConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateRequest {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateResponse {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tls_certificate: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tls_private_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogRequest {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub logs: ::prost::alloc::vec::Vec<super::super::common::v1::LogEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeedsRestartRequest {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeedsRestartResponse {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub must_restart: bool,
    #[prost(message, optional, tag="3")]
    pub restart_check_interval: ::core::option::Option<super::super::super::google::protobuf::Duration>,
}
/// ModuleConfig is the configuration for a module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// path to the executable
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    /// log level for module
    #[prost(string, tag="3")]
    pub log_level: ::prost::alloc::string::String,
    /// type of the module ("local" or "registry")
    #[prost(string, tag="4")]
    pub r#type: ::prost::alloc::string::String,
    /// the id of the module if it is a registry module
    #[prost(string, tag="5")]
    pub module_id: ::prost::alloc::string::String,
    /// additional environment variables passed to the module process
    #[prost(map="string, string", tag="6")]
    pub env: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// info about the validity of the module
    #[prost(message, optional, tag="7")]
    pub status: ::core::option::Option<AppValidationStatus>,
    /// timeout for first_run script
    #[prost(message, optional, tag="8")]
    pub first_run_timeout: ::core::option::Option<super::super::super::google::protobuf::Duration>,
}
/// PackageConfig is the configration for deployed Packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageConfig {
    /// Name is the local name of the package on the RDK. Must be unique across Packages. Must not be empty.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Package is the unique package name hosted by Viam. Must not be empty.
    #[prost(string, tag="2")]
    pub package: ::prost::alloc::string::String,
    /// version of the package ID hosted by Viam. If not specified "latest" is assumed.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// type of the package
    #[prost(string, tag="4")]
    pub r#type: ::prost::alloc::string::String,
    /// info about the validity of the package
    #[prost(message, optional, tag="5")]
    pub status: ::core::option::Option<AppValidationStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenanceConfig {
    #[prost(message, optional, tag="1")]
    pub sensor_name: ::core::option::Option<super::super::common::v1::ResourceName>,
    #[prost(string, tag="2")]
    pub maintenance_allowed_key: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CredentialsType {
    Unspecified = 0,
    Internal = 1,
    ApiKey = 2,
    RobotSecret = 3,
    RobotLocationSecret = 4,
}
impl CredentialsType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CredentialsType::Unspecified => "CREDENTIALS_TYPE_UNSPECIFIED",
            CredentialsType::Internal => "CREDENTIALS_TYPE_INTERNAL",
            CredentialsType::ApiKey => "CREDENTIALS_TYPE_API_KEY",
            CredentialsType::RobotSecret => "CREDENTIALS_TYPE_ROBOT_SECRET",
            CredentialsType::RobotLocationSecret => "CREDENTIALS_TYPE_ROBOT_LOCATION_SECRET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CREDENTIALS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CREDENTIALS_TYPE_INTERNAL" => Some(Self::Internal),
            "CREDENTIALS_TYPE_API_KEY" => Some(Self::ApiKey),
            "CREDENTIALS_TYPE_ROBOT_SECRET" => Some(Self::RobotSecret),
            "CREDENTIALS_TYPE_ROBOT_LOCATION_SECRET" => Some(Self::RobotLocationSecret),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
