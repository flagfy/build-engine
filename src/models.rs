use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildInput {
    pub project: String,
    pub globals: String,
    pub contexts: Vec<FileEntry>,
    pub segments: Vec<FileEntry>,
    pub decisions: Vec<FileEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectSpec {
    pub spec: String,
    pub info: ProjectInfo,
    pub project: ProjectSource,
    pub providers: HashMap<String, ProviderSpec>,
    pub state: StateSpec,
    pub versioning: VersionSpec,
    pub publish: PublishSpec,
    pub bootstrap: BootstrapSpec,
    pub policies: PolicySpec,
    pub environments: HashMap<String, EnvironmentSpec>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectInfo {
    pub title: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectSource {
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderSpec {
    pub driver: String,
    pub config: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateSpec {
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionSpec {
    pub strategy: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishSpec {
    pub provider: String,
    pub path: String,
    pub strategy: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BootstrapSpec {
    pub provider: String,
    pub pointer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PolicySpec {
    #[serde(rename = "strictSchema")]
    pub strict_schema: bool,
    #[serde(rename = "immutablePublish")]
    pub immutable_publish: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentSpec {
    pub overrides: EnvironmentOverrides,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvironmentOverrides {
    pub publish: Option<PublishOverride>,
    pub bootstrap: Option<BootstrapOverride>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublishOverride {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BootstrapOverride {
    pub pointer: String,
}

#[derive(Debug, Clone)]
pub struct FileMeta {
    pub environments: EnvironmentFilter,
}

#[derive(Debug, Clone)]
pub enum EnvironmentFilter {
    All,
    List(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct ParsedEntry {
    pub name: String,
    pub meta: FileMeta,
    pub content: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvBundle {
    pub core_version: String,
    pub contexts: HashMap<String, serde_json::Value>,
    pub segments: HashMap<String, serde_json::Value>,
    pub decisions: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvManifest {
    pub publish_path: String,
    pub bootstrap_pointer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuickCheck {
    pub bundle_uri: String,
    pub checksum: String,
    pub env_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootstrap_pointer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentOutput {
    pub bundle: EnvBundle,
    pub quick_check: QuickCheck,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildOutput {
    pub spec: String,
    pub generated_at: String,
    pub globals: HashMap<String, serde_json::Value>,
    pub active_environments: Vec<String>,
    pub files: HashMap<String, serde_json::Value>,
}
