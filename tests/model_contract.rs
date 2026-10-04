//! 模型契约与边界测试（面向已发布的 `wist-control` 公共 API）。
//!
//! 与 `serde_roundtrip.rs` 的分工：那里只对少量代表类型做冒烟往返；这里
//! **补齐其余全部公开类型的 serde 往返**，并锁定边界与异常输入的行为——
//! 空值/极值、缺字段、`null`、未知字段、未知枚举变体。这些是下游
//! （`wist-center` / `wist-gateway` 等）依赖的线上 JSON 契约，一旦改变即破坏兼容。
//!
//! 约束：本文件只读地使用模型类型，**不改动任何 jumo 生成类型**；4 个手写补充类型
//! （`GatewayListView` 与 3 个 `*Returned` 外壳）的 JSON 形状也在此固化。

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use wist_control::{
    AdminBindGatewayCustomer,
    AdminCreateGatewayInstance,
    AdminDispatchGlobalPolicy,
    AdminGatewayListReturned,
    AdminGatewayStatusListReturned,
    AdminGatewayStatusReturned,
    AdminGetAgentInstallCode,
    AdminListGatewayStatus,
    AdminQueryHostInventory,
    AdminShowAgentRuntimeStatus,
    AdminShowGatewayStatus,
    AdminViewEffectiveResponsibility,
    AdminViewGatewayList,
    AdminViewHostRuntimeState,
    AdminViewNetworkTopology,
    AdminViewServiceTopology,
    AdminViewSoftwareVulnerabilities,
    AgentBootstrapBundle,
    AgentControlAuthProfile,
    AgentControlCommand,
    AgentControlCommandsReturned,
    AgentCredentialIssued,
    AgentCredentialRejected,
    AgentCredentialRevoked,
    AgentCredentialVerificationResult,
    // 枚举
    AgentCredentialVerificationStatus,
    AgentCredentialVerified,
    AgentDownstreamMessageType,
    AgentEnrollmentResult,
    AgentEnrollmentResultStatus,
    AgentEnrollmentToken,
    AgentEnrollmentTokenStatus,
    AgentEnrollmentTokenValidation,
    AgentEnrollmentTokenValidationStatus,
    AgentFleetDispatchReceipt,
    AgentIdentity,
    AgentIdentityStatus,
    AgentInitialConfig,
    AgentInstallCode,
    AgentPolicyBinding,
    AgentRuntimeStatus,
    AgentUpstreamMessageType,
    ApproveUpgradePlan,
    ControlCenterTrustBundle,
    CreateUpgradePlan,
    CredentialBundle,
    DispatchAgentFleetCommand,
    DuplicateRegistrationDetected,
    GatewayControlConfig,
    GatewayCustomerBinding,
    GatewayHealth,
    GatewayIdentity,
    GatewayIdentityStatus,
    GatewayInitialConfig,
    GatewayInitializationStatus,
    GatewayInstance,
    GatewayInstanceLifecycleState,
    GatewayListView,
    GatewayManagementState,
    GatewayOverviewView,
    GatewayRuntimeStatus,
    GatewayStatusAccepted,
    GlobalPolicyDispatch,
    HostProfile,
    InitializeGatewayViaUrl,
    LinkUpstream,
    ManagementEndpointTrustBundle,
    PollControlCommands,
    PublishWarpGateway,
    PublishWistAgentd,
    QueryGatewayInitializationStatus,
    ReportGatewayStatus,
    SubmitEnrollmentRequest,
    UpgradePlan,
    UpgradePlanApproval,
    UpgradeStep,
    UpgradeTarget,
    ViewGatewayList,
    ViewGatewayManagementState,
    WarpGatewayInstance,
    WarpGatewayRelease,
    WistAgentdRelease,
};

const TS: &str = "2026-09-27T12:00:00Z";

/// 往返并断言序列化稳定（再序列化结果与首次一致）。
fn rt<T: Serialize + DeserializeOwned>(value: Value) -> Value {
    let decoded: T = serde_json::from_value(value).expect("deserialize");
    let first = serde_json::to_value(&decoded).expect("serialize");
    let redecoded: T = serde_json::from_value(first.clone()).expect("re-deserialize");
    let second = serde_json::to_value(&redecoded).expect("re-serialize");
    assert_eq!(first, second, "serde round-trip should be stable");
    first
}

fn rt_ok<T: Serialize + DeserializeOwned>(value: Value) {
    rt::<T>(value);
}

// ── 可复用的取样片段 ──────────────────────────────────────────────

fn gw_runtime_status() -> Value {
    json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "version": "0.3.0",
        "status": "online",
        "health": "healthy",
        "memory_bytes": 536_870_912,
        "cpu_percent": 12.5,
        "last_seen_at": TS,
    })
}

fn host_profile() -> Value {
    json!({
        "cloud_instance_id": "i-abc",
        "node_id": "node-1",
        "hostname": "host-1",
        "os": "linux",
        "machine_id": "mid-1",
        "arch": "x86_64",
        "k8s_node_uid": "uid-1",
        "ip_addresses": ["10.0.0.1", "10.0.0.2"],
    })
}

fn agent_identity() -> Value {
    json!({
        "tenant_id": "t-1",
        "expires_at": TS,
        "status": "Active",
        "instance_id": "inst-1",
        "environment_id": "env-1",
        "agent_id": "agent-1",
        "issued_at": TS,
        "node_id": "node-1",
    })
}

// ── 1. 手写补充类型（非生成）的 JSON 形状 ─────────────────────────

#[test]
fn hand_written_view_types_round_trip() {
    rt_ok::<GatewayListView>(json!({
        "gateway_count": 6,
        "online_count": 4,
        "degraded_count": 1,
        "offline_count": 2,
        "updated_at": TS,
    }));

    rt_ok::<AdminGatewayListReturned>(json!({
        "list": {
            "gateway_count": 6,
            "online_count": 4,
            "degraded_count": 1,
            "offline_count": 2,
            "updated_at": TS,
        },
    }));

    rt_ok::<AdminGatewayStatusReturned>(json!({ "status": gw_runtime_status() }));

    rt_ok::<AdminGatewayStatusListReturned>(json!({
        "statuses": [gw_runtime_status()],
    }));
}

// ── 2. 网关域生成类型的往返 ──────────────────────────────────────

#[test]
fn gateway_domain_types_round_trip() {
    rt_ok::<GatewayIdentity>(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "tenant_id": "t-1",
        "environment_id": "env-1",
        "node_id": "node-1",
        "issued_at": TS,
        "expires_at": TS,
        "status": "Active",
    }));

    rt_ok::<WarpGatewayInstance>(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "boot_id": "boot-1",
        "version": "0.3.0",
        "started_at": TS,
        "last_seen_at": TS,
    }));

    rt_ok::<GatewayControlConfig>(json!({
        "config_id": "cfg-1",
        "gateway_id": "gw-1",
        "advertise_url": "https://gw.example",
        "enrollment_url": "https://gw.example/enroll",
        "gateway_url": "https://gw.example",
        "telemetry_url": "https://gw.example/telemetry",
        "updated_at": TS,
    }));

    let health = json!({ "gateway_id": "gw-1", "status": "healthy", "reported_at": TS });
    rt_ok::<GatewayHealth>(health.clone());

    rt_ok::<GatewayManagementState>(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "version": "0.3.0",
        "status": "online",
        "config": {
            "config_id": "cfg-1",
            "gateway_id": "gw-1",
            "advertise_url": "https://gw.example",
            "enrollment_url": "https://gw.example/enroll",
            "gateway_url": "https://gw.example",
            "telemetry_url": "https://gw.example/telemetry",
            "updated_at": TS,
        },
        "health": health,
        "last_seen_at": TS,
    }));

    rt_ok::<GatewayOverviewView>(json!({
        "gateway_id": "gw-1",
        "agent_total": 10,
        "agent_online": 8,
        "agent_unhealthy": 1,
        "status": "online",
        "updated_at": TS,
    }));

    rt_ok::<ViewGatewayManagementState>(json!({ "requested_by": "admin-1" }));

    rt_ok::<ControlCenterTrustBundle>(json!({
        "trust_bundle_id": "trust-1",
        "control_endpoint": "https://center.example",
        "ca_bundle": "control-center.pem",
        "server_name": "center.example",
        "expected_san": "center.example",
        "issued_at": TS,
        "expires_at": TS,
    }));

    rt_ok::<GatewayInitialConfig>(json!({
        "gateway_id": "gw-1",
        "control_center_endpoint": "https://center.example",
        "trust_bundle": {
            "trust_bundle_id": "trust-1",
            "control_endpoint": "https://center.example",
            "ca_bundle": "control-center.pem",
            "server_name": "center.example",
            "expected_san": "center.example",
            "issued_at": TS,
            "expires_at": TS,
        },
        "server_tls_required": true,
        "protocol_version": "v1",
        "enrollment_token_id": "tok-1",
    }));

    let instance = json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "lifecycle_state": "Running",
        "created_at": TS,
        "initialized_at": TS,
    });
    rt_ok::<GatewayInstance>(instance.clone());
    // initialized_at 允许缺省（尚未初始化）。
    rt_ok::<GatewayInstance>(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "lifecycle_state": "Provisioned",
        "created_at": TS,
    }));

    rt_ok::<GlobalPolicyDispatch>(json!({
        "dispatch_id": "disp-1",
        "policy_version": "v1",
        "target_count": 3,
        "status": "accepted",
        "dispatched_at": TS,
    }));

    rt_ok::<GatewayStatusAccepted>(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "accepted_at": TS,
    }));

    rt_ok::<GatewayCustomerBinding>(json!({
        "gateway_id": "gw-1",
        "customer_id": "cust-1",
        "status": "active",
        "bound_at": TS,
    }));

    rt_ok::<LinkUpstream>(json!({ "gateway_id": "gw-1", "requested_at": TS }));
    rt_ok::<InitializeGatewayViaUrl>(json!({ "init_url": "https://init", "requested_at": TS }));
    rt_ok::<ReportGatewayStatus>(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "version": "0.3.0",
        "status": "online",
        "health": "healthy",
        "memory_bytes": 1024,
        "cpu_percent": 1.5,
        "reported_at": TS,
    }));
}

// ── 3. Agent 域生成类型的往返 ────────────────────────────────────

#[test]
fn agent_domain_types_round_trip() {
    rt_ok::<HostProfile>(host_profile());

    rt_ok::<AgentControlAuthProfile>(json!({
        "enrollment_token_rejected": true,
        "mtls_client_certificate_allowed": false,
        "agent_credential_required": true,
        "server_tls_required": true,
    }));

    rt_ok::<CredentialBundle>(json!({
        "not_before": TS,
        "not_after": TS,
        "certificate": "cert-pem",
        "auth_scheme": "bearer",
        "bearer_token": "agent-token",
        "ca_bundle": "ca.pem",
        "instance_id": "inst-1",
        "issued_at": TS,
        "credential_id": "cred-1",
        "private_key_ref": "file:///etc/agent.key",
        "agent_id": "agent-1",
    }));

    rt_ok::<AgentIdentity>(agent_identity());

    rt_ok::<AgentControlCommand>(json!({
        "requested_by": "admin-1",
        "command_kind": "refresh-policy",
        "issued_at": TS,
        "target_version": "v2",
        "sequence": 7,
        "expires_at": TS,
        "command_id": "cmd-1",
        "payload": "{}",
        "agent_id": "agent-1",
    }));

    rt_ok::<AgentBootstrapBundle>(json!({
        "expires_at": TS,
        "environment_id": "env-1",
        "control_endpoint": "https://center.example",
        "tenant_id": "t-1",
        "bundle_id": "bundle-1",
        "install_script_url": "https://dl/install.sh",
        "agent_package_url": "https://dl/agent.tar.gz",
        "agent_package_sha256": "deadbeef",
        "trust_bundle": "trust-bundle",
    }));

    rt_ok::<AgentInstallCode>(json!({
        "x86_linux_install_code": "curl x86",
        "bootstrap_enrollment_token": "boot-token",
        "bootstrap_bundle": {
            "expires_at": TS,
            "environment_id": "env-1",
            "control_endpoint": "https://center.example",
            "tenant_id": "t-1",
            "bundle_id": "bundle-1",
            "install_script_url": "https://dl/install.sh",
            "agent_package_url": "https://dl/agent.tar.gz",
            "agent_package_sha256": "deadbeef",
            "trust_bundle": "trust-bundle",
        },
        "arm_linux_install_code": "curl arm",
        "macos_install_code": "curl macos",
    }));

    rt_ok::<ManagementEndpointTrustBundle>(json!({
        "control_endpoint": "https://center.example",
        "expires_at": TS,
        "trust_bundle_id": "trust-1",
        "ca_bundle": "ca.pem",
        "expected_san": "center.example",
        "issued_at": TS,
        "server_name": "center.example",
    }));

    rt_ok::<AgentEnrollmentToken>(json!({
        "revoked_at": TS,
        "allowed_node_selector": "nodename=host-1",
        "tenant_id": "t-1",
        "environment_id": "env-1",
        "issued_at": TS,
        "used_count": 0,
        "issued_by": "admin-1",
        "token_id": "tok-1",
        "max_uses": 1,
        "expires_at": TS,
        "token_hash": "sha256:...",
        "status": "Active",
    }));

    rt_ok::<AgentEnrollmentTokenValidation>(json!({
        "host_profile": host_profile(),
        "validated_at": TS,
        "status": "Valid",
        "tenant_id": "t-1",
        "environment_id": "env-1",
        "token_id": "tok-1",
        "reason_code": "ok",
    }));

    rt_ok::<DuplicateRegistrationDetected>(json!({
        "node_id": "node-1",
        "existing_agent_id": "agent-1",
        "candidate_instance_id": "inst-2",
        "action": "reject",
        "detected_at": TS,
    }));

    rt_ok::<AgentCredentialIssued>(json!({
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "credential": "issued",
        "issued_at": TS,
    }));
    rt_ok::<AgentCredentialRejected>(json!({
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "reason_code": "bad",
        "rejected_at": TS,
    }));
    rt_ok::<AgentCredentialRevoked>(json!({
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "reason_code": "revoked",
        "revoked_at": TS,
    }));
    rt_ok::<AgentCredentialVerified>(json!({
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "verified_at": TS,
    }));
    rt_ok::<AgentCredentialVerificationResult>(json!({
        "agent_id": "agent-1",
        "instance_id": "inst-1",
        "reason_code": "ok",
        "status": "Verified",
        "verified_at": TS,
    }));

    rt_ok::<AgentEnrollmentResult>(json!({
        "credential_bundle": {
            "not_before": TS,
            "not_after": TS,
            "certificate": "cert-pem",
            "auth_scheme": "bearer",
            "bearer_token": "agent-token",
            "ca_bundle": "ca.pem",
            "instance_id": "inst-1",
            "issued_at": TS,
            "credential_id": "cred-1",
            "private_key_ref": "file:///etc/agent.key",
            "agent_id": "agent-1",
        },
        "initial_config": {
            "gateway_endpoint": "https://gw.example",
            "schema_version": "v1",
            "telemetry_output": "out",
            "policy_version": "policy-1",
            "mode": "normal",
        },
        "reason_code": "ok",
        "instance_id": "inst-1",
        "agent_id": "agent-1",
        "policy_binding": {
            "bound_at": TS,
            "policy_id": "policy-1",
            "agent_id": "agent-1",
            "policy_version": "v1",
        },
        "status": "Accepted",
        "issued_identity": agent_identity(),
    }));

    rt_ok::<AgentInitialConfig>(json!({
        "gateway_endpoint": "https://gw.example",
        "schema_version": "v1",
        "telemetry_output": "out",
        "policy_version": "policy-1",
        "mode": "normal",
    }));

    rt_ok::<AgentPolicyBinding>(json!({
        "bound_at": TS,
        "policy_id": "policy-1",
        "agent_id": "agent-1",
        "policy_version": "v1",
    }));

    rt_ok::<AgentRuntimeStatus>(json!({
        "version": "0.3.0",
        "health": "healthy",
        "instance_id": "inst-1",
        "agent_id": "agent-1",
        "status": "running",
        "last_seen_at": TS,
        "memory_bytes": 512,
        "cpu_percent": 10.5,
        "admin_latency_ms": 3,
    }));
}

// ── 4. agent_app / 平台发布 / 管理台消息类型 ─────────────────────

#[test]
fn app_and_admin_message_types_round_trip() {
    rt_ok::<AgentControlCommandsReturned>(json!({
        "messages": [{
            "requested_by": "admin-1",
            "command_kind": "refresh-policy",
            "issued_at": TS,
            "target_version": "v2",
            "sequence": 1,
            "expires_at": TS,
            "command_id": "cmd-1",
            "payload": "{}",
            "agent_id": "agent-1",
        }],
        "next_sequence": 2,
        "agent_id": "agent-1",
        "returned_at": TS,
        "instance_id": "inst-1",
    }));

    rt_ok::<PollControlCommands>(json!({
        "requested_at": TS,
        "last_seen_sequence": 1,
        "wait_ms": 30_000,
        "agent_id": "agent-1",
        "instance_id": "inst-1",
    }));

    rt_ok::<SubmitEnrollmentRequest>(json!({
        "capability_summary": "caps",
        "token": "token",
        "requested_at": TS,
        "host_profile": host_profile(),
        "credential_request": "req",
    }));

    rt_ok::<AdminGetAgentInstallCode>(json!({}));
    rt_ok::<AdminShowAgentRuntimeStatus>(json!({
        "requested_by": "admin-1",
        "agent_id": "agent-1",
    }));
    rt_ok::<AgentFleetDispatchReceipt>(json!({
        "dispatch_id": "disp-1",
        "command_kind": "upgrade",
        "target_count": 2,
        "status": "accepted",
        "created_at": TS,
    }));
    rt_ok::<DispatchAgentFleetCommand>(json!({
        "command_kind": "upgrade",
        "agent_ids": ["agent-1", "agent-2"],
        "requested_by": "admin-1",
    }));
    rt_ok::<GatewayInitializationStatus>(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "lifecycle_state": "Initializing",
        "initialized": false,
    }));
    rt_ok::<QueryGatewayInitializationStatus>(json!({ "gateway_id": "gw-1" }));

    rt_ok::<AdminBindGatewayCustomer>(json!({
        "gateway_id": "gw-1",
        "customer_id": "cust-1",
        "requested_by": "admin-1",
    }));
    rt_ok::<AdminCreateGatewayInstance>(json!({
        "gateway_name": "gw-1",
        "requested_by": "admin-1",
    }));
    rt_ok::<AdminDispatchGlobalPolicy>(json!({
        "policy_version": "v1",
        "gateway_ids": ["gw-1", "gw-2"],
        "requested_by": "admin-1",
    }));
    rt_ok::<AdminListGatewayStatus>(json!({ "requested_by": "admin-1" }));
    rt_ok::<AdminQueryHostInventory>(json!({
        "tenant_id": "t-1",
        "environment_id": "env-1",
        "host_name": "host-1",
        "requested_by": "admin-1",
    }));
    rt_ok::<AdminShowGatewayStatus>(json!({
        "gateway_id": "gw-1",
        "requested_by": "admin-1",
    }));
    rt_ok::<AdminViewEffectiveResponsibility>(json!({
        "host_id": "host-1",
        "requested_by": "admin-1",
    }));
    rt_ok::<AdminViewGatewayList>(json!({ "requested_by": "admin-1" }));
    rt_ok::<AdminViewHostRuntimeState>(json!({
        "host_id": "host-1",
        "requested_by": "admin-1",
    }));
    rt_ok::<AdminViewNetworkTopology>(json!({ "requested_by": "admin-1" }));
    rt_ok::<AdminViewServiceTopology>(json!({
        "business_id": "biz-1",
        "service_id": "svc-1",
        "requested_by": "admin-1",
    }));
    rt_ok::<AdminViewSoftwareVulnerabilities>(json!({
        "software_id": "sw-1",
        "host_id": "host-1",
        "requested_by": "admin-1",
    }));
    rt_ok::<ViewGatewayList>(json!({ "requested_by": "admin-1" }));

    rt_ok::<ApproveUpgradePlan>(json!({
        "plan_id": "plan-1",
        "approved_by": "admin-1",
        "approved_at": TS,
    }));
    rt_ok::<CreateUpgradePlan>(json!({
        "targets": [{ "component": "agentd", "target_version": "v2" }],
        "gateway_ids": ["gw-1"],
        "steps": [{ "step_index": 0, "gateway_ids": ["gw-1"], "status": "pending" }],
        "requested_by": "admin-1",
        "requested_at": TS,
    }));
    rt_ok::<UpgradePlan>(json!({
        "plan_id": "plan-1",
        "targets": [{ "component": "agentd", "target_version": "v2" }],
        "target_count": 1,
        "status": "pending",
        "created_at": TS,
        "steps": [{ "step_index": 0, "gateway_ids": ["gw-1"], "status": "pending" }],
    }));
    rt_ok::<UpgradePlanApproval>(json!({
        "plan_id": "plan-1",
        "status": "approved",
        "approved_by": "admin-1",
        "approved_at": TS,
    }));
    rt_ok::<UpgradeStep>(json!({
        "step_index": 0,
        "gateway_ids": ["gw-1"],
        "status": "pending",
    }));
    rt_ok::<UpgradeTarget>(json!({ "component": "agentd", "target_version": "v2" }));
    rt_ok::<PublishWarpGateway>(json!({
        "version": "v2",
        "artifact_url": "https://dl/gw",
        "requested_by": "admin-1",
        "requested_at": TS,
    }));
    rt_ok::<PublishWistAgentd>(json!({
        "version": "v2",
        "artifact_url": "https://dl/agentd",
        "requested_by": "admin-1",
        "requested_at": TS,
    }));
    rt_ok::<WarpGatewayRelease>(json!({
        "version": "v2",
        "artifact_url": "https://dl/gw",
        "status": "published",
        "published_at": TS,
    }));
    rt_ok::<WistAgentdRelease>(json!({
        "version": "v2",
        "artifact_url": "https://dl/agentd",
        "status": "published",
        "published_at": TS,
    }));
}

// ── 5. 边界：空集合 / 空字符串 / 数值极值 ────────────────────────

#[test]
fn empty_collections_and_strings_round_trip() {
    // 空列表是合法状态（无网关/无 Agent/无步骤），必须与「请求失败」可区分。
    rt_ok::<AdminGatewayStatusListReturned>(json!({ "statuses": [] }));
    rt_ok::<HostProfile>(json!({
        "cloud_instance_id": "",
        "node_id": "node-1",
        "hostname": "",
        "os": "",
        "machine_id": "",
        "arch": "",
        "k8s_node_uid": "",
        "ip_addresses": [],
    }));
    rt_ok::<DispatchAgentFleetCommand>(json!({
        "command_kind": "",
        "agent_ids": [],
        "requested_by": "",
    }));
    rt_ok::<CreateUpgradePlan>(json!({
        "targets": [],
        "gateway_ids": [],
        "steps": [],
        "requested_by": "",
        "requested_at": TS,
    }));
    rt_ok::<UpgradePlan>(json!({
        "plan_id": "",
        "targets": [],
        "target_count": 0,
        "status": "",
        "created_at": TS,
        "steps": [],
    }));
    rt_ok::<UpgradeStep>(json!({
        "step_index": 0,
        "gateway_ids": [],
        "status": "",
    }));
    rt_ok::<AgentControlCommandsReturned>(json!({
        "messages": [],
        "next_sequence": 0,
        "agent_id": "",
        "returned_at": TS,
        "instance_id": "",
    }));

    // 纯 DTO、无校验：全空字符串被原样接受。
    rt_ok::<AdminViewGatewayList>(json!({ "requested_by": "" }));
}

#[test]
fn numeric_boundaries_round_trip() {
    let encoded = rt::<GatewayListView>(json!({
        "gateway_count": i64::MAX,
        "online_count": i64::MIN,
        "degraded_count": 0,
        "offline_count": -1,
        "updated_at": TS,
    }));
    assert_eq!(encoded["gateway_count"], json!(i64::MAX));
    assert_eq!(encoded["online_count"], json!(i64::MIN));

    let encoded = rt::<AgentRuntimeStatus>(json!({
        "version": "v",
        "health": "h",
        "instance_id": "i",
        "agent_id": "a",
        "status": "s",
        "last_seen_at": TS,
        "memory_bytes": i64::MIN,
        "cpu_percent": -1.5e300,
        "admin_latency_ms": i64::MAX,
    }));
    assert_eq!(encoded["memory_bytes"], json!(i64::MIN));
    assert_eq!(encoded["admin_latency_ms"], json!(i64::MAX));

    // f64 极值（有限）：不应丢失或溢出。
    let encoded = rt::<GatewayRuntimeStatus>(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "version": "v",
        "status": "s",
        "health": "h",
        "memory_bytes": 0,
        "cpu_percent": f64::MAX,
        "last_seen_at": TS,
    }));
    assert_eq!(encoded["cpu_percent"], json!(f64::MAX));
}

// ── 6. 契约：缺字段 / null / 未知字段 / 未知枚举变体 ─────────────

#[test]
fn optional_fields_missing_or_null_yield_none() {
    // 注意：`version`/`status`/`health` 在代码里是必填 `String`（模型标为可选，见 review 报告），
    // 这里只验证真正的可选字段 `memory_bytes`/`cpu_percent`。
    let decoded: GatewayRuntimeStatus = serde_json::from_value(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "version": "v1",
        "status": "online",
        "health": "healthy",
        "last_seen_at": TS,
    }))
    .expect("可选字段缺省应可反序列化");
    assert_eq!(decoded.memory_bytes, None);
    assert_eq!(decoded.cpu_percent, None);

    let decoded: GatewayRuntimeStatus = serde_json::from_value(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "version": "v1",
        "status": "online",
        "health": "healthy",
        "memory_bytes": null,
        "cpu_percent": null,
        "last_seen_at": TS,
    }))
    .expect("显式 null 应映射为 None");
    assert_eq!(decoded.memory_bytes, None);
    assert_eq!(decoded.cpu_percent, None);

    let decoded: GatewayInitialConfig = serde_json::from_value(json!({
        "gateway_id": "gw-1",
        "control_center_endpoint": "https://center",
        "server_tls_required": true,
        "protocol_version": "v1",
        "enrollment_token_id": "tok-1",
    }))
    .expect("trust_bundle 缺省应可反序列化");
    assert!(decoded.trust_bundle.is_none());

    let decoded: GatewayInstance = serde_json::from_value(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "lifecycle_state": "Provisioned",
        "created_at": TS,
    }))
    .expect("initialized_at 缺省应可反序列化");
    assert!(decoded.initialized_at.is_none());

    let decoded: ControlCenterTrustBundle = serde_json::from_value(json!({
        "trust_bundle_id": "trust-1",
        "control_endpoint": "https://center",
        "ca_bundle": "ca.pem",
        "server_name": "center",
        "expected_san": "center",
    }))
    .expect("issued_at/expires_at 缺省应可反序列化");
    assert!(decoded.issued_at.is_none());
    assert!(decoded.expires_at.is_none());

    let decoded: AdminQueryHostInventory = serde_json::from_value(json!({
        "requested_by": "admin-1",
    }))
    .expect("三个过滤条件缺省应可反序列化");
    assert!(decoded.tenant_id.is_none());
    assert!(decoded.environment_id.is_none());
    assert!(decoded.host_name.is_none());

    let decoded: GatewayInitializationStatus = serde_json::from_value(json!({
        "gateway_id": "gw-1",
        "lifecycle_state": "Provisioned",
        "initialized": false,
    }))
    .expect("instance_id 缺省应可反序列化");
    assert!(decoded.instance_id.is_none());
}

#[test]
fn missing_required_field_is_rejected() {
    // 缺 gateway_count：非 Option 字段，必须报错（不能被默认成 0）。
    assert!(
        serde_json::from_value::<GatewayListView>(json!({
            "online_count": 1,
            "degraded_count": 0,
            "offline_count": 0,
            "updated_at": TS,
        }))
        .is_err(),
        "缺少必填字段 gateway_count 应报错"
    );
    assert!(
        serde_json::from_value::<GatewayRuntimeStatus>(json!({
            "gateway_id": "gw-1",
            "instance_id": "inst-1",
            "version": "v",
            "status": "s",
            "health": "h",
        }))
        .is_err(),
        "缺少必填字段 last_seen_at 应报错"
    );
    assert!(
        serde_json::from_value::<UpgradeStep>(json!({
            "gateway_ids": [],
            "status": "pending",
        }))
        .is_err(),
        "缺少必填字段 step_index 应报错"
    );
}

#[test]
fn null_for_required_field_is_rejected() {
    assert!(
        serde_json::from_value::<GatewayListView>(json!({
            "gateway_count": null,
            "online_count": 1,
            "degraded_count": 0,
            "offline_count": 0,
            "updated_at": TS,
        }))
        .is_err(),
        "必填字段为 null 应报错"
    );
    assert!(
        serde_json::from_value::<GatewayInstance>(json!({
            "gateway_id": "gw-1",
            "instance_id": "inst-1",
            "lifecycle_state": null,
            "created_at": TS,
        }))
        .is_err(),
        "枚举字段为 null 应报错"
    );
    assert!(
        serde_json::from_value::<ControlCenterTrustBundle>(json!({
            "trust_bundle_id": "trust-1",
            "control_endpoint": null,
            "ca_bundle": "ca.pem",
            "server_name": "center",
            "expected_san": "center",
        }))
        .is_err(),
        "必填 String 为 null 应报错"
    );
}

#[test]
fn unknown_fields_are_ignored() {
    // 默认 serde 行为：忽略未知字段（前向兼容——新增字段不会打挂旧消费者）。
    let encoded = rt::<GatewayStatusAccepted>(json!({
        "gateway_id": "gw-1",
        "instance_id": "inst-1",
        "accepted_at": TS,
        "future_field": "ignored",
    }));
    assert!(
        encoded.get("future_field").is_none(),
        "未知字段不应出现在重新序列化结果中"
    );

    let encoded = rt::<GatewayListView>(json!({
        "gateway_count": 1,
        "online_count": 1,
        "degraded_count": 0,
        "offline_count": 0,
        "updated_at": TS,
        "extra": 42,
    }));
    assert!(encoded.get("extra").is_none());
}

#[test]
fn unknown_enum_variant_is_rejected() {
    assert!(
        serde_json::from_value::<AgentDownstreamMessageType>(json!("NotAVariant")).is_err(),
        "未知下游消息类型应报错"
    );
    assert!(
        serde_json::from_value::<GatewayInstanceLifecycleState>(json!("Unknown")).is_err(),
        "未知生命周期状态应报错"
    );
    // 变体名大小写敏感。
    assert!(
        serde_json::from_value::<AgentIdentityStatus>(json!("active")).is_err(),
        "枚举变体名应大小写敏感"
    );
    // 合法变体照常通过。
    assert_eq!(
        serde_json::from_value::<AgentIdentityStatus>(json!("RenewalRequired")).expect("合法变体"),
        AgentIdentityStatus::RenewalRequired
    );
}

#[test]
fn enum_wire_names_are_exact() {
    let cases: &[(Value, &str)] = &[
        (
            serde_json::to_value(AgentDownstreamMessageType::PolicyRefreshHint).unwrap(),
            "PolicyRefreshHint",
        ),
        (
            serde_json::to_value(AgentUpstreamMessageType::ActionResult).unwrap(),
            "ActionResult",
        ),
        (
            serde_json::to_value(AgentEnrollmentTokenStatus::Exhausted).unwrap(),
            "Exhausted",
        ),
        (
            serde_json::to_value(AgentEnrollmentTokenValidationStatus::HostNotAllowed).unwrap(),
            "HostNotAllowed",
        ),
        (
            serde_json::to_value(AgentCredentialVerificationStatus::CredentialMismatch).unwrap(),
            "CredentialMismatch",
        ),
        (
            serde_json::to_value(AgentEnrollmentResultStatus::PendingReview).unwrap(),
            "PendingReview",
        ),
        (
            serde_json::to_value(GatewayIdentityStatus::RenewalRequired).unwrap(),
            "RenewalRequired",
        ),
        (
            serde_json::to_value(GatewayInstanceLifecycleState::Failed).unwrap(),
            "Failed",
        ),
    ];
    for (encoded, expected) in cases {
        assert_eq!(encoded, &json!(expected), "枚举线上名应与变体名一致");
    }
}

#[test]
fn hand_written_types_derive_equality_and_clone() {
    let value: GatewayListView = serde_json::from_value(json!({
        "gateway_count": 1,
        "online_count": 1,
        "degraded_count": 0,
        "offline_count": 0,
        "updated_at": TS,
    }))
    .expect("deserialize");
    let cloned = value.clone();
    assert_eq!(value, cloned, "Clone 后应相等");
    assert!(!format!("{value:?}").is_empty(), "应实现 Debug");
}
