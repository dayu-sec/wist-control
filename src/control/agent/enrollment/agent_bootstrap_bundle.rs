// @jumo generated
// @jumo hash=5a7809976f6c209f

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Agent.Enrollment"
)]
pub struct AgentBootstrapBundle {
    pub expires_at: crate::control::types::DateTime,
    pub environment_id: String,
    pub control_endpoint: String,
    pub tenant_id: String,
    #[jumo(unique)]
    pub bundle_id: String,
    // 每平台一份：安装脚本地址 + 网关托管包地址/摘要（agentd 是平台专用制品）。
    // 旧版的单值 `install_script_url` / `agent_package_url` / `agent_package_sha256` 已被本列表取代
    // —— 否则一个网关只能描述一个平台的包，其余平台装不到。
    pub platforms: Vec<crate::control::types::AgentPlatformPackage>,
    pub trust_bundle: String,
}
