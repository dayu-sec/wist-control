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
}
