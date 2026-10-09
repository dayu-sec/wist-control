// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "struct",
    domain = "Control",
    module = "Control.Gateway.Supervision"
)]
pub struct GatewayUpgradePlan {
    pub gateway_id: String,
    pub has_plan: bool,
    pub plan_id: Option<String>,
    pub component: Option<String>,
    pub to_version: Option<String>,
    // NOTE(hand-added): 中心**派生**的制品下发地址（镜像后的可下载 URL）。
    // 执行器据此取件（gops `--to <url>`）；缺省 = 无对应 release 记录，回落用 `to_version`。
    #[serde(default)]
    pub artifact_url: Option<String>,
    // NOTE(hand-added): 计划动作（`upgrade` / `push-agent-package`）。缺省 = 老中心，按 `upgrade` 处理；
    // gwlinkd 据此分派执行器：升级走 gops，包下发走环回写网关包管理。
    #[serde(default)]
    pub action: Option<String>,
    // NOTE(hand-added): `artifact_url` 的期望摘要（sha256，裸 hex / `sha256:` 前缀）。
    // **仅**「agent 包下发」（②）带它；① 升级路径为空（给 ① 加摘要会改变其既有取件/校验行为）。
    // 见设计 `edge/agent-package-push-to-gateways.md`。
    #[serde(default)]
    pub artifact_sha256: Option<String>,
}
