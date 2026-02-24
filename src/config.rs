use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    pub mkube_url: String,
    pub registry_url: String,
    pub dns_endpoints: Vec<DnsEndpoint>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DnsEndpoint {
    pub name: String,
    pub url: String,
    pub zone: String,
}

fn default_listen_addr() -> String {
    "0.0.0.0:8080".to_string()
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
