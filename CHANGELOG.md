# 更新日志

本文件记录 `wist-control` 的所有重要变更。格式遵循 [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)，
版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [0.6.1] - 2026-10-05

与 `wist-design/jumo` 模型对齐：网关状态上报/视图**富化**（机队 / 存储 / 数据面 / 主机资源）。**新增可选字段**，向前兼容
（老网关不带这些键，中心侧 `serde(default)` 兜底）。

### 新增

- `ReportGatewayStatus`：`uptime_seconds`、`agent_count` / `online_agents` / `offline_agents` / `last_seen_lag_seconds`、
  `store_bytes`、`ingest_accepted_total` / `ingest_rejected_total` / `last_ingest_at`、`memory_total_bytes`、
  `load_1m` / `load_5m` / `load_15m`、`disk_usage_percent` / `disk_total_bytes` / `disk_available_bytes`。
- `GatewayRuntimeStatus`：同上一组（供中心视图展示）。

### 说明

- `status` 取值约定为 `online` / `offline`（中心按 `== "online"` 计数与展示；勿再发 `running`）。

## [0.6.0] - 2026-10-04

与 `wist-design/jumo` 模型对齐：补充网关面「升级取指令 + 回执」契约（CR-002 C2）。**新增**，无破坏。

### 新增

- `GetGatewayUpgradePlan`（query）与 `GatewayUpgradePlan`：该网关应升到的目标（来自覆盖它的已批准升级计划）；
  无计划时 `has_plan = false`。
- `ReportGatewayUpgradeResult`（command，字段对齐 `upgrade.json`）与 `GatewayUpgradeResultAccepted`：升级结果回执。

## [0.5.0] - 2026-10-04

与 `wist-design/jumo` 模型对齐：网关「链接上级」入口由 `initial-config` 更名为 `link-upstream`。
**破坏性**变更（请求类型改名、旧路由移除）。

### 变更

- 请求类型 `GetGatewayInitialConfig` → `LinkUpstream`（字段不变：`gateway_id`、`requested_at`）。
- 网关面路由 `GET /api/v1/gateway/initial-config` → `GET /api/v1/gateway/link-upstream`（旧路径**移除**，
  不留别名；消费方请同步升级）。

## [0.4.0] - 2026-10-04

与 `wist-design/jumo` 模型对齐（身份统一）：网关面 / 初始配置的查询键统一为 `gateway_id`，
`instance_id` 只保留给置备域，不再承载 gateway_id。**破坏性**变更。

### 变更

- `QueryGatewayInitializationStatus`：`instance_id: Option<String>` → `gateway_id: String`。
- `GetGatewayInitialConfig`：`instance_id: String` → `gateway_id: String`。

对应 HTTP 查询参数由 `?instance_id=` 改为 `?gateway_id=`（`wist-center` 已同步，升级本组件后请同步升级依赖方）。

## [0.3.0] - 2026-09-29

### 移除

- **破坏性**：移除 `AdminPauseAgent`、`AdminUpgradeAgent` 两个管理员命令及其回执类型 `DispatchReceipt`。
  agent 的暂停 / 升级改由 `DispatchAgentFleetCommand`（按 `command_kind` 区分动作）统一承载，调用方请改用
  该入口及其回执 `AgentFleetDispatchReceipt`。

## [0.2.0]

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
