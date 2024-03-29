use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use crate::errors::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

const DEFAULT_PER_PAGE: u32 = 25;

#[derive(Debug, Serialize)]
pub struct NewPackageRequest {
    distribution: String,
    package_name: String,
    project_name: String,
    project_ecosystem: String,
}

impl NewPackageRequest {
    pub fn new(distribution: String, package_name: String, project_ecosystem: String, project_name: String) -> Self {
        NewPackageRequest {
            distribution,
            package_name,
            project_ecosystem,
            project_name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewPackage {
    pub distribution: String,
    pub name: String,
}

impl SingleRequest<NewPackage, NewPackage> for NewPackageRequest {
    fn method(&self) -> RequestMethod {
        RequestMethod::POST
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(String::from("/api/v2/packages/"))
    }

    fn body(&self) -> Result<Option<String>, QueryError> {
        Ok(Some(serde_json::to_string_pretty(self)?))
    }

    fn parse(&self, string: &str) -> Result<NewPackage, QueryError> {
        Ok(serde_json::from_str(string)?)
    }

    fn extract(&self, page: NewPackage) -> NewPackage {
        page
    }
}

pub struct PackageQuery {
    distribution: Option<String>,
    name: Option<String>,
    items_per_page: u32,
    callback: Option<Box<dyn Fn(u32, u32)>>,
}

impl PackageQuery {
    pub fn new() -> Self {
        PackageQuery {
            distribution: None,
            name: None,
            items_per_page: DEFAULT_PER_PAGE,
            callback: None,
        }
    }
}

impl Debug for PackageQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PackageQuery")
            .field("distribution", &self.distribution)
            .field("name", &self.name)
            .field("items_per_page", &self.items_per_page)
            .finish()
    }
}

impl Default for PackageQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageQuery {
    pub fn distribution(mut self, distribution: String) -> Self {
        self.distribution = Some(distribution);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn items_per_page(mut self, items_per_page: u32) -> Self {
        self.items_per_page = items_per_page.clamp(1, 250);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct PackagePageQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    distribution: Option<&'a String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a String>,

    page: u32,
    items_per_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct PackagePage {
    items: Vec<Package>,

    #[allow(unused)]
    page: u32,
    items_per_page: u32,
    total_items: u32,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub distribution: String,
    pub name: String,
    pub project: String,
    pub ecosystem: String,
    pub version: Option<String>,
}

impl<'a> SingleRequest<PackagePage, Vec<Package>> for PackagePageQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/api/v2/packages/?{}", serde_url_params::to_string(self)?))
    }

    fn body(&self) -> Result<Option<String>, QueryError> {
        Ok(None)
    }

    fn parse(&self, string: &str) -> Result<PackagePage, QueryError> {
        Ok(serde_json::from_str(string)?)
    }

    fn extract(&self, page: PackagePage) -> Vec<Package> {
        page.items
    }
}

impl Pagination for PackagePage {
    fn pages(&self) -> u32 {
        super::num_pages(self.total_items, self.items_per_page)
    }
}

impl<'a> PaginatedRequest<'a, PackagePage, Vec<Package>, PackagePageQuery<'a>> for PackageQuery {
    fn page_request(&'a self, page: u32) -> PackagePageQuery<'a> {
        PackagePageQuery {
            distribution: self.distribution.as_ref(),
            name: self.name.as_ref(),
            page,
            items_per_page: self.items_per_page,
        }
    }

    fn callback(&'a self, page: u32, pages: u32) {
        if let Some(ref callback) = &self.callback {
            callback(page, pages)
        }
    }
}
