mod config;
mod dns_client;
mod mkube_client;
mod registry_client;
mod routes;
mod template_response;

use axum::{routing::{get, post}, Router};
use clap::Parser;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub config: std::sync::Arc<config::Config>,
    pub mkube: mkube_client::MkubeClient,
    pub dns: dns_client::DnsClient,
    pub registry: registry_client::RegistryClient,
}

#[derive(Parser)]
#[command(name = "mkube-dashboard")]
struct Cli {
    #[arg(short, long, default_value = "/etc/dashboard/config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cli = Cli::parse();

    let cfg = config::Config::load(&cli.config).unwrap_or_else(|e| {
        tracing::warn!("Failed to load config from {}: {}, using defaults", cli.config, e);
        config::Config {
            listen_addr: "0.0.0.0:8080".to_string(),
            mkube_url: "http://192.168.200.2:8082".to_string(),
            registry_url: "http://192.168.200.3:5000".to_string(),
            dns_endpoints: vec![
                config::DnsEndpoint { name: "gt".into(), url: "http://192.168.200.199:8080".into(), zone: "gt.lo".into() },
                config::DnsEndpoint { name: "g10".into(), url: "http://192.168.10.252:8080".into(), zone: "g10.lo".into() },
                config::DnsEndpoint { name: "g11".into(), url: "http://192.168.11.252:8080".into(), zone: "g11.lo".into() },
                config::DnsEndpoint { name: "gw".into(), url: "http://192.168.1.199:8080".into(), zone: "gw.lo".into() },
            ],
        }
    });

    let state = AppState {
        mkube: mkube_client::MkubeClient::new(&cfg.mkube_url),
        dns: dns_client::DnsClient::new(),
        registry: registry_client::RegistryClient::new(&cfg.registry_url),
        config: std::sync::Arc::new(cfg.clone()),
    };

    let app = Router::new()
        // Dashboard
        .route("/", get(routes::dashboard::index))
        .route("/dashboard/content", get(routes::dashboard::index_content))
        // Pods
        .route("/pods", get(routes::pods::list))
        .route("/pods/{namespace}/{name}", get(routes::pods::detail))
        .route("/pods/{namespace}/{name}/logs", get(routes::pods::logs_page))
        .route("/pods/{namespace}/{name}/restart", post(routes::pods::restart))
        .route("/pods/{namespace}/{name}/delete", post(routes::pods::delete))
        // Log proxy
        .route("/proxy/logs/{namespace}/{name}/{container}", get(routes::pods::logs_content))
        // DNS
        .route("/dns", get(routes::dns::list))
        .route("/dns/{network}/{zone_id}", get(routes::dns::zone_detail))
        .route("/dns/{network}/{zone_id}/records", post(routes::dns::create_record))
        .route("/dns/{network}/{zone_id}/records/{record_id}/delete", post(routes::dns::delete_record))
        // Networks
        .route("/networks", get(routes::networks::list))
        .route("/networks/{name}", get(routes::networks::detail))
        // Nodes
        .route("/nodes", get(routes::nodes::list))
        .route("/nodes/{name}", get(routes::nodes::detail))
        // Registry
        .route("/registry", get(routes::registry::list))
        // Events
        .route("/events", get(routes::events::list))
        // Health
        .route("/healthz", get(routes::health::healthz))
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listen_addr = cfg.listen_addr.clone();
    tracing::info!("Starting mkube-dashboard on {}", listen_addr);

    let listener = tokio::net::TcpListener::bind(&listen_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
