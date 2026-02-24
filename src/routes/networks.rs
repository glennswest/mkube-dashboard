use askama::Template;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use crate::AppState;
use crate::mkube_client::Network;
use crate::template_response::HtmlTemplate;

#[derive(Template)]
#[template(path = "networks.html")]
pub struct NetworksTemplate {
    pub networks: Vec<Network>,
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    let networks = state.mkube.list_networks().await.unwrap_or_default();
    HtmlTemplate(NetworksTemplate { networks })
}

#[derive(Template)]
#[template(path = "network_detail.html")]
pub struct NetworkDetailTemplate {
    pub network: Network,
}

pub async fn detail(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let network = state.mkube.get_network(&name).await.unwrap_or_default();
    HtmlTemplate(NetworkDetailTemplate { network })
}
