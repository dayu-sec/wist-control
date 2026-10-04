// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "message",
    role = "command",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct ReportGatewayUpgradeResult {
    pub gateway_id: String,
    pub work_id: String,
    pub from_version: String,
    pub to_version: String,
    pub step: String,
    pub status: String,
    pub detail: String,
    pub reported_at: crate::DateTime,
}
