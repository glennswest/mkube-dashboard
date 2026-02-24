use askama::Template;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::AppState;
use crate::template_response::HtmlTemplate;

pub struct EventView {
    pub timestamp: String,
    pub event_type: String,
    pub reason: String,
    pub obj_namespace: String,
    pub obj_name: String,
    pub message: String,
    pub count: i32,
}

#[derive(Template)]
#[template(path = "events.html")]
pub struct EventsTemplate {
    pub events: Vec<EventView>,
    pub filter_ns: String,
}

#[derive(Deserialize, Default)]
pub struct EventFilter {
    #[serde(default)]
    pub ns: String,
}

pub async fn list(State(state): State<AppState>, Query(filter): Query<EventFilter>) -> impl IntoResponse {
    let mut raw_events = state.mkube.list_events().await.unwrap_or_default();

    if !filter.ns.is_empty() {
        raw_events.retain(|e| e.metadata.namespace == filter.ns);
    }

    raw_events.reverse();

    let events: Vec<EventView> = raw_events.iter().map(|e| {
        let (obj_ns, obj_name) = match &e.involved_object {
            Some(obj) => (obj.namespace.clone(), obj.name.clone()),
            None => (String::new(), String::new()),
        };
        EventView {
            timestamp: e.last_timestamp.clone().unwrap_or_else(|| "-".to_string()),
            event_type: e.event_type.clone(),
            reason: e.reason.clone(),
            obj_namespace: obj_ns,
            obj_name: obj_name,
            message: e.message.clone(),
            count: e.count.unwrap_or(1),
        }
    }).collect();

    HtmlTemplate(EventsTemplate {
        events,
        filter_ns: filter.ns,
    })
}
