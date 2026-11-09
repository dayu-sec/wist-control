// NOTE(hand-added): 数据清单里的**一条采集授权**（一个采集面 = 一份常驻工作）。

use super::AgentDataSource;
//
// 期望侧（`status` / `plan_version`）来自网关的 `StandingWork`；
// 实测侧（`collection_status` / `collection_detail`）来自 agentd 上报的 `AgentLocalWork`。
// 两侧都缺时字段为 `null` —— 「没报过」不等于「正常」。
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
    /// agentd 的实测采集结论：`ok` / `degraded` / `failed` / `unknown` / `paused`。
    /// `null` = 本机还没报过（旧 agentd）。
    pub collection_status: Option<String>,
    /// 失败 / 降级的人话说明；`ok` 与 `null` 时为空。
    pub collection_detail: Option<String>,
    /// 这份授权实际覆盖的来源（文件路径 / exporter / 指标间隔 …）。
    pub inputs: Vec<AgentDataSource>,
}
