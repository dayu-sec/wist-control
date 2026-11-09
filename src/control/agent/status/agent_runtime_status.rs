// @jumo generated
// @jumo hash=b505b229ad05d5d4

#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, ::jumo_derive::Jumo)]
#[jumo(kind = "struct", domain = "Control", module = "Control.Agent.Status")]
pub struct AgentRuntimeStatus {
    pub version: String,
    pub health: String,
    pub instance_id: String,
    pub agent_id: String,
    pub status: String,
    pub last_seen_at: crate::control::types::DateTime,
    pub memory_bytes: Option<i64>,
    pub cpu_percent: Option<f64>,
    pub admin_latency_ms: Option<i64>,
    /// 机器身份：中心页要能回答“这是哪台机器”。
    ///
    /// 来自网关注册表的机器画像；**旧版生产者不报时为 `None`**（不是空串，也不是“没有机器”）。
    /// 加性字段：老消费者忽略它们即可。
    pub node_id: Option<String>,
    pub hostname: Option<String>,
    /// 形如 `en0 192.168.1.5/24`；`None` / 空表 = 还没报过 / 确实没有地址。
    pub ip_addresses: Option<Vec<String>>,
    /// 数据清单：采集授权 + 调查数据（网关从工作授权、agentd 本机视图与调查报告派生）。
    ///
    /// `None` = 旧版生产者不报 —— 中心页据此显示「该网关还没有上报数据清单」，
    /// 而不是显示成「这台机器什么数据都没有」。加性字段：老消费者忽略即可。
    #[serde(default)]
    pub data_manifest: Option<super::AgentDataManifest>,
}
