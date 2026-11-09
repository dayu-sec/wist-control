// NOTE(hand-added): 数据清单里的一条**采集来源**。
//
// 与 `wist_contracts::work::WorkSpecSource` 同源（`kind` + `target`），但这里只带
// 「采什么」的稳定描述，不带 multiline / filters 这类执行参数 —— 清单是给中心看的
// **数据目录**，不是可重放的工作副本。
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Agent.Status")]
pub struct AgentDataSource {
    /// `FileGlob` / `Exporter` / `UnifiedLogPredicate` / `MetricInterval` /
    /// `KafkaCluster` / `MysqlInstance` / `PostgresInstance` / `DorisCluster`。
    pub kind: String,
    /// 文件路径 / exporter 标识 / 指标间隔 / `auto`（自探）。
    pub target: String,
}
