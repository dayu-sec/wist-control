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
pub struct ApproveRolloutPlan {
    pub plan_id: String,
    pub approved_by: String,
    pub requested_by: String,
}
