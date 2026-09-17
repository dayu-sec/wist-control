// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Management"
)]
pub struct GatewayHealth {
    pub gateway_id: String,
    pub status: String,
    pub reported_at: crate::DateTime,
}
