#[derive(Debug, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
pub struct WarpGatewayRelease {
    pub version: String,
    pub artifact_url: String,
    pub status: String,
    pub published_at: crate::DateTime,
}
