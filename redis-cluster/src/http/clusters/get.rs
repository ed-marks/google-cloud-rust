use reqwest_middleware::{ClientWithMiddleware as Client, RequestBuilder};

use crate::http::Escape;

/// Request message for DeleteBucket.
#[derive(Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetClusterRequest {
    /// Required. Name of a redis cluster.
    #[serde(skip_serializing)]
    pub cluster: String,
    /// Required. location of a redis cluster.
    #[serde(skip_serializing)]
    pub location: String,
    /// Required. project of a redis cluster.
    #[serde(skip_serializing)]
    pub project: String,
}

pub(crate) fn build(base_url: &str, client: &Client, req: &GetClusterRequest) -> RequestBuilder {
    let url = format!(
        "{}/projects/{}/locations/{}/clusters/{}",
        base_url,
        req.project.escape(),
        req.location.escape(),
        req.cluster.escape()
    );
    client.get(url).query(&req)
}
