// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "message",
    role = "query",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct GetGatewayUpgradePlan {
    pub gateway_id: String,
}
