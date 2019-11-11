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

pub struct BaseClientCredentials<'a> {
    access_key: &'a str,
    secret: &'a str,
    role_arn: &'a str,
    region: &'a str,
}

pub struct Clients<'a> {
    clients: HashMap<&'a str, BaseClientCredentials<'a>>,
    client_ids: Vec<&'a str>,
}

impl<'a> Clients<'a> {
    pub fn new(settings: &'a Settings) -> Self {
        let mut clients: HashMap<&str, BaseClientCredentials> = HashMap::new();
        let mut client_ids: Vec<&str> = Vec::new();

        for account in &settings.clients {
            for role in &account.roles {
                let client = BaseClientCredentials {
                    access_key: &account.key,
                    secret: &account.secret,
                    role_arn: &role.arn,
                    region: &role.region,
                };
                client_ids.push(&role.name);
                clients.insert(&role.name, client);
            }
        }

        Clients {
            clients,
            client_ids}
    }

    pub fn available_clients(self: &Self) -> &Vec<&str> {
        &self.client_ids
    }

    pub fn create_ssm_client(self: &Self, id: &str) -> SsmClient {
        let credentials = self.clients.get(id).unwrap();
        let client = HttpClient::from_connector(build_proxy_connector());
        SsmClient::new_with(
        client,
            build_provider(credentials.role_arn, build_region(credentials.region), credentials.access_key, credentials.secret)
            , build_region(credentials.region))
    }

    pub fn create_ssm_clients(self: &Self, ids: &Vec<&str>) -> Vec<SsmClient> {
        let mut clients = vec![];
        for id in ids {
            clients.push(self.create_ssm_client(id));
        }
        clients
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

fn build_provider(role_arn: &str, region: Region, access_key: &str, secret: &str) -> AutoRefreshingProvider<StsAssumeRoleSessionCredentialsProvider> {
    let credentials_provider = StaticProvider::new(access_key.to_owned(), secret.to_owned(), None, None);
    let client = HttpClient::from_connector(build_proxy_connector());
    let sts = StsClient::new_with(client, credentials_provider, region);

    let provider = StsAssumeRoleSessionCredentialsProvider::new(
        sts,
        role_arn.to_owned(),
        "default".to_owned(),
        None, None, None, None,
    );

    AutoRefreshingProvider::new(provider).unwrap()
}