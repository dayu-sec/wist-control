//! Smoke tests for the Control domain model.
//!
//! Every type in `wist-control` is a plain `serde` struct/enum generated from the
//! Control model. These tests verify that each representative type serializes and
//! deserializes stably over JSON. They exist mainly to give `cargo llvm-cov` real
//! coverage data and to catch accidental breakage in the generated `serde` derives.

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::json;

use wist_control::{
    AgentControlCommandsReturned, AgentCredentialVerificationStatus, AgentDownstreamMessageType,
    AgentEnrollmentAccepted, AgentEnrollmentAuthProfile, AgentEnrollmentRejected,
    AgentEnrollmentResultStatus, AgentEnrollmentTokenAccepted, AgentEnrollmentTokenRejected,
    AgentEnrollmentTokenStatus, AgentEnrollmentTokenValidationStatus, AgentIdentityStatus,
    AgentInitialConfig, AgentInstance, AgentPolicyBinding, AgentRuntimeStatus,
    AgentUpstreamMessageType, ControlCommandsReturned, ControlLongPollTimedOut,
    ControlMessageAccepted, ControlMessageRejected, GatewayCustomerBinding, GatewayIdentityStatus,
    GatewayInstanceLifecycleState, GatewayRuntimeStatus, GatewayStatusAccepted,
};

const TS: &str = "2026-09-14T00:00:00Z";

fn assert_enum<T>(variants: &[(T, &str)])
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    for (variant, name) in variants {
        let encoded = serde_json::to_value(variant).expect("serialize enum");
        assert_eq!(encoded, json!(name), "serialized variant name mismatch");
        let decoded: T = serde_json::from_value(encoded).expect("deserialize enum");
        assert_eq!(&decoded, variant, "deserialized variant mismatch");
    }
}

fn assert_round_trip<T>(value: serde_json::Value)
where
    T: Serialize + DeserializeOwned,
{
    let decoded: T = serde_json::from_value(value).expect("deserialize struct");
    let first = serde_json::to_value(&decoded).expect("serialize struct");
    let redecoded: T = serde_json::from_value(first.clone()).expect("re-deserialize struct");
    let second = serde_json::to_value(&redecoded).expect("re-serialize struct");
    assert_eq!(first, second, "serde round-trip should be stable");
}

#[test]
fn enums_serialize_and_round_trip() {
    assert_enum(&[
        (AgentCredentialVerificationStatus::Verified, "Verified"),
        (
            AgentCredentialVerificationStatus::UnknownAgent,
            "UnknownAgent",
        ),
        (
            AgentCredentialVerificationStatus::CredentialMissing,
            "CredentialMissing",
        ),
        (
            AgentCredentialVerificationStatus::CredentialExpired,
            "CredentialExpired",
        ),
        (
            AgentCredentialVerificationStatus::CredentialRevoked,
            "CredentialRevoked",
        ),
        (
            AgentCredentialVerificationStatus::CredentialMismatch,
            "CredentialMismatch",
        ),
    ]);

    assert_enum(&[
        (
            AgentDownstreamMessageType::EnrollmentResult,
            "EnrollmentResult",
        ),
        (
            AgentDownstreamMessageType::ControlCommands,
            "ControlCommands",
        ),
        (
            AgentDownstreamMessageType::PolicyRefreshHint,
            "PolicyRefreshHint",
        ),
        (
            AgentDownstreamMessageType::IdentityRotationHint,
            "IdentityRotationHint",
        ),
    ]);

    assert_enum(&[
        (AgentEnrollmentResultStatus::Accepted, "Accepted"),
        (AgentEnrollmentResultStatus::Rejected, "Rejected"),
        (AgentEnrollmentResultStatus::PendingReview, "PendingReview"),
    ]);

    assert_enum(&[
        (AgentEnrollmentTokenStatus::Active, "Active"),
        (AgentEnrollmentTokenStatus::Expired, "Expired"),
        (AgentEnrollmentTokenStatus::Revoked, "Revoked"),
        (AgentEnrollmentTokenStatus::Exhausted, "Exhausted"),
    ]);

    assert_enum(&[
        (AgentEnrollmentTokenValidationStatus::Valid, "Valid"),
        (
            AgentEnrollmentTokenValidationStatus::HashMismatch,
            "HashMismatch",
        ),
        (AgentEnrollmentTokenValidationStatus::Expired, "Expired"),
        (AgentEnrollmentTokenValidationStatus::Revoked, "Revoked"),
        (AgentEnrollmentTokenValidationStatus::Exhausted, "Exhausted"),
        (
            AgentEnrollmentTokenValidationStatus::EnvironmentMismatch,
            "EnvironmentMismatch",
        ),
        (
            AgentEnrollmentTokenValidationStatus::HostNotAllowed,
            "HostNotAllowed",
        ),
    ]);

    assert_enum(&[
        (AgentIdentityStatus::Active, "Active"),
        (AgentIdentityStatus::Revoked, "Revoked"),
        (AgentIdentityStatus::Expired, "Expired"),
        (AgentIdentityStatus::RenewalRequired, "RenewalRequired"),
    ]);

    assert_enum(&[
        (
            AgentUpstreamMessageType::EnrollmentRequest,
            "EnrollmentRequest",
        ),
        (AgentUpstreamMessageType::StatusReport, "StatusReport"),
        (AgentUpstreamMessageType::CommandPoll, "CommandPoll"),
        (AgentUpstreamMessageType::ActionResult, "ActionResult"),
    ]);

    assert_enum(&[
        (GatewayIdentityStatus::Active, "Active"),
        (GatewayIdentityStatus::Revoked, "Revoked"),
        (GatewayIdentityStatus::Expired, "Expired"),
        (GatewayIdentityStatus::RenewalRequired, "RenewalRequired"),
    ]);

    assert_enum(&[
        (GatewayInstanceLifecycleState::Provisioned, "Provisioned"),
        (GatewayInstanceLifecycleState::Initializing, "Initializing"),
        (GatewayInstanceLifecycleState::Running, "Running"),
        (GatewayInstanceLifecycleState::Failed, "Failed"),
    ]);
}

#[test]
fn protocol_structs_round_trip() {
    assert_round_trip::<ControlMessageAccepted>(json!({
        "message_id": "msg-1",
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "accepted_at": TS,
    }));

    assert_round_trip::<ControlMessageRejected>(json!({
        "message_id": "msg-1",
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "reason_code": "bad_request",
        "rejected_at": TS,
    }));

    assert_round_trip::<ControlCommandsReturned>(json!({
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "messages": "[]",
        "next_sequence": "1",
        "returned_at": TS,
    }));

    assert_round_trip::<ControlLongPollTimedOut>(json!({
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "wait_ms": "100",
        "timed_out_at": TS,
    }));
}

#[test]
fn agent_enrollment_structs_round_trip() {
    assert_round_trip::<AgentEnrollmentAccepted>(json!({
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "tenant_id": "tenant-1",
        "environment_id": "env-1",
        "node_id": "node-1",
        "accepted_at": TS,
    }));

    assert_round_trip::<AgentEnrollmentAuthProfile>(json!({
        "trust_bundle_required": true,
        "enrollment_token_required": true,
        "credential_request_required": true,
        "server_tls_required": true,
    }));

    assert_round_trip::<AgentEnrollmentRejected>(json!({
        "token_id": "token-1",
        "node_id": "node-1",
        "reason_code": "rejected",
        "rejected_at": TS,
    }));

    assert_round_trip::<AgentEnrollmentTokenAccepted>(json!({
        "token_id": "token-1",
        "tenant_id": "tenant-1",
        "environment_id": "env-1",
        "node_id": "node-1",
        "accepted_at": TS,
    }));

    assert_round_trip::<AgentEnrollmentTokenRejected>(json!({
        "token_id": "token-1",
        "node_id": "node-1",
        "reason_code": "rejected",
        "rejected_at": TS,
    }));
}

#[test]
fn agent_registry_and_status_structs_round_trip() {
    assert_round_trip::<AgentInitialConfig>(json!({
        "gateway_endpoint": "https://gateway.example",
        "schema_version": "v1",
        "telemetry_output": "out",
        "policy_version": "policy-1",
        "mode": "normal",
    }));

    assert_round_trip::<AgentPolicyBinding>(json!({
        "bound_at": TS,
        "policy_id": "policy-1",
        "agent_id": "agent-1",
        "policy_version": "v1",
    }));

    assert_round_trip::<AgentRuntimeStatus>(json!({
        "version": "v1",
        "health": "healthy",
        "instance_id": "inst-1",
        "agent_id": "agent-1",
        "status": "running",
        "last_seen_at": TS,
        "memory_bytes": 512,
        "cpu_percent": 10.5,
        "admin_latency_ms": 3,
    }));

    assert_round_trip::<AgentInstance>(json!({
        "started_at": TS,
        "last_seen_at": TS,
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "version": "v1",
        "boot_id": "boot-1",
    }));

    assert_round_trip::<AgentControlCommandsReturned>(json!({
        "messages": [],
        "next_sequence": 1,
        "agent_id": "agent-1",
        "returned_at": TS,
        "instance_id": "inst-1",
    }));
}

#[test]
fn gateway_structs_round_trip() {
    assert_round_trip::<GatewayStatusAccepted>(json!({
        "gateway_id": "gateway-1",
        "instance_id": "inst-1",
        "accepted_at": TS,
    }));

    assert_round_trip::<GatewayRuntimeStatus>(json!({
        "gateway_id": "gateway-1",
        "instance_id": "inst-1",
        "version": "v1",
        "status": "running",
        "health": "healthy",
        "memory_bytes": 512,
        "cpu_percent": 10.5,
        "last_seen_at": TS,
    }));

    assert_round_trip::<GatewayCustomerBinding>(json!({
        "gateway_id": "gateway-1",
        "customer_id": "customer-1",
        "status": "active",
        "bound_at": TS,
    }));
}
