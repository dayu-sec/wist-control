// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Rollout")]
pub struct RolloutPlanEntry {
    pub plan_id: String,
    pub target_id: String,
    pub work_id: Option<String>,
    pub status: String,
    pub detail: Option<String>,
    pub updated_at: crate::DateTime,
}
