use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use std::collections::HashMap;
use crate::AppState;
use crate::registry_client::{RepoInfo, format_size};
use crate::template_response::HtmlTemplate;

#[derive(Template)]
#[template(path = "registry.html")]
pub struct RegistryTemplate {
    pub repos: Vec<RepoInfo>,
    pub total_storage: String,
    pub total_repos: usize,
    pub total_tags: usize,
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    let repos_names = state.registry.catalog().await.unwrap_or_default();

    // Build map of images in use by pods
    let pods = state.mkube.list_pods().await.unwrap_or_default();
    let mut used_images: HashMap<String, Vec<String>> = HashMap::new();
    for pod in &pods {
        let pod_name = format!("{}/{}", pod.metadata.namespace, pod.metadata.name);
        for container in &pod.spec.containers {
            let repo = extract_repo_name(&container.image);
            used_images.entry(repo).or_default().push(pod_name.clone());
        }
    }

    let mut repos = Vec::new();
    let mut total_storage: u64 = 0;
    let mut total_tags: usize = 0;

    for name in &repos_names {
        let info = state.registry.get_repo_info(name, &used_images).await;
        total_storage += info.total_size;
        total_tags += info.tag_count;
        repos.push(info);
    }

    HtmlTemplate(RegistryTemplate {
        total_storage: format_size(total_storage),
        total_repos: repos.len(),
        total_tags,
        repos,
    })
}

fn extract_repo_name(image: &str) -> String {
    let without_tag = image.split(':').next().unwrap_or(image);
    without_tag.rsplit('/').next().unwrap_or(without_tag).to_string()
}
