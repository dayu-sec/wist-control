// @jumo generated
// Control 模型域（可复用）：agent 控制面 + 中心治理模块

pub mod control;

pub use control::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_message_accepted_round_trips() {
        let message = ControlMessageAccepted {
            message_id: "msg-1".to_string(),
            agent_id: "agent-1".to_string(),
            instance_id: "inst-1".to_string(),
            accepted_at: "2026-09-14T00:00:00Z".to_string(),
        };
        let json = serde_json::to_string(&message).expect("serialize");
        let decoded: ControlMessageAccepted = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded.message_id, "msg-1");
        assert_eq!(decoded.agent_id, "agent-1");
        assert_eq!(decoded.instance_id, "inst-1");
        assert_eq!(decoded.accepted_at, "2026-09-14T00:00:00Z");
    }
}
