pub mod get;
pub mod get_certs;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CaCerts {
    pub certificates: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ManagedServer {
    pub ca_certs: Vec<CaCerts>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CertResponse {
    pub managed_server_ca: ManagedServer,
}
