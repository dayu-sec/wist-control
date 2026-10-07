// @jumo generated
use super::RolloutPhase;

#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Rollout")]
pub struct RolloutPlan {
    pub plan_id: String,
    pub action: String,
    pub spec: String,
    pub deadline_at: crate::DateTime,
    pub timeout_seconds: i64,
    pub phases: Vec<RolloutPhase>,
    pub batch_size: i64,
    pub current_phase: i64,
    pub status: String,
    pub created_by: String,
    pub created_at: crate::DateTime,
    pub approved_by: Option<String>,
    pub approved_at: Option<crate::DateTime>,
}
