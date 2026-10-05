// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "message",
    role = "response",
    domain = "Control",
    module = "Control.GatewayApp.FacingInterface"
)]
pub struct AgentStatusAcceptedReturned {
    pub gateway_id: String,
    pub agents_accepted: i64,
}
