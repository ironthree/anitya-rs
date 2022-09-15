use serde::{Deserialize, Serialize};

use crate::errors::QueryError;
use crate::request::{RequestMethod, SingleRequest};

#[derive(Debug, Serialize)]
pub struct ModifyProjectRequest {
    // search parameters for existing projects
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<String>,

    // parameters for creating a temporary project
    #[serde(skip_serializing_if = "Option::is_none")]
    backend: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_scheme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pre_release_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    releases_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dry_run: Option<bool>,
}

impl ModifyProjectRequest {
    pub fn backend(mut self, backend: String) -> Self {
        self.backend = Some(backend);
        self
    }

    pub fn version_url(mut self, version_url: String) -> Self {
        self.version_url = Some(version_url);
        self
    }

    pub fn version_scheme(mut self, version_scheme: String) -> Self {
        self.version_scheme = Some(version_scheme);
        self
    }

    pub fn version_pattern(mut self, version_pattern: String) -> Self {
        self.version_pattern = Some(version_pattern);
        self
    }

    pub fn version_prefix(mut self, version_prefix: String) -> Self {
        self.version_prefix = Some(version_prefix);
        self
    }

    pub fn pre_release_filter(mut self, pre_release_filter: String) -> Self {
        self.pre_release_filter = Some(pre_release_filter);
        self
    }

    pub fn version_filter(mut self, version_filter: String) -> Self {
        self.version_filter = Some(version_filter);
        self
    }

    pub fn regex(mut self, regex: String) -> Self {
        self.regex = Some(regex);
        self
    }

    pub fn insecure(mut self, insecure: bool) -> Self {
        self.insecure = Some(insecure);
        self
    }

    pub fn releases_only(mut self, releases_only: bool) -> Self {
        self.releases_only = Some(releases_only);
        self
    }

    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = Some(dry_run);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct NewVersions {
    pub found_versions: Vec<String>,
    pub latest_version: String,
    pub versions: Vec<String>,
    pub stable_versions: Vec<String>,
}

impl SingleRequest<NewVersions, NewVersions> for ModifyProjectRequest {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/api/v2/versions/"))
    }

    fn body(&self) -> Result<Option<String>, QueryError> {
        Ok(Some(serde_json::to_string_pretty(self)?))
    }

    fn parse(&self, string: &str) -> Result<NewVersions, QueryError> {
        Ok(serde_json::from_str(string)?)
    }

    fn extract(&self, page: NewVersions) -> NewVersions {
        page
    }
}

#[derive(Debug, Serialize)]
pub struct VersionQuery {
    project_id: u32,
}

impl VersionQuery {
    pub fn new(project_id: u32) -> Self {
        VersionQuery { project_id }
    }
}

#[derive(Debug, Deserialize)]
pub struct Versions {
    pub latest_version: String,
    pub versions: Vec<String>,
    pub stable_versions: Vec<String>,
}

impl SingleRequest<Versions, Versions> for VersionQuery {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/api/v2/versions/?{}", serde_url_params::to_string(self)?))
    }

    fn body(&self) -> Result<Option<String>, QueryError> {
        Ok(None)
    }

    fn parse(&self, string: &str) -> Result<Versions, QueryError> {
        Ok(serde_json::from_str(string)?)
    }

    fn extract(&self, page: Versions) -> Versions {
        page
    }
}
