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
    // NOTE(hand-added): 任务执行记录 —— 目录（网关本机路径）+ 摘要（写入时已脱敏、有界）。
    // 可选 —— 老网关不带这两个键，中心侧 `#[serde(default)]` 兜底，并继续从 `detail` 里的
    // `日志: <dir>` 解析目录。见设计 `doc/design/edge/task-log-report.md`。
    #[serde(default)]
    pub log_dir: Option<String>,
    #[serde(default)]
    pub log_excerpt: Option<String>,
    pub reported_at: crate::DateTime,
}
