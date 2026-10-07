// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(
    kind = "message",
    role = "command",
    domain = "Control",
    module = "Control.RolloutApp.AdminInterface"
)]
pub struct AdvanceRolloutPlan {
    pub plan_id: String,
    pub requested_by: String,
}
