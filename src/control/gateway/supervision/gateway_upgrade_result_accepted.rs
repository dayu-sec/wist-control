#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct GatewayUpgradeResultAccepted {
    pub gateway_id: String,
    pub work_id: String,
    pub accepted_at: crate::DateTime,
}
