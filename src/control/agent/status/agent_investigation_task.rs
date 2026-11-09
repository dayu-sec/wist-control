// NOTE(hand-added): 一条调查任务的**进度**（网关注入，随 Agent 状态上报）。
//
// 与 `AgentInvestigationEntry` 不是一回事：那条答「这台机器有什么报告」，这条答
// 「这条命令跑到哪了」—— 在飞的任务还没有报告，但状态已经有了。所以两段分开存，
// 保留期也不同（任务状态只留最近 3 条 / 7 天）。
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Agent.Status")]
pub struct AgentInvestigationTask {
    /// 中心铸的号，网关建命令行时用它 —— 报告正文与证据库索引也是它。
    pub command_id: String,
    /// 调查面 id（`disk` / `load` / `accounts` / `persistence` / `exposure` / `telemetry`）。
    pub topic: String,
    pub requested_at: crate::control::types::DateTime,
    /// 网关收到 agentd 阶段 ack（`outcome=accepted`）的时刻；`None` = 还没接单。
    pub claimed_at: Option<crate::control::types::DateTime>,
    /// 终态 ack（`succeeded` / `failed`）的时刻；`None` = 还在跑，或错过了回执。
    pub acked_at: Option<crate::control::types::DateTime>,
    /// `succeeded` / `failed`；未回执时为 `None`（不得默认成 `succeeded`）。
    pub outcome: Option<String>,
    /// 一行摘要（≤ 900 字）；正文在证据库，按 `command_id` 取。
    pub detail: Option<String>,
}

/// 把 `null` 也当成空表。
///
/// 契约里“没报”用缺字段表达（`#[serde(default)]`），但外部生产者可能写成 `"investigations": null`。
/// 一个字段的脏值不能拖垮**整份**机队状态上报（那是所有 Agent 一起丢），所以这里宽进。
pub(crate) fn deserialize_vec_or_null<'de, D>(
    deserializer: D,
) -> Result<Vec<AgentInvestigationTask>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(
        <Option<Vec<AgentInvestigationTask>> as serde::Deserialize>::deserialize(deserializer)?
            .unwrap_or_default(),
    )
}
