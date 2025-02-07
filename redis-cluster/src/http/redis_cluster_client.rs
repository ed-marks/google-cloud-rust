use std::sync::Arc;

// use reqwest::header::LOCATION;
// use reqwest::Request;
use reqwest_middleware::RequestBuilder;

use crate::http::check_response_status;
// use crate::http::clusters::get::GetClusterRequest;
use crate::http::clusters::get_certs::GetClusterCertsRequest;
use crate::http::{clusters, Error};

use google_cloud_token::TokenSource;

use super::clusters::CertResponse;

pub const SCOPES: [&str; 1] = ["https://www.googleapis.com/auth/cloud-platform"];

#[derive(Clone)]
pub struct RedisClusterClient {
    ts: Option<Arc<dyn TokenSource>>,
    v1_endpoint: String,
    http: reqwest_middleware::ClientWithMiddleware,
}

impl RedisClusterClient {
    pub(crate) fn new(
        ts: Option<Arc<dyn TokenSource>>,
        endpoint: &str,
        http: reqwest_middleware::ClientWithMiddleware,
    ) -> Self {
        Self {
            ts,
            v1_endpoint: format!("{endpoint}/v1"),
            http,
        }
    }

    // /// Gets a cluster.
    // /// https://cloud.google.com/storage/docs/json_api/v1/buckets/delete
    // ///
    // /// ```
    // /// use google_cloud_redis_cluster::client::Client;
    // /// use google_cloud_redis_cluster::http::clusters::get::GetClusterRequest;
    // ///
    // /// async fn run(client:Client) {
    // ///     let result = client.get_cluster(&GetClusterRequest {
    // ///         cluster: "cluster".to_string(),
    // ///         ..Default::default()
    // ///     }).await;
    // /// }
    // /// ```
    // #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    // pub async fn get_cluster(&self, req: &GetClusterRequest) -> Result<(), Error> {
    //     let builder = clusters::get::build(self.v1_endpoint.as_str(), &self.http, req);
    //     self.send_get_empty(builder).await
    // }

    /// Gets CA certificates for a cluster.
    /// https://cloud.google.com/storage/docs/json_api/v1/buckets/insert
    ///
    /// ```
    /// use google_cloud_storage::client::Client;
    /// use google_cloud_storage::http::clusters::get_certs::GetClusterCertsRequest;
    ///
    /// async fn run(client:Client) {
    ///     let result = client.get_cluster_certs(&GetClusterCertsRequest {
    ///         cluster: "cluster".to_string(),
    ///         },
    ///         ..Default::default()
    ///     }).await;
    /// }
    /// ```
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn get_cluster_certs(&self, req: &GetClusterCertsRequest) -> Result<CertResponse, Error> {
        let builder = clusters::get_certs::build(self.v1_endpoint.as_str(), &self.http, req);
        self.send(builder).await
    }

    async fn with_headers(&self, builder: RequestBuilder) -> Result<RequestBuilder, Error> {
        let builder = builder
            .header("X-Goog-Api-Client", "rust")
            .header(reqwest::header::USER_AGENT, "google-cloud-storage");
        let builder = match &self.ts {
            Some(ts) => {
                let token = ts.token().await.map_err(Error::TokenSource)?;
                builder.header(reqwest::header::AUTHORIZATION, token)
            }
            None => builder,
        };
        Ok(builder)
    }

    // async fn send_request<T>(&self, request: Request) -> Result<T, Error>
    // where
    //     T: serde::de::DeserializeOwned,
    // {
    //     let response = self.http.execute(request).await?;
    //     let response = check_response_status(response).await?;
    //     Ok(response.json().await?)
    // }

    async fn send<T>(&self, builder: RequestBuilder) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let builder = self.with_headers(builder).await?;
        let response = builder.send().await?;
        let response = check_response_status(response).await?;
        Ok(response.json().await?)
    }

    // async fn send_get_empty(&self, builder: RequestBuilder) -> Result<(), Error> {
    //     let builder = self.with_headers(builder).await?;
    //     let response = builder.send().await?;
    //     check_response_status(response).await?;
    //     Ok(())
    // }

    //     async fn send_get_url(&self, builder: RequestBuilder) -> Result<String, Error> {
    //         let builder = self.with_headers(builder).await?;
    //         let response = builder.send().await?;
    //         let response = check_response_status(response).await?;
    //         Ok(String::from_utf8_lossy(response.headers()[LOCATION].as_bytes()).into_owned())
    //     }
    // }
}

// #[cfg(test)]
// pub(crate) mod test {
//     // use bytes::Buf;
//     // use futures_util::StreamExt;
//     use serial_test::serial;
//
//     use google_cloud_auth::project::Config;
//     use google_cloud_auth::token::DefaultTokenSourceProvider;
//     use google_cloud_token::TokenSourceProvider;
//
//     // use crate::http::clusters::get::GetClusterRequest;
//     use crate::http::clusters::get_certs::GetClusterCertsRequest;
//
//     use super::{RedisClusterClient, SCOPES};
//
//     #[ctor::ctor]
//     fn init() {
//         let filter = tracing_subscriber::filter::EnvFilter::from_default_env()
//             .add_directive("google_cloud_redis_cluster=trace".parse().unwrap());
//         let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
//     }
//
//     async fn client() -> (RedisClusterClient, String, String) {
//         let tsp = DefaultTokenSourceProvider::new(Config::default().with_scopes(&SCOPES))
//             .await
//             .unwrap();
//         let cred = tsp.source_credentials.clone();
//         let ts = tsp.token_source();
//         let client = RedisClusterClient::new(
//             Some(ts),
//             "https://redis.googleapis.com",
//             reqwest_middleware::ClientBuilder::new(reqwest::Client::default()).build(),
//         );
//         let cred = cred.unwrap();
//         (client, cred.project_id.unwrap(), cred.client_email.unwrap())
//     }
//
//     #[tokio::test]
//     #[serial]
//     pub async fn get_cluster() {
//         let (client, project, _) = client().await;
//         let buckets = client
//             .get_cluster(&GetClusterRequest {
//                 project: project.clone(),
//                 max_results: None,
//                 page_token: None,
//                 prefix: Some(bucket_name(&project, "object")),
//                 projection: None,
//                 match_glob: None,
//             })
//             .await
//             .unwrap();
//         assert_eq!(2, buckets.items.len());
//     }
//
//     #[tokio::test]
//     #[serial]
//     pub async fn get_cluster_certs() {
//         let (client, project, _) = client().await;
//         let certs = client
//             .get_cluster_certs(&GetClusterCertsRequest {
//                 project: project.clone(),
//                 cluster: "cluster".to_string(),
//                 location: "europe-west1".to_string(),
//             })
//             .await
//             .unwrap();
//         assert_eq!(2, certs.certificates.len());
//     }
// }
