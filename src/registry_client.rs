use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone)]
pub struct RegistryClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Catalog {
    #[serde(default)]
    pub repositories: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TagList {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Manifest {
    #[serde(default, rename = "schemaVersion")]
    pub schema_version: i32,
    #[serde(default, rename = "mediaType")]
    pub media_type: String,
    #[serde(default)]
    pub config: Option<ManifestConfig>,
    #[serde(default)]
    pub layers: Vec<ManifestLayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManifestConfig {
    #[serde(default, rename = "mediaType")]
    pub media_type: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManifestLayer {
    #[serde(default, rename = "mediaType")]
    pub media_type: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default)]
    pub digest: String,
}

#[derive(Debug, Clone, Default)]
pub struct RepoInfo {
    pub name: String,
    pub tags: Vec<TagInfo>,
    pub total_size: u64,
    pub tag_count: usize,
    pub in_use: bool,
    pub used_by: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TagInfo {
    pub name: String,
    pub digest: String,
    pub size: u64,
    pub layers: usize,
    pub created: Option<String>,
    pub architecture: Option<String>,
    pub os: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageConfig {
    #[serde(default)]
    pub architecture: Option<String>,
    #[serde(default)]
    pub os: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub config: Option<ImageConfigDetail>,
    #[serde(default)]
    pub history: Vec<HistoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageConfigDetail {
    #[serde(default, rename = "Env")]
    pub env: Vec<String>,
    #[serde(default, rename = "Entrypoint")]
    pub entrypoint: Option<Vec<String>>,
    #[serde(default, rename = "Cmd")]
    pub cmd: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryEntry {
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub created_by: Option<String>,
}

impl RegistryClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn catalog(&self) -> Result<Vec<String>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/v2/_catalog", self.base_url))
            .send()
            .await?;
        let catalog: Catalog = resp.json().await.unwrap_or_default();
        Ok(catalog.repositories)
    }

    pub async fn tags(&self, repo: &str) -> Result<Vec<String>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/v2/{}/tags/list", self.base_url, repo))
            .send()
            .await?;
        let tag_list: TagList = resp.json().await.unwrap_or_default();
        Ok(tag_list.tags.unwrap_or_default())
    }

    pub async fn manifest(&self, repo: &str, tag: &str) -> Result<(Manifest, String), reqwest::Error> {
        let resp = self.client
            .get(format!("{}/v2/{}/manifests/{}", self.base_url, repo, tag))
            .header("Accept", "application/vnd.docker.distribution.manifest.v2+json, application/vnd.oci.image.manifest.v1+json")
            .send()
            .await?;
        let digest = resp.headers()
            .get("docker-content-digest")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let manifest: Manifest = resp.json().await.unwrap_or_default();
        Ok((manifest, digest))
    }

    pub async fn image_config(&self, repo: &str, digest: &str) -> Result<ImageConfig, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/v2/{}/blobs/{}", self.base_url, repo, digest))
            .send()
            .await?;
        let config: ImageConfig = resp.json().await.unwrap_or_default();
        Ok(config)
    }

    pub async fn get_repo_info(&self, repo: &str, used_images: &HashMap<String, Vec<String>>) -> RepoInfo {
        let tags = self.tags(repo).await.unwrap_or_default();
        let mut tag_infos = Vec::new();
        let mut total_size: u64 = 0;

        for tag in &tags {
            if let Ok((manifest, digest)) = self.manifest(repo, tag).await {
                let layer_size: u64 = manifest.layers.iter().map(|l| l.size).sum();
                let config_size = manifest.config.as_ref().map(|c| c.size).unwrap_or(0);
                let size = layer_size + config_size;
                total_size += size;

                let mut created = None;
                let mut architecture = None;
                let mut os = None;

                if let Some(cfg) = &manifest.config {
                    if let Ok(img_cfg) = self.image_config(repo, &cfg.digest).await {
                        created = img_cfg.created.clone();
                        architecture = img_cfg.architecture.clone();
                        os = img_cfg.os.clone();
                    }
                }

                tag_infos.push(TagInfo {
                    name: tag.clone(),
                    digest,
                    size,
                    layers: manifest.layers.len(),
                    created,
                    architecture,
                    os,
                });
            }
        }

        let image_ref = format!("{}", repo);
        let (in_use, used_by) = if let Some(pods) = used_images.get(&image_ref) {
            (true, pods.clone())
        } else {
            (false, vec![])
        };

        RepoInfo {
            name: repo.to_string(),
            tags: tag_infos,
            total_size,
            tag_count: tags.len(),
            in_use,
            used_by,
        }
    }
}

pub fn format_size(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{} {}", bytes, units[unit_idx])
    } else {
        format!("{:.1} {}", size, units[unit_idx])
    }
}
