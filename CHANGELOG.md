# 更新日志

本文件记录 `wist-control` 的所有重要变更。格式遵循 [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)，
版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [0.2.0] latest

与 `warp-insight/jumo/model` 对齐（`jumo-code diff` 差异分类归零）。改名与删除均为**破坏性**变更。

### 变更

- `GateWay`（生成期错拼）统一为 `Gateway`：`GateWayIdentity`、`GateWayIdentityStatus`、`GateWayControlConfig`、
  `GateWayHealth`、`GateWayOverviewView`、`ViewGateWayManagementState`、`WarpGateWayInstance`、
  `WarpGateWayRelease`、`PublishWarpGateWay` 只改拼写；`GateWayManagementStateView` → `GatewayManagementState`。
- 去掉 `Agent` 前缀：`AgentHostProfile` → `HostProfile`，`AgentCredentialBundle` → `CredentialBundle`。
- **HTTP 响应体少一层包装**，直接返回 `binding.mju` 中 `status` 指定的领域类型：

  | 入口 | 原外壳类型 | 现响应类型 |
  |---|---|---|
  | `AdminBindGatewayCustomer` | `AdminGatewayCustomerBindingReturned` | `GatewayCustomerBinding` |
  | `RegisterGateway` | `GatewayEnrollmentResultReturned` | `GatewayEnrollmentResult` |
  | `ReportGatewayStatus` | `GatewayStatusAcceptedReturned` | `GatewayStatusAccepted` |
  | `AdminGetGatewayInitialConfig` | `GatewayInitialConfigReturned` | `GatewayInitialConfig` |
  | `AdminGetAgentInstallCode` | `AdminAgentInstallCodeReturned` | `AgentInstallCode` |
  | `AdminShowAgentRuntimeStatus` | `AdminAgentRuntimeStatusReturned` | `AgentRuntimeStatus` |
  | `AdminPauseAgent` / `AdminUpgradeAgent` | `AdminPauseAgentDispatchReturned` / `AdminUpgradeAgentDispatchReturned` | `DispatchReceipt` |

- 纯注解层面的修正（不影响 API）：`AgentFleetDispatchReceipt` 归属 `Control.Agent.Command`、
  `ViewGatewayList` 归属 `Control.InsightCenterApp.WistCenter`、读投影 `GatewayOverviewView` 不再挂 `jumo` 注解。

### 移除

- 3 个无调用方的响应类型：`AdminGatewayInstanceReturned`、`AdminGlobalPolicyDispatchReturned`、
  `AgentEnrollmentResultReturned`。
- `DispatchGlobalPolicy`：与已有的 `AdminDispatchGlobalPolicy` 完全重复，管理员入口统一用后者。

## [0.1.2] - 2026-09-14

- 本仓首个发布：`Control` 域模型、CI（build / test / clippy）与全模型 serde 往返测试。
