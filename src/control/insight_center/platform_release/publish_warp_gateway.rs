#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct PublishWarpGateway {
    pub version: String,
    pub artifact_url: String,
    pub requested_by: String,
    pub requested_at: crate::DateTime,
}
