use serde::{Deserialize, Serialize};

use crate::errors::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

const DEFAULT_PER_PAGE: u32 = 25;

#[derive(Debug, Serialize)]
pub struct NewProjectRequest {
    name: String,
    homepage: String,
    backend: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    version_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    insecure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    check_release: Option<bool>,
}

impl NewProjectRequest {
    pub fn new(name: String, homepage: String, backend: String) -> Self {
        NewProjectRequest {
            name,
            homepage,
            backend,

            version_url: None,
            version_prefix: None,
            regex: None,
            insecure: None,
            check_release: None,
        }
    }

    pub fn version_url(mut self, version_url: String) -> Self {
        self.version_url = Some(version_url);
        self
    }

    pub fn version_prefix(mut self, version_prefix: String) -> Self {
        self.version_prefix = Some(version_prefix);
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

    pub fn check_release(mut self, check_release: bool) -> Self {
        self.check_release = Some(check_release);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct NewProject {
    pub backend: String,
    pub created_on: f64,
    pub homepage: String,
    pub id: u32,
    pub name: String,
    pub regex: Option<String>,
    pub updated_on: f64,
    pub version: Option<String>,
    pub version_url: Option<String>,
    pub versions: Vec<String>,
    pub stable_versions: Vec<String>,
}

impl SingleRequest<NewProject, NewProject> for NewProjectRequest {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/api/v2/projects/"))
    }

    fn body(&self) -> Result<Option<String>, QueryError> {
        Ok(Some(serde_json::to_string_pretty(self)?))
    }

    fn parse(&self, string: &str) -> Result<NewProject, QueryError> {
        Ok(serde_json::from_str(string)?)
    }

    fn extract(&self, page: NewProject) -> NewProject {
        page
    }
}

#[derive(Debug)]
pub struct ProjectQuery {
    ecosystem: Option<String>,
    name: Option<String>,
    items_per_page: u32,
}

impl ProjectQuery {
    pub fn new() -> Self {
        ProjectQuery {
            ecosystem: None,
            name: None,
            items_per_page: DEFAULT_PER_PAGE,
        }
    }
}

impl Default for ProjectQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectQuery {
    pub fn ecosystem(mut self, ecosystem: String) -> Self {
        self.ecosystem = Some(ecosystem);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn items_per_page(mut self, items_per_page: u32) -> Self {
        self.items_per_page = items_per_page;
        self
    }
}

#[derive(Debug, Serialize)]
pub struct ProjectPageQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ecosystem: Option<&'a String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a String>,

    page: u32,
    items_per_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct ProjectPage {
    items: Vec<Project>,

    #[allow(unused)]
    page: u32,
    items_per_page: u32,
    total_items: u32,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub backend: String,
    pub created_on: f64,
    pub ecosystem: String,
    pub homepage: String,
    pub id: u32,
    pub name: String,
    pub regex: Option<String>,
    pub updated_on: f64,
    pub version: String,
    pub version_url: Option<String>,
    pub versions: Vec<String>,
    pub stable_versions: Vec<String>,
}

impl<'a> SingleRequest<ProjectPage, Vec<Project>> for ProjectPageQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/api/v2/projects/?{}", serde_url_params::to_string(self)?))
    }

    fn body(&self) -> Result<Option<String>, QueryError> {
        Ok(None)
    }

    fn parse(&self, string: &str) -> Result<ProjectPage, QueryError> {
        Ok(serde_json::from_str(string)?)
    }

    fn extract(&self, page: ProjectPage) -> Vec<Project> {
        page.items
    }
}

impl Pagination for ProjectPage {
    fn pages(&self) -> u32 {
        super::num_pages(self.total_items, self.items_per_page)
    }
}

impl<'a> PaginatedRequest<'a, ProjectPage, Vec<Project>, ProjectPageQuery<'a>> for ProjectQuery {
    fn page_request(&'a self, page: u32) -> ProjectPageQuery<'a> {
        ProjectPageQuery {
            ecosystem: self.ecosystem.as_ref(),
            name: self.name.as_ref(),
            page,
            items_per_page: self.items_per_page,
        }
    }

    fn callback(&'a self, page: u32, pages: u32) {
        log::debug!("Callback: Page {} of {}", page, pages);
    }
}
