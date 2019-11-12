use self::super::settings::Settings;

use hyper::client::connect::dns::GaiResolver;
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use hyper_tls::HttpsConnector;
use rusoto_core::{request, HttpClient, Region};
use rusoto_credential::{AutoRefreshingProvider, StaticProvider};
use rusoto_ssm::SsmClient;
use rusoto_sts::{StsAssumeRoleSessionCredentialsProvider, StsClient};
use std::collections::HashMap;
use std::str::FromStr;

pub struct Clients<'a> {
    client_ids: Vec<&'a str>,
    ssm_clients: HashMap<&'a str, SsmClient>,
}

impl<'a> Clients<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        let mut client_ids: Vec<&str> = Vec::new();
        let mut ssm_clients: HashMap<&str, SsmClient> = HashMap::new();

        for account in &settings.clients {
            for role in &account.roles {
                let ssm_client =
                    create_ssm_client(&role.arn, &role.region, &account.key, &account.secret);
                client_ids.push(&role.name);
                ssm_clients.insert(&role.name, ssm_client);
            }
        }

        Clients {
            client_ids,
            ssm_clients,
        }
    }

    pub fn available_clients(&self) -> &Vec<&str> {
        &self.client_ids
    }

    pub fn get_ssm_client(&self, id: &str) -> &SsmClient {
        &self.ssm_clients.get(id).unwrap()
    }
}

type HttpConnector =
    hyper_proxy::ProxyConnector<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

fn build_proxy_connector() -> HttpConnector {
    let client = match std::env::var("HTTPS_PROXY") {
        Ok(proxy_uri) => {
            let proxy = Proxy::new(Intercept::All, proxy_uri.parse().unwrap());
            let proxy_connector =
                ProxyConnector::from_proxy(HttpsConnector::new(4).unwrap(), proxy).unwrap();
            proxy_connector
        }
        Err(_) => {
            let connector = HttpsConnector::new(4).unwrap();
            ProxyConnector::new(connector).unwrap()
        }
    };

    client
}

fn build_region(region: &str) -> Region {
    Region::from_str(region).unwrap()
}

fn build_provider(
    role_arn: &str,
    region: Region,
    access_key: &str,
    secret: &str,
) -> AutoRefreshingProvider<StsAssumeRoleSessionCredentialsProvider> {
    let credentials_provider =
        StaticProvider::new(access_key.to_owned(), secret.to_owned(), None, None);
    let client = HttpClient::from_connector(build_proxy_connector());
    let sts = StsClient::new_with(client, credentials_provider, region);

    let provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts,
        role_arn.to_owned(),
        "default".to_owned(),
        None,
        None,
        None,
        None,
    );

    AutoRefreshingProvider::new(provider).unwrap()
}

fn create_ssm_client(role_arn: &str, region: &str, access_key: &str, secret: &str) -> SsmClient {
    let client = HttpClient::from_connector(build_proxy_connector());
    SsmClient::new_with(
        client,
        build_provider(role_arn, build_region(region), access_key, secret),
        build_region(region),
    )
}
