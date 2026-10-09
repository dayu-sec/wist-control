# 更新日志

本文件记录 `wist-control` 的所有重要变更。格式遵循 [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)，
版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)。

## [0.13.0] - 2026-10-09

### 新增

- **`GatewayUpgradePlan` 增 `artifacts`（多平台制品清单）**：`Control.Gateway.Supervision` 加
  `artifacts: List<GatewayUpgradeArtifact>`（新结构 `GatewayUpgradeArtifact { platform, artifact_url,
  artifact_sha256 }`；空清单序列化时省略）。「agent 包下发」（发布 ②）要网关替 **Agent 机队**托管各平台的
  `wist-agentd` 包（机队平台可能 ≠ 网关自己主机的平台），单值 `artifact_url` 无法描述；① 升级仍用单值
  （网关本机就一个平台）。见设计 `edge/agent-package-push-to-gateways.md`。

## [0.12.0] - 2026-10-09

### 新增

- **升级目标可带计划动作与制品摘要**：`GatewayUpgradePlan` 增可选字段 **`action`**（`upgrade` /
  `push-agent-package`，缺省 = 老中心按 `upgrade` 处理）与 **`artifact_sha256`**（`artifact_url` 的期望
  摘要；**仅**「agent 包下发」带它，① 升级路径为空）。gwlinkd 据此分派执行器：升级走 gops，包下发环回写
  网关包管理。见设计 `edge/agent-package-push-to-gateways.md`（发布 ②）。
- **计划动作常量** `ACTION_UPGRADE` / `ACTION_PUSH_AGENT_PACKAGE`（`Control.Rollout`）：中心与
  `wist-gwlinkd` **共用一份**动作字面量，避免两处漂移。

## [0.11.0] - 2026-10-08

### 变更（不兼容）

- **`AgentBootstrapBundle` 改为按平台托管安装包**：以单值 `install_script_url` / `agent_package_url` /
  `agent_package_sha256` 无法描述多平台 `wist-agentd`（macOS-ARM + Linux x86_64/ARM64 三平台），
  一个网关只能给出一个平台的包。现改为 `platforms: List<AgentPlatformPackage>`，每个平台一份
  （`platform` = target-triple + `install_script_url` / `agent_package_url` / `agent_package_sha256`）。
  新增结构 `Control.Agent.Enrollment.AgentPlatformPackage`。

## [0.10.0] - 2026-10-07

### 新增

- **灰度发布计划的管理入参收进共享模块** `Control.RolloutApp.AdminInterface`：`CreateRolloutPlan` /
  `ListRolloutPlans` / `ApproveRolloutPlan` / `AdvanceRolloutPlan` / `ViewRolloutPlan` —— **中心与网关
  共用同一份**（各 app 的入口用 `input` 绑过去，入参形状只定义一次）。计划本体是 `Control.Rollout`
  （`RolloutPlan` / `RolloutPhase` / `RolloutPlanEntry` / `RolloutPlanView`）。

### 变更（不兼容）

- 删除无模型对应的死骨架 `Control.InsightCenter.PlatformRelease.ApproveUpgradePlan` /
  `AdvanceUpgradePlan`（批准/推进已由 `Control.ApproveRolloutPlan` / `Control.AdvanceRolloutPlan` 取代）。

## [0.9.0] - 2026-10-06

### 变更

- **网关「对外域名」进入接入报文**（`Control.Gateway.Supervision`）：`RegisterGateway` /
  `ReportGatewayStatus` / `GatewayRuntimeStatus` 各增可选字段 **`public_base_url`**
  （管理面「对外地址」优先，未设回落本机 `[server] public_base_url`）。
  注册与周期状态上报都带上它，中心据此知道「这个网关对外是哪个域名」。
- **升级目标可带中心派生的制品地址**：`GatewayUpgradePlan` 增可选字段 **`artifact_url`** —— 由中心反查
  已发布的 release 记录派生（`/api/v1/releases/artifact/...`）。执行器用它取件（`gops --to <url>`），
  不让运维手输远端 / 本机路径。见设计 `edge/gateway-upgrade-and-releases.md`。

### 说明

- 三个字段均可选、`#[serde(default)]`：**线上 JSON 向后兼容**（老 `wist-gwlinkd` / 老网关不带该键，
  中心侧兑底为 `None`）；但 **构造点需补字段**（Rust 结构体字面量），故本版为 **minor**。

## [0.8.0] - 2026-10-06

与 `wist-design/jumo` 模型对齐：网关面「注册 / 凭据」报文体**从 `wist-contracts::gateway_control` 手写副本
收回本 crate 按模型生成**（消除第二份定义；`center` / `gwlinkd` 改用同一类型）。**新增**，无破坏。

### 新增（`Control.Gateway.Security`）

- `GatewayCredentialBundle`、`GatewayClientCertificate`、`GatewayClientCertificateStatus`、
  `GatewayCredentialVerificationResult`。

### 新增（`Control.Gateway.Supervision`）

- `RegisterGateway`、`RenewGatewayCredential`、`VerifyGatewayCredential`、`GatewayEnrollmentResult`。

### 说明

- `GatewayEnrollmentResult` 带回 `credential_bundle: GatewayCredentialBundle`（注册即签发长期凭据）；
  `RenewGatewayCredential` 含 `requested_at`。二者原属「模型落后于代码」的漂移，已在模型侧补齐后生成。
- 时间戳字段为 `crate::DateTime`（线上仍是 RFC3339 串）；生成类型**默认容忍未知字段**
  （原手写副本为 `deny_unknown_fields`）。
- 同时保留了 `Control.GatewayApp.FacingInterface` 的 `ReportAgentStatus` / `AgentStatusAcceptedReturned`（0.7.0）。

## [0.7.0] - 2026-10-06

与 `wist-design/jumo` 模型对齐：网关面的 **agent 状态上报报文体收口进模型**（原为 `wist-center`
本地定义，现由本仓生成），并清理死骨架。

### 新增

- `ReportAgentStatus`（command）：`POST /api/v1/gateway/agents/status` 的报文体
  （`gateway_id` + `agents: List<AgentRuntimeStatus>`）。
- `AgentStatusAcceptedReturned`（response）：该入口的 200 回执（`gateway_id` + `agents_accepted`）。
  接收端 `wist-center` 改用它，**不再本地定义** `AgentStatusReportRequest` / `AgentStatusEntry`，
  消除发送/接收两侧的 drift。

### 移除

- **破坏性**：删除 `SubmitEnrollmentRequest` 生成骨架 —— 无模型对应、无调用方
  （agent 侧报名报文已由 `wist-api::enrollment` 承载）。

### 说明

- `WarpGatewayInstance` / `GatewayUpgradeResultAccepted` / `PublishWarpGateway` / `WarpGatewayRelease`
  去除 `#[jumo]` 注解（分别为无模型对应、或注解 `kind` 与模型不符）；仅注解层，不影响 wire。

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
