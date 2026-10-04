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
}
