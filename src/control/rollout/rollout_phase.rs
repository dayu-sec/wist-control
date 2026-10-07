// @jumo generated
#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Rollout")]
pub struct RolloutPhase {
    pub phase_index: i64,
    pub target_ids: Vec<String>,
    pub advance_rule: String,
    pub status: String,
}
