use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct DnsClient {
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Zone {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub record_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub ttl: i32,
    pub data: RecordData,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum RecordData {
    A(String),
    AAAA(String),
    CNAME(String),
    MX(MxData),
    NS(String),
    PTR(String),
    SRV(SrvData),
    TXT(String),
    CAA(CaaData),
}

impl RecordData {
    pub fn record_type(&self) -> &str {
        match self {
            RecordData::A(_) => "A",
            RecordData::AAAA(_) => "AAAA",
            RecordData::CNAME(_) => "CNAME",
            RecordData::MX(_) => "MX",
            RecordData::NS(_) => "NS",
            RecordData::PTR(_) => "PTR",
            RecordData::SRV(_) => "SRV",
            RecordData::TXT(_) => "TXT",
            RecordData::CAA(_) => "CAA",
        }
    }

    pub fn display_value(&self) -> String {
        match self {
            RecordData::A(v) | RecordData::AAAA(v) | RecordData::CNAME(v)
            | RecordData::NS(v) | RecordData::PTR(v) | RecordData::TXT(v) => v.clone(),
            RecordData::MX(mx) => format!("{} {}", mx.preference, mx.exchange),
            RecordData::SRV(srv) => format!("{} {} {} {}", srv.priority, srv.weight, srv.port, srv.target),
            RecordData::CAA(caa) => format!("{} {} \"{}\"", caa.flags, caa.tag, caa.value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MxData {
    #[serde(default)]
    pub preference: i32,
    #[serde(default)]
    pub exchange: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SrvData {
    #[serde(default)]
    pub priority: i32,
    #[serde(default)]
    pub weight: i32,
    #[serde(default)]
    pub port: i32,
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CaaData {
    #[serde(default)]
    pub flags: i32,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecordRequest {
    pub name: String,
    pub ttl: i32,
    pub data: RecordData,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRecordRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<RecordData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DnsEndpointInfo {
    pub name: String,
    pub url: String,
    pub zone: String,
    pub reachable: bool,
}

impl DnsClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn check_health(&self, base_url: &str) -> bool {
        self.client
            .get(format!("{}/api/v1/health", base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    pub async fn list_zones(&self, base_url: &str) -> Result<Vec<Zone>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/zones", base_url))
            .send()
            .await?;
        let zones: Vec<Zone> = resp.json().await.unwrap_or_default();
        Ok(zones)
    }

    pub async fn list_records(&self, base_url: &str, zone_id: &str) -> Result<Vec<DnsRecord>, reqwest::Error> {
        let resp = self.client
            .get(format!("{}/api/v1/zones/{}/records?limit=500", base_url, zone_id))
            .send()
            .await?;
        let records: Vec<DnsRecord> = resp.json().await.unwrap_or_default();
        Ok(records)
    }

    pub async fn create_record(&self, base_url: &str, zone_id: &str, req: &CreateRecordRequest) -> Result<DnsRecord, String> {
        let resp = self.client
            .post(format!("{}/api/v1/zones/{}/records", base_url, zone_id))
            .json(req)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if resp.status().is_success() {
            resp.json().await.map_err(|e| e.to_string())
        } else {
            Err(format!("HTTP {}: {}", resp.status(), resp.text().await.unwrap_or_default()))
        }
    }

    pub async fn update_record(&self, base_url: &str, zone_id: &str, record_id: &str, req: &UpdateRecordRequest) -> Result<DnsRecord, String> {
        let resp = self.client
            .put(format!("{}/api/v1/zones/{}/records/{}", base_url, zone_id, record_id))
            .json(req)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if resp.status().is_success() {
            resp.json().await.map_err(|e| e.to_string())
        } else {
            Err(format!("HTTP {}: {}", resp.status(), resp.text().await.unwrap_or_default()))
        }
    }

    pub async fn delete_record(&self, base_url: &str, zone_id: &str, record_id: &str) -> Result<(), String> {
        let resp = self.client
            .delete(format!("{}/api/v1/zones/{}/records/{}", base_url, zone_id, record_id))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("HTTP {}: {}", resp.status(), resp.text().await.unwrap_or_default()))
        }
    }
}
