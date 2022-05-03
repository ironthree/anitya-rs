use serde::{Deserialize, Serialize};

use crate::errors::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

const DEFAULT_PER_PAGE: u32 = 25;

#[derive(Debug, Default, Serialize)]
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
        Ok(Some(
            serde_json::to_string_pretty(self).map_err(|error| QueryError::SerializationError { error })?,
        ))
    }

    fn parse(&self, string: &str) -> Result<NewPackage, QueryError> {
        let new_package: NewPackage =
            serde_json::from_str(string).map_err(|error| QueryError::SerializationError { error })?;
        Ok(new_package)
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

#[derive(Debug, Serialize)]
pub struct PackagePageQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    distribution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    page: u32,
    items_per_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct PackageListPage {
    items: Vec<Package>,

    page: i32,
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

impl SingleRequest<PackageListPage, Vec<Package>> for PackagePageQuery {
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
        let package_page: PackageListPage =
            serde_json::from_str(string).map_err(|error| QueryError::DeserializationError { error })?;
        Ok(package_page)
    }

    fn extract(&self, page: PackageListPage) -> Vec<Package> {
        page.items
    }
}

impl Pagination for PackageListPage {
    fn pages(&self) -> u32 {
        // https://doc.rust-lang.org/std/primitive.u32.html#method.div_ceil
        let div = self.total_items / self.items_per_page;
        let rem = self.total_items - div * self.items_per_page;

        if rem == 0 {
            div
        } else {
            div + 1
        }
    }
}

impl PaginatedRequest<PackageListPage, Vec<Package>, PackagePageQuery> for PackageQuery {
    fn page_request(&self, page: u32) -> PackagePageQuery {
        PackagePageQuery {
            distribution: self.distribution.clone(),
            name: self.name.clone(),
            page,
            items_per_page: self.items_per_page,
        }
    }

    fn callback(&self, page: u32, pages: u32) {
        eprintln!("Callback: Page {} of {}", page, pages);
    }
}
