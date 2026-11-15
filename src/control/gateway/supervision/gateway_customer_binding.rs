// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct GatewayCustomerBinding {
    pub gateway_id: String,
    pub customer_id: String,
    pub status: String,
    pub bound_at: crate::DateTime,
    // NOTE(hand-added): 绑定版本 / 下发回执（管理面仍以 `customer_id` 为字段名）。
    // 旧数据缺这些键 → `serde(default)` 兜底。缺省值与 store 侧对齐（1，不是 0 ——
    // `0` 在任何 `>=` 比较里都会无条件成立）。
    #[serde(default = "default_binding_revision")]
    pub revision: i64,
    // NOTE(hand-added): wparse 事件号的实例段（`WP_INS_ID`，100..=1123）。
    // 旧数据缺这个键 → 缺省 100（与 wparse / 网关栈的默认实例号一致）。
    #[serde(default = "default_wp_ins_id")]
    pub wp_ins_id: i64,
    // NOTE(hand-added): 显式释放的时刻 / 操作者。`None` = 仍占用（全中心唯一性只看这一对为空的行）。
    #[serde(default)]
    pub wp_ins_id_released_at: Option<crate::DateTime>,
    #[serde(default)]
    pub wp_ins_id_released_by: Option<String>,
    #[serde(default)]
    pub applied_revision: Option<i64>,
    #[serde(default)]
    pub applied_at: Option<crate::DateTime>,
    #[serde(default)]
    pub last_error: Option<String>,
}

fn default_binding_revision() -> i64 {
    1
}

fn default_wp_ins_id() -> i64 {
    crate::WP_INS_ID_DEFAULT
}
