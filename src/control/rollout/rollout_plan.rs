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

// NOTE(hand-added): 计划动作（`RolloutPlan.action` / `GatewayUpgradePlan.action`）的**值**。
// 中心与 `wist-gwlinkd` 共用这一份，避免两处字面量漂移（漂移会把 `push-agent-package`
// 静默当 `upgrade`）。见设计 `wist-design/doc/design/edge/agent-package-push-to-gateways.md`。

/// 升级安装：中心推下去安装（宿主侧 gops / tool-copy）。
pub const ACTION_UPGRADE: &str = "upgrade";
/// Agent 包下发：中心把 `wist-agentd` 包交给网关包管理，**升不升由网关决定**（发布 ②）。
pub const ACTION_PUSH_AGENT_PACKAGE: &str = "push-agent-package";
