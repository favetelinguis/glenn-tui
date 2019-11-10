use self::super::settings::Settings;

use rusoto_core::{HttpClient, Region, request};
use rusoto_ssm::SsmClient;
use std::collections::HashMap;
use rusoto_credential::{StaticProvider, AutoRefreshingProvider};
use rusoto_sts::{StsClient, StsAssumeRoleSessionCredentialsProvider};
use hyper_proxy::{Proxy, Intercept, ProxyConnector};
use hyper_tls::HttpsConnector;
use hyper::client::connect::dns::GaiResolver;
use std::str::FromStr;

pub struct BaseClientCredentials {
    access_key: String,
    secret: String,
    role_arn: String,
    region: String,
}

pub struct Clients {
    clients: HashMap<String, BaseClientCredentials>,
    client_ids: Vec<String>,
}

impl Clients {
    pub fn new(settings: &Settings) -> Self {
        let mut clients = HashMap::new();
        let mut client_ids = Vec::new();

        for account in &settings.clients {
            for role in &account.roles {
                let client = BaseClientCredentials {
                    access_key: account.key.to_owned(),
                    secret: account.secret.to_owned(),
                    role_arn: role.arn.to_owned(),
                    region: role.region.to_owned(),
                };
                client_ids.push(role.name.to_owned());
                clients.insert(role.name.to_owned(), client);
            }
        }

        Clients {
            clients,
            client_ids}
    }

    pub fn available_clients(self: &Self) -> &Vec<String> {
        &self.client_ids
    }

    pub fn create_ssm_client(self: &Self, id: String) -> SsmClient {
        let credentials = self.clients.get(id.as_str()).unwrap();
        let client = HttpClient::from_connector(build_proxy_connector());
        SsmClient::new_with(
        client,
            build_provider(credentials.role_arn.to_owned(), build_region(credentials.region.as_str()), credentials.access_key.to_owned(), credentials.secret.to_owned())
            , build_region(credentials.region.as_str()))
    }
}

type HttpConnector = hyper_proxy::ProxyConnector<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

fn build_proxy_connector() -> HttpConnector {
    let client = match std::env::var("HTTPS_PROXY") {
        Ok(proxy_uri) => {
            let proxy = Proxy::new(Intercept::All, proxy_uri.parse().unwrap());
            let proxy_connector = ProxyConnector::from_proxy(
                HttpsConnector::new(4).unwrap(), proxy,
            ).unwrap();
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

fn build_provider(role_arn: String, region: Region, access_key: String, secret: String) -> AutoRefreshingProvider<StsAssumeRoleSessionCredentialsProvider> {
    let credentials_provider = StaticProvider::new(access_key, secret, None, None);
    let client = HttpClient::from_connector(build_proxy_connector());
    let sts = StsClient::new_with(client, credentials_provider, region);

    let provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts,
        role_arn,
        "default".to_owned(),
        None, None, None, None,
    );

    AutoRefreshingProvider::new(provider).unwrap()
}