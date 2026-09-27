// 手写补充类型（非 jumo 生成）：管理台入口 `AdminViewGatewayList` 的响应外壳。
//
// 模型把该入口的返回声明为 `Projection<List<GatewayInstance>>`，而实现层返回的是
// 聚合快照 [`GatewayListView`]，故这里手写一层单字段外壳承载该读投影。

/// `AdminViewGatewayList` 的响应体：包一层 `list` 字段，便于前端按字段取值。
#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct AdminGatewayListReturned {
    /// 网关列表聚合视图。
    pub list: crate::GatewayListView,
}
