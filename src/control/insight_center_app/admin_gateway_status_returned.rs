// 手写补充类型（非 jumo 生成）：管理台入口 `AdminShowGatewayStatus` 的响应外壳。
//
// 对应 `GET /api/v1/admin/gateways/{gateway_id}/status`，返回单个网关的运行态。

/// `AdminShowGatewayStatus` 的响应体：单个网关的运行态快照。
#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct AdminGatewayStatusReturned {
    /// 目标网关的运行态快照。
    pub status: crate::GatewayRuntimeStatus,
}
