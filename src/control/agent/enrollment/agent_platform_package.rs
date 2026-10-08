// @jumo generated
// @jumo hash=0000000000000000

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Agent.Enrollment"
)]
pub struct AgentPlatformPackage {
    // 目标平台（target-triple），如 `aarch64-apple-darwin` / `x86_64-unknown-linux-musl`。
    // 网关按它把安装脚本与托管包一一对上（多平台 agentd 包）。
    pub platform: String,
    // 该平台的安装脚本地址（对平台限定）。
    pub install_script_url: String,
    // 网关托管并由其派生的下载地址（agent 只从网关取包）。
    pub agent_package_url: String,
    // 与实际会服务出去的那份制品同源的 sha256（裸 hex 或 `sha256:` 前缀）。
    pub agent_package_sha256: String,
}
