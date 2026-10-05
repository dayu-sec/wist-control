// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct GatewayEnrollmentResult {
    pub status: String,
    pub gateway_id: String,
    pub instance_id: String,
    pub credential_id: String,
    pub initial_config: String,
    /// 注册成功同时签发的长期凭据（客户端证书）——网关侧必须拿到它才能接入。
    pub credential_bundle: crate::GatewayCredentialBundle,
}
