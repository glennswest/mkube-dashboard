use askama::Template;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use serde::Deserialize;
use crate::AppState;
use crate::mkube_client::Pod;
use crate::template_response::HtmlTemplate;

// View model for pod row to pre-compute display values
pub struct PodRow {
    pub namespace: String,
    pub name: String,
    pub phase: String,
    pub pod_ip: String,
    pub container_count: usize,
    pub restart_count: i32,
    pub created: String,
}

impl PodRow {
    pub fn from_pod(pod: &Pod) -> Self {
        let (phase, pod_ip, restart_count) = match &pod.status {
            Some(s) => (
                s.phase.clone(),
                s.pod_ip.clone().unwrap_or_else(|| "-".to_string()),
                s.container_statuses.iter().map(|c| c.restart_count).sum(),
            ),
            None => ("Unknown".to_string(), "-".to_string(), 0),
        };
        Self {
            namespace: pod.metadata.namespace.clone(),
            name: pod.metadata.name.clone(),
            phase,
            pod_ip,
            container_count: pod.spec.containers.len(),
            restart_count,
            created: pod.metadata.creation_timestamp.clone().unwrap_or_else(|| "-".to_string()),
        }
    }
}

#[derive(Template)]
#[template(path = "pods.html")]
pub struct PodsTemplate {
    pub pods: Vec<PodRow>,
    pub filter_ns: String,
}

#[derive(Deserialize, Default)]
pub struct PodFilter {
    #[serde(default)]
    pub ns: String,
}

pub async fn list(State(state): State<AppState>, Query(filter): Query<PodFilter>) -> impl IntoResponse {
    let raw_pods = if filter.ns.is_empty() {
        state.mkube.list_pods().await.unwrap_or_default()
    } else {
        state.mkube.list_pods_in_namespace(&filter.ns).await.unwrap_or_default()
    };
    let pods: Vec<PodRow> = raw_pods.iter().map(PodRow::from_pod).collect();

    HtmlTemplate(PodsTemplate {
        pods,
        filter_ns: filter.ns,
    })
}

// Pre-computed container status for detail view
pub struct ContainerDetail {
    pub name: String,
    pub image: String,
    pub ready: bool,
    pub restart_count: i32,
    pub state_display: String,
    pub state_class: String,
}

pub struct VolumeDetail {
    pub name: String,
    pub vol_type: String,
    pub source: String,
}

#[derive(Template)]
#[template(path = "pod_detail.html")]
pub struct PodDetailTemplate {
    pub namespace: String,
    pub pod_name: String,
    pub phase: String,
    pub pod_ip: String,
    pub start_time: String,
    pub containers: Vec<ContainerDetail>,
    pub volumes: Vec<VolumeDetail>,
    pub annotations: Vec<(String, String)>,
}

pub async fn detail(
    State(state): State<AppState>,
    Path((namespace, name)): Path<(String, String)>,
) -> impl IntoResponse {
    let pod = state.mkube.get_pod(&namespace, &name).await.unwrap_or_default();

    let (phase, pod_ip, start_time, container_details) = match &pod.status {
        Some(s) => {
            let details: Vec<ContainerDetail> = s.container_statuses.iter().map(|cs| {
                let (state_display, state_class) = match &cs.state {
                    Some(st) => {
                        if st.running.is_some() {
                            ("Running".to_string(), "running".to_string())
                        } else if let Some(w) = &st.waiting {
                            (format!("Waiting: {}", w.reason.as_deref().unwrap_or("unknown")), "pending".to_string())
                        } else if let Some(t) = &st.terminated {
                            (format!("Terminated ({})", t.exit_code), "failed".to_string())
                        } else {
                            ("-".to_string(), "unknown".to_string())
                        }
                    }
                    None => ("-".to_string(), "unknown".to_string()),
                };
                ContainerDetail {
                    name: cs.name.clone(),
                    image: cs.image.clone(),
                    ready: cs.ready,
                    restart_count: cs.restart_count,
                    state_display,
                    state_class,
                }
            }).collect();
            (
                s.phase.clone(),
                s.pod_ip.clone().unwrap_or_else(|| "-".to_string()),
                s.start_time.clone().unwrap_or_else(|| "-".to_string()),
                details,
            )
        }
        None => {
            let details: Vec<ContainerDetail> = pod.spec.containers.iter().map(|c| {
                ContainerDetail {
                    name: c.name.clone(),
                    image: c.image.clone(),
                    ready: false,
                    restart_count: 0,
                    state_display: "-".to_string(),
                    state_class: "unknown".to_string(),
                }
            }).collect();
            ("Unknown".to_string(), "-".to_string(), "-".to_string(), details)
        }
    };

    let volumes: Vec<VolumeDetail> = pod.spec.volumes.iter().map(|v| {
        let (vol_type, source) = if let Some(cm) = &v.config_map {
            ("ConfigMap".to_string(), cm.name.clone())
        } else if let Some(hp) = &v.host_path {
            ("HostPath".to_string(), hp.path.clone())
        } else {
            ("Other".to_string(), "-".to_string())
        };
        VolumeDetail { name: v.name.clone(), vol_type, source }
    }).collect();

    let annotations: Vec<(String, String)> = pod.metadata.annotations.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    HtmlTemplate(PodDetailTemplate {
        namespace,
        pod_name: name,
        phase,
        pod_ip,
        start_time,
        containers: container_details,
        volumes,
        annotations,
    })
}

pub struct ContainerTab {
    pub name: String,
    pub active: bool,
}

#[derive(Template)]
#[template(path = "pod_logs.html")]
pub struct PodLogsTemplate {
    pub namespace: String,
    pub pod_name: String,
    pub container: String,
    pub containers: Vec<ContainerTab>,
}

#[derive(Deserialize, Default)]
pub struct LogParams {
    pub container: Option<String>,
    pub tail: Option<i32>,
}

pub async fn logs_page(
    State(state): State<AppState>,
    Path((namespace, name)): Path<(String, String)>,
    Query(params): Query<LogParams>,
) -> impl IntoResponse {
    let pod = state.mkube.get_pod(&namespace, &name).await.unwrap_or_default();
    let container_names: Vec<String> = pod.spec.containers.iter().map(|c| c.name.clone()).collect();
    let container = params.container.unwrap_or_else(|| {
        container_names.first().cloned().unwrap_or_default()
    });
    let containers: Vec<ContainerTab> = container_names.iter().map(|c| {
        ContainerTab { name: c.clone(), active: c == &container }
    }).collect();

    HtmlTemplate(PodLogsTemplate {
        namespace,
        pod_name: name,
        container,
        containers,
    })
}

pub async fn logs_content(
    State(state): State<AppState>,
    Path((namespace, name, container)): Path<(String, String, String)>,
    Query(params): Query<LogParams>,
) -> impl IntoResponse {
    let tail = params.tail.unwrap_or(200);
    match state.mkube.get_pod_logs(&namespace, &name, &container, Some(tail)).await {
        Ok(logs) => {
            let escaped = logs
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            Html(format!("<pre class=\"log-output\">{}</pre>", escaped))
        }
        Err(e) => Html(format!("<pre class=\"log-output error\">Error fetching logs: {}</pre>", e)),
    }
}

pub async fn restart(
    State(state): State<AppState>,
    Path((namespace, name)): Path<(String, String)>,
) -> impl IntoResponse {
    match state.mkube.delete_pod(&namespace, &name).await {
        Ok(_) => Redirect::to(&format!("/pods/{}/{}", namespace, name)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to restart: {}", e)).into_response(),
    }
}

pub async fn delete(
    State(state): State<AppState>,
    Path((namespace, name)): Path<(String, String)>,
) -> impl IntoResponse {
    match state.mkube.delete_pod(&namespace, &name).await {
        Ok(_) => Redirect::to("/pods").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to delete: {}", e)).into_response(),
    }
}
