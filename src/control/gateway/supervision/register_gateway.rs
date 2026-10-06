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
    // NOTE(hand-added): 网关对外基址（对外域名）。管理面「对外地址」优先，未设回落本机 `[server] public_base_url`。
    // 可选 —— 老 gwlinkd 不带这个键，中心侧 `#[serde(default)]` 兜底。
    #[serde(default)]
    pub public_base_url: Option<String>,
    /// 网关侧生成的证书签名请求（CSR，PEM）：公钥在其中，私钥永不出网关。
    pub certificate_signing_request: String,
    pub requested_at: crate::DateTime,
}
