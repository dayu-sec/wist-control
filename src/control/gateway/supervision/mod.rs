// @jumo generated
// Model module: Control.Gateway.Supervision

// NOTE(hand-added): `wp_event_id` 的 10 bit 实例段（wparse 的 `WP_INS_ID`）。
// 取值区间来自上游契约（`wp-examples/core/instance_id`）：`instance_id - 100` 占 0..=1023，
// 所以 100..=1123 是全部可用车道。**多站点汇到同一个中心时必须互不相同**，
// 否则同一秒发号会产重复 ID；越界 / 非数字会让 wparse 启动失败（有意的 fail-closed）。
pub const WP_INS_ID_MIN: i64 = 100;
pub const WP_INS_ID_MAX: i64 = 1123;
/// 未设置时 wparse 自己回退到 100（并打 WARN）；模型侧缺省值与它对齐。
pub const WP_INS_ID_DEFAULT: i64 = 100;

/// 是否是合法的 `wp_ins_id`（纯函数，中心与网关共用同一口径）。
pub fn is_valid_wp_ins_id(value: i64) -> bool {
    (WP_INS_ID_MIN..=WP_INS_ID_MAX).contains(&value)
}

pub mod gateway_initial_config;
pub use gateway_initial_config::*;
pub mod gateway_status_accepted;
pub use gateway_status_accepted::*;
pub mod desired_business_identity;
pub use desired_business_identity::*;
pub mod gateway_runtime_status;
pub use gateway_runtime_status::*;
pub mod gateway_instance;
pub use gateway_instance::*;
pub mod gateway_instance_lifecycle_state;
pub use gateway_instance_lifecycle_state::*;
pub mod gateway_customer_binding;
pub use gateway_customer_binding::*;
pub mod gateway_list_view;
pub use gateway_list_view::*;
pub mod global_policy_dispatch;
pub use global_policy_dispatch::*;
pub mod link_upstream;
pub use link_upstream::*;
pub mod initialize_gateway_via_url;
pub use initialize_gateway_via_url::*;
pub mod report_gateway_status;
pub use report_gateway_status::*;
pub mod get_gateway_upgrade_plan;
pub use get_gateway_upgrade_plan::*;
pub mod gateway_upgrade_plan;
pub use gateway_upgrade_plan::*;
pub mod report_gateway_upgrade_result;
pub use report_gateway_upgrade_result::*;
pub mod gateway_upgrade_result_accepted;
pub use gateway_upgrade_result_accepted::*;
pub mod register_gateway;
pub use register_gateway::*;
pub mod renew_gateway_credential;
pub use renew_gateway_credential::*;
pub mod verify_gateway_credential;
pub use verify_gateway_credential::*;
pub mod gateway_enrollment_result;
pub use gateway_enrollment_result::*;
