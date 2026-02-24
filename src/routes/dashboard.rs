use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use crate::AppState;
use crate::mkube_client::{ConsistencyResult, Event};
use crate::dns_client::DnsEndpointInfo;
use crate::template_response::HtmlTemplate;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate;

fn count_pods(pods: &[crate::mkube_client::Pod]) -> (usize, usize, usize, usize) {
    let running = pods.iter()
        .filter(|p| p.status.as_ref().map(|s| s.phase.as_str()) == Some("Running"))
        .count();
    let pending = pods.iter()
        .filter(|p| p.status.as_ref().map(|s| s.phase.as_str()) == Some("Pending"))
        .count();
    let failed = pods.iter()
        .filter(|p| {
            let phase = p.status.as_ref().map(|s| s.phase.as_str()).unwrap_or("");
            phase == "Failed" || phase == "Unknown"
        })
        .count();
    (pods.len(), running, pending, failed)
}

async fn fetch_dns_status(state: &AppState) -> Vec<DnsEndpointInfo> {
    let mut dns_status = Vec::new();
    for ep in &state.config.dns_endpoints {
        let reachable = state.dns.check_health(&ep.url).await;
        dns_status.push(DnsEndpointInfo {
            name: ep.name.clone(),
            url: ep.url.clone(),
            zone: ep.zone.clone(),
            reachable,
        });
    }
    dns_status
}

pub async fn index() -> impl IntoResponse {
    HtmlTemplate(DashboardTemplate)
}

#[derive(Template)]
#[template(path = "partials/dashboard_content.html")]
pub struct DashboardContentTemplate {
    pub running_count: usize,
    pub pending_count: usize,
    pub failed_count: usize,
    pub total_count: usize,
    pub consistency: ConsistencyResult,
    pub recent_events: Vec<Event>,
    pub dns_status: Vec<DnsEndpointInfo>,
}

pub async fn index_content(State(state): State<AppState>) -> impl IntoResponse {
    let pods = state.mkube.list_pods().await.unwrap_or_default();
    let consistency = state.mkube.get_consistency().await.unwrap_or_default();
    let events = state.mkube.list_events().await.unwrap_or_default();
    let dns_status = fetch_dns_status(&state).await;
    let (total_count, running_count, pending_count, failed_count) = count_pods(&pods);
    let recent_events: Vec<Event> = events.into_iter().rev().take(20).collect();

    HtmlTemplate(DashboardContentTemplate {
        running_count,
        pending_count,
        failed_count,
        total_count,
        consistency,
        recent_events,
        dns_status,
    })
}
