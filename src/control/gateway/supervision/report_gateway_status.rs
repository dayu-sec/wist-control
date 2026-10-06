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
pub struct ReportGatewayStatus {
    pub gateway_id: String,
    pub instance_id: String,
    // NOTE(hand-added): 网关对外基址（对外域名）。管理面「对外地址」优先，未设回落本机 `[server] public_base_url`。
    // 可选 —— 老版本网关不带这个键，中心侧 `#[serde(default)]` 兜底。
    #[serde(default)]
    pub public_base_url: Option<String>,
    pub version: String,
    pub status: String,
    pub health: String,
    pub memory_bytes: Option<i64>,
    pub cpu_percent: Option<f64>,
    pub reported_at: crate::DateTime,
    // NOTE(hand-added): 网关状态富化字段（与自述面 GatewaySelfState 同一份值）。
    // 全部可选 —— 老版本网关不带这些键，中心侧 `#[serde(default)]` 兜底。
    // 见设计 `wist-design/doc/design/edge/gateway-status-report.md`。
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
