// @jumo generated
use super::{RolloutPlan, RolloutPlanEntry};

#[derive(
    Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo,
)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Rollout")]
pub struct RolloutPlanView {
    pub plan: RolloutPlan,
    pub entries: Vec<RolloutPlanEntry>,
}
