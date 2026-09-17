# 更新日志

本文件记录 `wist-control` 的所有重要变更。格式遵循 [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)，
版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [0.2.0] latest

本次把 crate 与 `warp-insight/jumo/model` 对齐（`jumo-code diff` 的「注解不匹配」与「代码已注解、模型无」
两个分类均已归零）。**改名、删除都是破坏性变更**；其中 8 个响应类型对应的 HTTP 响应体少了一层包装，
服务端消费方需同步（`wist-center`、`wist-gateway`、`wist-gateway-stack/sysrun/re-enroll.sh` 已随本次更新）。

### 变更

- **类型改名**：`GateWay`（中间大写 W）是生成期错拼，模型与拼写习惯都是 `Gateway`。
  - `GateWayIdentity` → `GatewayIdentity`，`GateWayIdentityStatus` → `GatewayIdentityStatus`
  - `GateWayControlConfig` → `GatewayControlConfig`，`GateWayHealth` → `GatewayHealth`
  - `GateWayManagementStateView` → `GatewayManagementState`（模型里就叫这名，label en 为
    "Gateway Management State View"）
  - `GateWayOverviewView` → `GatewayOverviewView`
  - `ViewGateWayManagementState` → `ViewGatewayManagementState`
  - `WarpGateWayInstance` → `WarpGatewayInstance`，`WarpGateWayRelease` → `WarpGatewayRelease`
  - `PublishWarpGateWay` → `PublishWarpGateway`
  - `AgentHostProfile` → `HostProfile`，`AgentCredentialBundle` → `CredentialBundle`
  - 文件名、`mod.rs` 与 `tests/serde_roundtrip.rs` 同步。
- **HTTP 响应体去掉单字段外壳**，直接返回领域类型，与 `binding.mju` 的 `status <领域类型> <code>` 一致：

  | 入口 | 原响应体 | 现响应体 |
  |---|---|---|
  | `AdminBindGatewayCustomer` | `{ "binding": {…} }` | `GatewayCustomerBinding` |
  | `RegisterGateway` | `{ "result": {…} }` | `GatewayEnrollmentResult` |
  | `ReportGatewayStatus` | `{ "receipt": {…} }` | `GatewayStatusAccepted` |
  | `AdminGetGatewayInitialConfig` | `{ "config": {…} }` | `GatewayInitialConfig` |
  | `AdminGetAgentInstallCode` | `{ "install_code": {…} }` | `AgentInstallCode` |
  | `AdminShowAgentRuntimeStatus` | `{ "status": {…} }` | `AgentRuntimeStatus` |
  | `AdminPauseAgent` / `AdminUpgradeAgent` | `{ "result": {…} }` | `DispatchReceipt` |

- 模型归属修正：`AgentFleetDispatchReceipt` → `Control.Agent.Command`（原 `Control.GatewayApp.Application`）；
  `ViewGatewayList` → `Control.InsightCenterApp.WistCenter`（模型模块由 `WarpInsightCenter` 改名）。
- 读投影视图不再挂 `jumo` 注解：`GatewayOverviewView`（模型注明"形状由实现层派生，不建模"）。

### 移除

- 8 个单字段响应外壳类型：`AdminGatewayCustomerBindingReturned`、`GatewayEnrollmentResultReturned`、
  `GatewayStatusAcceptedReturned`、`GatewayInitialConfigReturned`、`AdminAgentInstallCodeReturned`、
  `AdminAgentRuntimeStatusReturned`、`AdminPauseAgentDispatchReturned`、`AdminUpgradeAgentDispatchReturned`
  —— 模型从未有过这些名字，属生成期产物。
- 3 个没有任何调用方的响应类型：`AdminGatewayInstanceReturned`、`AdminGlobalPolicyDispatchReturned`、
  `AgentEnrollmentResultReturned`（对应入口分别使用本地类型、直接返回领域类型、以及 `wist-contracts`
  的 `EnrollmentEnvelope`）。
- `DispatchGlobalPolicy`：与已有的 `AdminDispatchGlobalPolicy` 字段完全重复，且注解挂在其实际不归属的
  `InsightCenterApp.WarpInsightCenter` 模块；管理员入口统一使用 `AdminDispatchGlobalPolicy`。

## [0.1.2] - 2026-09-14

### 新增

- 本仓首个发布（此后到 0.1.2 未再变更版本号）：`Control` 域模型（agent / gateway / insight-center
  三组应用与领域模块，含 actors、binding 与静态域契约）。
- 接入 GitHub Actions（build / test / clippy）、包元数据与由 `version.txt` 驱动的版本号。
- 全模型 serde 往返测试；以及让 `llvm-cov` 能产出覆盖率的 smoke test。
