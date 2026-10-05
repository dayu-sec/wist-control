// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Security"
)]
pub struct GatewayCredentialBundle {
    pub credential_id: String,
    pub gateway_id: String,
    pub instance_id: Option<String>,
    /// 客户端证书（PEM）。mTLS 是网关与中心之间的**唯一**凭据路径（bearer / auth_scheme 已删），必填。
    pub certificate: String,
    pub ca_bundle: Option<String>,
    pub issued_at: crate::DateTime,
    pub not_before: Option<crate::DateTime>,
    pub not_after: Option<crate::DateTime>,
}
