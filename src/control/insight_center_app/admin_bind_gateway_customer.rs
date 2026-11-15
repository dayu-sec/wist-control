// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "message",
    role = "command",
    domain = "Control",
    module = "Control.InsightCenterApp.AdminFacingInterface"
)]
pub struct AdminBindGatewayCustomer {
    pub gateway_id: String,
    pub customer_id: String,
    // wparse 事件号的实例段（`wp_event_id` 的 10 bit 实例位）；多站点汇到同一中心时必须互不相同。
    pub wp_ins_id: i64,
    pub requested_by: String,
}
