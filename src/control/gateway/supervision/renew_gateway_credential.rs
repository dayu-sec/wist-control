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
pub struct RenewGatewayCredential {
    pub gateway_id: String,
    pub current_certificate_serial: String,
    /// 网关为新证书生成的 CSR（轮换：用旧证书证明身份）。
    pub certificate_signing_request: String,
    pub requested_at: crate::DateTime,
}
