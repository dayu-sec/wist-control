// NOTE(hand-added): 数据清单里的一份**调查数据**（一条已落盘的调查报告的摘要）。
//
// 只带列表页需要的结论与计数，**不带报告正文**：正文可能有几十 KB，且属于数据面证据，
// 由网关自己的「调查报告」页按需读取。中心拿到的是「有什么数据」，不是数据本身。
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Agent.Status")]
pub struct AgentInvestigationEntry {
    /// 与下发命令 / ack 的关联键（报告走数据面，靠它 join）。
    pub command_id: String,
    /// 调查面 id（`disk` / `load` / `accounts` …）。
    pub topic: String,
    /// 报告观测时刻（RFC3339，原样转述）。
    pub observed_at: crate::control::types::DateTime,
    /// 结论档：`ok` / `warn` / `fail`；报告里没写就是 `unknown`。
    pub overall: String,
    /// 报告级判定（`ok` / `warn` / `fail` / `unknown`）。
    pub verdict: String,
    pub findings_ok: i64,
    pub findings_warn: i64,
    pub findings_fail: i64,
    /// 无法判定（读不到 / 平台不支持 / 源不存在）。**不能省**：少了它，“没读到”会被当成“正常”。
    pub findings_unknown: i64,
    /// 这一题做不做安全判定；`false`（如 `exposure`）时不得渲染成“全部正常”。
    pub judged: bool,
}
