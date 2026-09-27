// 手写补充类型（非 jumo 生成）：管理台入口 `AdminListGatewayStatus` 的响应外壳。
//
// 对应 `GET /api/v1/admin/gateways/status`，返回状态卡片列表（喂前端卡片栅格）。

/// `AdminListGatewayStatus` 的响应体：网关运行态列表。
///
/// 空列表是合法结果（例如尚无任何网关上报过状态），调用方需能区分「空列表」与「请求失败」。
#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct AdminGatewayStatusListReturned {
    /// 各网关的运行态快照；顺序由实现层决定（当前按 `gateway_id` 升序）。
    pub statuses: Vec<crate::GatewayRuntimeStatus>,
}
