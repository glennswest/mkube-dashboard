use askama::Template;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use crate::AppState;
use crate::template_response::HtmlTemplate;

pub struct NodeView {
    pub name: String,
    pub ready: bool,
    pub architecture: String,
    pub os: String,
    pub kernel: String,
}

#[derive(Template)]
#[template(path = "nodes.html")]
pub struct NodesTemplate {
    pub nodes: Vec<NodeView>,
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    let raw_nodes = state.mkube.list_nodes().await.unwrap_or_default();
    let nodes: Vec<NodeView> = raw_nodes.iter().map(|n| {
        let (ready, arch, os, kernel) = match &n.status {
            Some(s) => {
                let is_ready = s.conditions.iter()
                    .any(|c| c.condition_type == "Ready" && c.status == "True");
                let info = s.node_info.as_ref();
                (
                    is_ready,
                    info.map(|i| i.architecture.clone()).unwrap_or_default(),
                    info.map(|i| i.operating_system.clone()).unwrap_or_default(),
                    info.map(|i| i.kernel_version.clone()).unwrap_or_default(),
                )
            }
            None => (false, String::new(), String::new(), String::new()),
        };
        NodeView { name: n.metadata.name.clone(), ready, architecture: arch, os, kernel }
    }).collect();

    HtmlTemplate(NodesTemplate { nodes })
}

pub struct NodeConditionView {
    pub condition_type: String,
    pub status: String,
    pub message: String,
}

pub struct CapacityView {
    pub resource: String,
    pub capacity: String,
    pub allocatable: String,
}

pub struct PodOnNode {
    pub namespace: String,
    pub name: String,
    pub phase: String,
}

#[derive(Template)]
#[template(path = "node_detail.html")]
pub struct NodeDetailTemplate {
    pub node_name: String,
    pub architecture: String,
    pub os: String,
    pub kernel: String,
    pub conditions: Vec<NodeConditionView>,
    pub capacity: Vec<CapacityView>,
    pub pods: Vec<PodOnNode>,
}

pub async fn detail(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let node = state.mkube.get_node(&name).await.unwrap_or_default();
    let all_pods = state.mkube.list_pods().await.unwrap_or_default();

    let (arch, os, kernel, conditions, capacity) = match &node.status {
        Some(s) => {
            let info = s.node_info.as_ref();
            let conds: Vec<NodeConditionView> = s.conditions.iter().map(|c| {
                NodeConditionView {
                    condition_type: c.condition_type.clone(),
                    status: c.status.clone(),
                    message: c.message.clone().unwrap_or_else(|| "-".to_string()),
                }
            }).collect();
            let caps: Vec<CapacityView> = s.capacity.iter().map(|(k, v)| {
                CapacityView {
                    resource: k.clone(),
                    capacity: v.clone(),
                    allocatable: s.allocatable.get(k).cloned().unwrap_or_else(|| "-".to_string()),
                }
            }).collect();
            (
                info.map(|i| i.architecture.clone()).unwrap_or_default(),
                info.map(|i| i.operating_system.clone()).unwrap_or_default(),
                info.map(|i| i.kernel_version.clone()).unwrap_or_default(),
                conds,
                caps,
            )
        }
        None => (String::new(), String::new(), String::new(), vec![], vec![]),
    };

    let pods: Vec<PodOnNode> = all_pods.iter().map(|p| {
        PodOnNode {
            namespace: p.metadata.namespace.clone(),
            name: p.metadata.name.clone(),
            phase: p.status.as_ref().map(|s| s.phase.clone()).unwrap_or_else(|| "Unknown".to_string()),
        }
    }).collect();

    HtmlTemplate(NodeDetailTemplate {
        node_name: name,
        architecture: arch,
        os,
        kernel,
        conditions,
        capacity,
        pods,
    })
}
