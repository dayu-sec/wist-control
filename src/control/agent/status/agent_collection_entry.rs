// NOTE(hand-added): 数据清单里的**一条采集授权**（一个采集面 = 一份常驻工作）。

use super::AgentDataSource;
//
// 期望侧（`status` / `plan_version`）来自网关的 `StandingWork`；
// 实测侧（`collection_*`）来自 agentd 上报的 `AgentLocalWork`。
// 两侧都缺时字段为 `null` —— 「没报过」不等于「正常」。
//
// 实测侧是**多字段模型**：`collection_prerequisites`（前置条件）+ `collection_result`
// （执行结果）+ `collection_reasons`（原因）。旧 `collection_status` / `collection_detail`
// 只在对方是旧 agentd 时使用，新消费方以 `collection_result` 为准。
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Agent.Status")]
pub struct AgentCollectionEntry {
    /// 采集面（`CollectionFamily`）。
    pub family: String,
    pub work_id: String,
    /// 授权状态：`active` / `paused`（撤回与取代的历史版本不进清单）。
    pub status: String,
    /// 网关的期望版本。
    pub plan_version: i64,
    /// agentd 回报过的版本；`null` = 还没回报（网关侧看到的就是漂移）。
    pub acknowledged_version: Option<i64>,
    /// **兼容字段**：旧 agentd 的单一状态（`ok` / `degraded` / `failed` / `unknown` / `paused`）。
    /// `null` = 本机还没报过（或新 agentd 只报了新字段）。
    pub collection_status: Option<String>,
    /// **兼容字段**：旧版人话说明。新模型用 `collection_reasons`。
    pub collection_detail: Option<String>,
    /// 前置条件：`ready` / `blocked`；`null` = 旧 agentd 没报。
    pub collection_prerequisites: Option<String>,
    /// 执行结果：`success` / `partial` / `failure` / `cancelled` / `pending`；`null` = 旧 agentd。
    pub collection_result: Option<String>,
    /// 原因列表（结构化）；`null` = 旧 agentd 没报。
    pub collection_reasons: Option<Vec<AgentCollectionReason>>,
    /// 这份授权实际覆盖的来源（文件路径 / exporter / 指标间隔 …）。
    pub inputs: Vec<AgentDataSource>,
}

/// 一条采集原因（与 `wist-contracts` 的同名结构同形，避免 control 依赖 reporting 契约）。
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Agent.Status")]
pub struct AgentCollectionReason {
    pub kind: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub detail: String,
}
