// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "message",
    role = "command",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct RegisterGateway {
    pub enrollment_token: String,
    pub instance_id: String,
    /// 网关侧生成的证书签名请求（CSR，PEM）：公钥在其中，私钥永不出网关。
    pub certificate_signing_request: String,
    pub requested_at: crate::DateTime,
}
