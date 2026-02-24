use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone)]
pub struct MkubeClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Pod {
    #[serde(default)]
    pub metadata: PodMetadata,
    #[serde(default)]
    pub spec: PodSpec,
    #[serde(default)]
    pub status: Option<PodStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PodMetadata {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default, rename = "creationTimestamp")]
    pub creation_timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PodSpec {
    #[serde(default)]
    pub containers: Vec<Container>,
    #[serde(default)]
    pub volumes: Vec<Volume>,
    #[serde(default, rename = "restartPolicy")]
    pub restart_policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Container {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub env: Vec<EnvVar>,
    #[serde(default, rename = "volumeMounts")]
    pub volume_mounts: Vec<VolumeMount>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvVar {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Volume {
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "configMap")]
    pub config_map: Option<ConfigMapRef>,
    #[serde(default, rename = "hostPath")]
    pub host_path: Option<HostPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigMapRef {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HostPath {
    #[serde(default)]
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VolumeMount {
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "mountPath")]
    pub mount_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PodStatus {
    #[serde(default)]
    pub phase: String,
    #[serde(default, rename = "containerStatuses")]
    pub container_statuses: Vec<ContainerStatus>,
    #[serde(default, rename = "podIP")]
    pub pod_ip: Option<String>,
    #[serde(default, rename = "startTime")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerStatus {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub ready: bool,
    #[serde(default, rename = "restartCount")]
    pub restart_count: i32,
    #[serde(default)]
    pub state: Option<ContainerState>,
    #[serde(default)]
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerState {
    #[serde(default)]
    pub running: Option<ContainerStateRunning>,
    #[serde(default)]
    pub waiting: Option<ContainerStateWaiting>,
    #[serde(default)]
    pub terminated: Option<ContainerStateTerminated>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerStateRunning {
    #[serde(default, rename = "startedAt")]
    pub started_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerStateWaiting {
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerStateTerminated {
    #[serde(default, rename = "exitCode")]
    pub exit_code: i32,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PodList {
    #[serde(default)]
    pub items: Vec<Pod>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Node {
    #[serde(default)]
    pub metadata: NodeMetadata,
    #[serde(default)]
    pub status: Option<NodeStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeMetadata {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeStatus {
    #[serde(default)]
    pub conditions: Vec<NodeCondition>,
    #[serde(default, rename = "nodeInfo")]
    pub node_info: Option<NodeInfo>,
    #[serde(default)]
    pub capacity: HashMap<String, String>,
    #[serde(default)]
    pub allocatable: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeCondition {
    #[serde(default, rename = "type")]
    pub condition_type: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeInfo {
    #[serde(default)]
    pub architecture: String,
    #[serde(default, rename = "operatingSystem")]
    pub operating_system: String,
    #[serde(default, rename = "kernelVersion")]
    pub kernel_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeList {
    #[serde(default)]
    pub items: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Event {
    #[serde(default)]
    pub metadata: EventMetadata,
    #[serde(default, rename = "type")]
    pub event_type: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub message: String,
    #[serde(default, rename = "involvedObject")]
    pub involved_object: Option<InvolvedObject>,
    #[serde(default, rename = "firstTimestamp")]
    pub first_timestamp: Option<String>,
    #[serde(default, rename = "lastTimestamp")]
    pub last_timestamp: Option<String>,
    #[serde(default)]
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventMetadata {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvolvedObject {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventList {
    #[serde(default)]
    pub items: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsistencyResult {
    #[serde(default)]
    pub checks: Vec<ConsistencyCheck>,
    #[serde(default)]
    pub summary: Option<ConsistencySummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsistencyCheck {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsistencySummary {
    #[serde(default)]
    pub pass: i32,
    #[serde(default)]
    pub warn: i32,
    #[serde(default)]
    pub fail: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Network {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub bridge: String,
    #[serde(default)]
    pub cidr: String,
    #[serde(default)]
    pub gateway: String,
    #[serde(default)]
    pub dns: String,
    #[serde(default, rename = "dnsZone")]
    pub dns_zone: String,
    #[serde(default, rename = "dnsEndpoint")]
    pub dns_endpoint: String,
    #[serde(default, rename = "ipamStart")]
    pub ipam_start: String,
    #[serde(default, rename = "ipamEnd")]
    pub ipam_end: String,
    #[serde(default, rename = "externalDNS")]
    pub external_dns: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageInfo {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub digest: String,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default, rename = "lastUpdated")]
    pub last_updated: Option<String>,
    #[serde(default, rename = "inUse")]
    pub in_use: bool,
    #[serde(default, rename = "usedBy")]
    pub used_by: Vec<String>,
}

impl MkubeClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn list_pods(&self) -> Result<Vec<Pod>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/pods", self.base_url))
            .send()
            .await?;
        let list: PodList = resp.json().await.unwrap_or_default();
        Ok(list.items)
    }

    pub async fn list_pods_in_namespace(&self, namespace: &str) -> Result<Vec<Pod>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/namespaces/{}/pods", self.base_url, namespace))
            .send()
            .await?;
        let list: PodList = resp.json().await.unwrap_or_default();
        Ok(list.items)
    }

    pub async fn get_pod(&self, namespace: &str, name: &str) -> Result<Pod, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/namespaces/{}/pods/{}", self.base_url, namespace, name))
            .send()
            .await?;
        let pod: Pod = resp.json().await.unwrap_or_default();
        Ok(pod)
    }

    pub async fn delete_pod(&self, namespace: &str, name: &str) -> Result<(), reqwest::Error> {
        self.client
            .delete(format!("{}/api/v1/namespaces/{}/pods/{}", self.base_url, namespace, name))
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_pod_logs(&self, namespace: &str, name: &str, container: &str, tail: Option<i32>) -> Result<String, reqwest::Error> {
        let mut url = format!(
            "{}/api/v1/namespaces/{}/pods/{}/log?container={}",
            self.base_url, namespace, name, container
        );
        if let Some(t) = tail {
            url.push_str(&format!("&tailLines={}", t));
        }
        let resp = self.client.get(&url).send().await?;
        let text = resp.text().await.unwrap_or_default();
        Ok(text)
    }

    pub async fn list_nodes(&self) -> Result<Vec<Node>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/nodes", self.base_url))
            .send()
            .await?;
        let list: NodeList = resp.json().await.unwrap_or_default();
        Ok(list.items)
    }

    pub async fn get_node(&self, name: &str) -> Result<Node, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/nodes/{}", self.base_url, name))
            .send()
            .await?;
        let node: Node = resp.json().await.unwrap_or_default();
        Ok(node)
    }

    pub async fn list_events(&self) -> Result<Vec<Event>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/events", self.base_url))
            .send()
            .await?;
        let list: EventList = resp.json().await.unwrap_or_default();
        Ok(list.items)
    }

    pub async fn get_consistency(&self) -> Result<ConsistencyResult, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/consistency", self.base_url))
            .send()
            .await?;
        let result: ConsistencyResult = resp.json().await.unwrap_or_default();
        Ok(result)
    }

    pub async fn list_networks(&self) -> Result<Vec<Network>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/networks", self.base_url))
            .send()
            .await?;
        let networks: Vec<Network> = resp.json().await.unwrap_or_default();
        Ok(networks)
    }

    pub async fn get_network(&self, name: &str) -> Result<Network, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/networks/{}", self.base_url, name))
            .send()
            .await?;
        let network: Network = resp.json().await.unwrap_or_default();
        Ok(network)
    }

    pub async fn list_images(&self) -> Result<Vec<ImageInfo>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/images", self.base_url))
            .send()
            .await?;
        let images: Vec<ImageInfo> = resp.json().await.unwrap_or_default();
        Ok(images)
    }
}
