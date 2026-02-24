use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use serde::Deserialize;
use crate::AppState;
use crate::dns_client::{Zone, DnsRecord, CreateRecordRequest, RecordData};
use crate::template_response::HtmlTemplate;

pub struct NetworkZoneView {
    pub name: String,
    pub url: String,
    pub zone_name: String,
    pub reachable: bool,
    pub zones: Vec<Zone>,
}

#[derive(Template)]
#[template(path = "dns.html")]
pub struct DnsTemplate {
    pub networks: Vec<NetworkZoneView>,
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    let mut networks = Vec::new();
    for ep in &state.config.dns_endpoints {
        let reachable = state.dns.check_health(&ep.url).await;
        let zones = if reachable {
            state.dns.list_zones(&ep.url).await.unwrap_or_default()
        } else {
            vec![]
        };
        networks.push(NetworkZoneView {
            name: ep.name.clone(),
            url: ep.url.clone(),
            zone_name: ep.zone.clone(),
            reachable,
            zones,
        });
    }
    HtmlTemplate(DnsTemplate { networks })
}

// Pre-compute record display values
pub struct RecordView {
    pub id: String,
    pub name: String,
    pub record_type: String,
    pub value: String,
    pub ttl: i32,
    pub enabled: bool,
}

impl RecordView {
    pub fn from_record(r: &DnsRecord) -> Self {
        Self {
            id: r.id.clone(),
            name: r.name.clone(),
            record_type: r.data.record_type().to_string(),
            value: r.data.display_value(),
            ttl: r.ttl,
            enabled: r.enabled,
        }
    }
}

#[derive(Template)]
#[template(path = "dns_zone.html")]
pub struct DnsZoneTemplate {
    pub network: String,
    pub zone_id: String,
    pub zone_name: String,
    pub records: Vec<RecordView>,
    pub error: Option<String>,
}

pub async fn zone_detail(
    State(state): State<AppState>,
    Path((network, zone_id)): Path<(String, String)>,
) -> impl IntoResponse {
    let ep = state.config.dns_endpoints.iter()
        .find(|e| e.name == network);

    let (dns_url, zone_name) = match ep {
        Some(e) => (e.url.clone(), e.zone.clone()),
        None => return HtmlTemplate(DnsZoneTemplate {
            network,
            zone_id,
            zone_name: "Unknown".to_string(),
            records: vec![],
            error: Some("Unknown network".to_string()),
        }),
    };

    let raw_records = state.dns.list_records(&dns_url, &zone_id).await.unwrap_or_default();
    let records: Vec<RecordView> = raw_records.iter().map(RecordView::from_record).collect();

    HtmlTemplate(DnsZoneTemplate {
        network,
        zone_id,
        zone_name,
        records,
        error: None,
    })
}

#[derive(Deserialize)]
pub struct CreateRecordForm {
    pub name: String,
    pub record_type: String,
    pub value: String,
    #[serde(default = "default_ttl")]
    pub ttl: i32,
}

fn default_ttl() -> i32 { 300 }

pub async fn create_record(
    State(state): State<AppState>,
    Path((network, zone_id)): Path<(String, String)>,
    Form(form): Form<CreateRecordForm>,
) -> impl IntoResponse {
    let ep = state.config.dns_endpoints.iter().find(|e| e.name == network);
    let dns_url = match ep {
        Some(e) => e.url.clone(),
        None => return (StatusCode::BAD_REQUEST, "Unknown network").into_response(),
    };

    let data = match form.record_type.as_str() {
        "A" => RecordData::A(form.value),
        "AAAA" => RecordData::AAAA(form.value),
        "CNAME" => RecordData::CNAME(form.value),
        "NS" => RecordData::NS(form.value),
        "PTR" => RecordData::PTR(form.value),
        "TXT" => RecordData::TXT(form.value),
        _ => return (StatusCode::BAD_REQUEST, "Unsupported record type").into_response(),
    };

    let req = CreateRecordRequest {
        name: form.name,
        ttl: form.ttl,
        data,
        enabled: true,
    };

    match state.dns.create_record(&dns_url, &zone_id, &req).await {
        Ok(_) => Redirect::to(&format!("/dns/{}/{}", network, zone_id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed: {}", e)).into_response(),
    }
}

pub async fn delete_record(
    State(state): State<AppState>,
    Path((network, zone_id, record_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    let ep = state.config.dns_endpoints.iter().find(|e| e.name == network);
    let dns_url = match ep {
        Some(e) => e.url.clone(),
        None => return (StatusCode::BAD_REQUEST, "Unknown network").into_response(),
    };

    match state.dns.delete_record(&dns_url, &zone_id, &record_id).await {
        Ok(_) => Redirect::to(&format!("/dns/{}/{}", network, zone_id)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed: {}", e)).into_response(),
    }
}
