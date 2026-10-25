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
}
