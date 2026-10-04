// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "message",
    role = "query",
    domain = "Control",
    module = "Control.GatewayApp.FacingInterface"
)]
pub struct QueryGatewayInitializationStatus {
    pub gateway_id: String,
}
