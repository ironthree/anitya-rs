use serde::{Deserialize, Serialize};

use crate::errors::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

const DEFAULT_PER_PAGE: u32 = 25;

#[derive(Debug, Serialize)]
pub struct NewPackageRequest {
    distribution: String,
    package_name: String,
    project_ecosystem: String,
    project_name: String,
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

#[derive(Debug, Serialize)]
pub struct PackageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    distribution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    items_per_page: u32,
}

impl PackageQuery {
    pub fn new() -> Self {
        PackageQuery {
            distribution: None,
            name: None,
            items_per_page: DEFAULT_PER_PAGE,
        }
    }
}

impl Default for PackageQuery {
    fn default() -> Self {
        PackageQuery::new()
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
pub struct PackageListPage {
    items: Vec<Package>,

    #[allow(unused)]
    page: u32,
    items_per_page: u32,
    total_items: u32,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub distribution: String,
    pub ecosystem: String,
    pub name: String,
    pub project: String,
}

impl<'a> SingleRequest<PackageListPage, Vec<Package>> for PackagePageQuery<'a> {
    fn method(&self) -> RequestMethod {
        RequestMethod::GET
    }

    fn path(&self) -> Result<String, QueryError> {
        Ok(format!("/api/v2/packages/?{}", serde_url_params::to_string(self)?))
    }

    fn body(&self) -> Result<Option<String>, QueryError> {
        Ok(None)
    }

    fn parse(&self, string: &str) -> Result<PackageListPage, QueryError> {
        Ok(serde_json::from_str(string)?)
    }

    fn extract(&self, page: PackageListPage) -> Vec<Package> {
        page.items
    }
}

impl Pagination for PackageListPage {
    fn pages(&self) -> u32 {
        // https://doc.rust-lang.org/std/primitive.u32.html#method.div_ceil
        let div = self.total_items / self.items_per_page;
        let rem = self.total_items % self.items_per_page;

        if rem == 0 {
            div
        } else {
            div + 1
        }
    }
}

impl<'a> PaginatedRequest<'a, PackageListPage, Vec<Package>, PackagePageQuery<'a>> for PackageQuery {
    fn page_request(&'a self, page: u32) -> PackagePageQuery<'a> {
        PackagePageQuery {
            distribution: self.distribution.as_ref(),
            name: self.name.as_ref(),
            page,
            items_per_page: self.items_per_page,
        }
    }

    fn callback(&'a self, page: u32, pages: u32) {
        log::debug!("Callback: Page {} of {}", page, pages);
    }
}
