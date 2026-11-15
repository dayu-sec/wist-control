// @jumo generated
#[derive(
    Debug, Clone, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct DesiredBusinessIdentity {
    // NOTE(hand-added): 中心**权威**期望值（绑定表 customer_id → 数据面 tenant_id）。
    // 随状态回执下发，用于命令丢失 / 离线 / 重启后的补偿（设计 §11.1）。
    pub tenant_id: String,
    pub revision: i64,
    // NOTE(hand-added): wparse 事件号的实例段（`WP_INS_ID`）。旧中心不带这个键 → `None`，
    // 网关保持现场已有的实例号不动。
    #[serde(default)]
    pub wp_ins_id: Option<i64>,
}
