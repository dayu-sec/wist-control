// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct GatewayInitialConfig {
    pub gateway_id: String,
    pub control_center_endpoint: String,
    pub trust_bundle: Option<crate::ControlCenterTrustBundle>,
    pub server_tls_required: bool,
    pub protocol_version: String,
    pub enrollment_token_id: String,
    // NOTE(hand-added): 首次接入时随配置带出业务身份；老 gwlinkd 忽略这两个键。
    #[serde(default)]
    pub tenant_id: Option<String>,
    #[serde(default)]
    pub binding_revision: Option<i64>,
    // NOTE(hand-added): wparse 事件号的实例段（`WP_INS_ID`）—— 首次接入就带出去，
    // 网关落地后不必等一次 `bind-tenant` 命令。老 gwlinkd 忽略。
    #[serde(default)]
    pub wp_ins_id: Option<i64>,
}
