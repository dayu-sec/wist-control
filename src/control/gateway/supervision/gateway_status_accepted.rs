// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct GatewayStatusAccepted {
    pub gateway_id: String,
    pub instance_id: String,
    pub accepted_at: crate::DateTime,
    // NOTE(hand-added): 中心权威期望的业务身份（无绑定 → `None`）。
    // 网关据此在本地落后/未生效时自行补偿（命令丢失 / 离线 / 重启场景），见设计 §11.1。
    // 可选 —— 旧中心不带这个键，`serde(default)` 兜底。
    #[serde(default)]
    pub desired_business_identity: Option<crate::DesiredBusinessIdentity>,
}
