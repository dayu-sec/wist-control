// @jumo generated
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(
    kind = "message",
    role = "command",
    domain = "Control",
    module = "Control.GatewayApp.FacingInterface"
)]
pub struct ReportAgentStatus {
    pub gateway_id: String,
    pub agents: Vec<crate::AgentRuntimeStatus>,
}
