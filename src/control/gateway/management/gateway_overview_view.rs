// @jumo generated
#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct GatewayOverviewView {
    pub gateway_id: String,
    pub agent_total: i64,
    pub agent_online: i64,
    pub agent_unhealthy: i64,
    pub status: String,
    pub updated_at: crate::DateTime,
}
