// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Security"
)]
pub struct GatewayClientCertificate {
    pub certificate_id: String,
    pub gateway_id: String,
    pub serial: String,
    /// 客户端证书（PEM）。
    pub certificate: String,
    pub ca_bundle: Option<String>,
    pub issued_at: crate::DateTime,
    pub not_before: Option<crate::DateTime>,
    pub not_after: Option<crate::DateTime>,
    pub status: crate::GatewayClientCertificateStatus,
}
