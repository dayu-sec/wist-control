// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct GatewayRuntimeStatus {
    pub gateway_id: String,
    pub instance_id: String,
    pub version: String,
    pub status: String,
    pub health: String,
    pub memory_bytes: Option<i64>,
    pub cpu_percent: Option<f64>,
    pub last_seen_at: crate::DateTime,
    // NOTE(hand-added): 网关状态富化（机队聚合 + 进程运行时长）；见设计 edge/gateway-status-report.md。
    #[serde(default)]
    pub uptime_seconds: Option<i64>,
    #[serde(default)]
    pub agent_count: Option<i64>,
    #[serde(default)]
    pub online_agents: Option<i64>,
    #[serde(default)]
    pub offline_agents: Option<i64>,
    #[serde(default)]
    pub last_seen_lag_seconds: Option<i64>,
    #[serde(default)]
    pub store_bytes: Option<i64>,
    #[serde(default)]
    pub ingest_accepted_total: Option<i64>,
    #[serde(default)]
    pub ingest_rejected_total: Option<i64>,
    #[serde(default)]
    pub last_ingest_at: Option<crate::DateTime>,
    #[serde(default)]
    pub memory_total_bytes: Option<i64>,
    #[serde(default)]
    pub load_1m: Option<f64>,
    #[serde(default)]
    pub load_5m: Option<f64>,
    #[serde(default)]
    pub load_15m: Option<f64>,
    #[serde(default)]
    pub disk_usage_percent: Option<f64>,
    #[serde(default)]
    pub disk_total_bytes: Option<i64>,
    #[serde(default)]
    pub disk_available_bytes: Option<i64>,
}
