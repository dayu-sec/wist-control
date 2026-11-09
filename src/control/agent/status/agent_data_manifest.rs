// NOTE(hand-added): 一台 Agent 的**数据清单**：采集授权 + 调查数据。

use super::{AgentCollectionEntry, AgentInvestigationEntry};
//
// 为什么挂在 `AgentRuntimeStatus` 上：中心只知道「这台机器」这个对象，清单是它的一个属性；
// 另开一条上报通道会多一套队列、幂等与版本矩阵，而两者本来就在同一次状态心跳里。
//
// 内容按清单口径裁剪：采集面给「授权 + 来源 + 实测结论」，调查给「报告摘要」，
// 都不带正文 —— 中心页回答的是「这台机器上有什么数据」，不是「数据里写了什么」。
#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Agent.Status")]
pub struct AgentDataManifest {
    /// 这份清单的生成时刻（网关时钟）。
    pub collected_at: crate::control::types::DateTime,
    /// 采集授权（每个生效/暂停的采集面一条）。
    pub collection: Vec<AgentCollectionEntry>,
    /// 调查数据（最近若干份报告的摘要，新 → 旧）。
    pub investigations: Vec<AgentInvestigationEntry>,
}
