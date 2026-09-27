// 手写补充类型（非 jumo 生成）：模型里没有对应 item，由中心侧 handler 聚合返回。
//
// `GatewayListView` 是管理台「网关列表」页首屏的聚合快照，不是领域实体：
// 它不随上报告警流式更新，而是在查询时刻由中心把各网关最近一次运行态归并出的计数。

/// 网关列表聚合视图：按在线状况分桶的网关台数快照。
///
/// 语义约定（由生产者保证，本类型为纯 DTO、不做校验）：`online_count` 与
/// `offline_count` 互补，二者之和等于 `gateway_count`；`degraded_count` 按健康度
/// 另算（降级网关仍算在线），不参与上述二划分。`updated_at` 标明快照的生成时刻。
#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct GatewayListView {
    /// 纳入统计的网关总数。
    pub gateway_count: i64,
    /// 在线网关台数。
    pub online_count: i64,
    /// 降级（存活但健康异常）网关台数。
    pub degraded_count: i64,
    /// 离线网关台数。
    pub offline_count: i64,
    /// 本快照的生成时刻。
    pub updated_at: crate::DateTime,
}
