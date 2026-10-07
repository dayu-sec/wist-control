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
pub struct CreateRolloutPlan {
    pub action: String,
    pub spec: String,
    pub target_ids: Vec<String>,
    pub phase_count: i64,
    pub deadline_at: crate::DateTime,
    pub timeout_seconds: i64,
    pub batch_size: i64,
    pub requested_by: String,
}
