use serde::{Deserialize, Serialize};

use crate::errors::QueryError;
use crate::request::{RequestMethod, SingleRequest};

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum FetchVersionsRequest {
    ForId(FetchForId),
    WithSearch(FetchWithSearch),
}

#[derive(Debug, Serialize)]
pub struct FetchForId {
    id: u32,
}

impl From<FetchForId> for FetchVersionsRequest {
    fn from(_: FetchForId) -> Self { todo!() }
}

#[derive(Debug, Default, Serialize)]
pub struct FetchWithSearch {
    // search parameters for existing projects
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

impl From<FetchWithSearch> for FetchVersionsRequest {
    fn from(_: FetchWithSearch) -> Self { todo!() }
}

impl FetchVersionsRequest {
    pub fn for_id(id: u32) -> FetchForId {
        FetchForId { id }
    }

    pub fn for_name(name: String) -> FetchWithSearch {
        FetchWithSearch {
            name: Some(name),
            ..Default::default()
        }
    }

    pub fn for_homepage(homepage: String) -> FetchWithSearch {
        FetchWithSearch {
            homepage: Some(homepage),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewVersions {
    pub found_versions: Vec<String>,
    pub latest_version: String,
    pub versions: Vec<String>,
    pub stable_versions: Vec<String>,
}

impl SingleRequest<NewVersions, NewVersions> for FetchVersionsRequest {
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
