// TODO create function to create connector
// TODO create function for creating BaseClients
use rusoto_core::{HttpClient, Region};
use rusoto_ssm::SsmClient;

use self::super::settings::Settings;
use std::collections::HashMap;

pub struct Clients {
    // TODO create a map of clients and providers
    clients: HashMap<String, HttpClient>,
    providers: HashMap<String, String>, // TODO should be provider
}

impl Clients {
    pub fn new(settings: &Settings) -> Self {
        let mut clients = HashMap::new();
        let mut providers = HashMap::new();

        // //TODO iterate over each settigs and create a client and provider and put in map

        Clients { clients, providers }
    }
    // TODO create new service clients that are based on the base client/provider
    pub fn create_ssm_client(self: &Self) -> SsmClient {
        SsmClient::new(Region::EuWest1)
        // TODO SsmClient::new_with()
    }
}
