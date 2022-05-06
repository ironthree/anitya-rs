use serde::de::DeserializeOwned;

use crate::errors::QueryError;

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    GET,
    POST,
}

pub trait SingleRequest<P, T>
where
    T: DeserializeOwned,
{
    fn method(&self) -> RequestMethod;
    fn path(&self) -> Result<String, QueryError>;
    fn body(&self) -> Result<Option<String>, QueryError>;
    fn parse(&self, string: &str) -> Result<P, QueryError>;
    fn extract(&self, page: P) -> T;
}

pub trait PaginatedRequest<'a, P, T, S>
where
    P: Pagination,
    T: DeserializeOwned,
    S: SingleRequest<P, T> + 'a,
{
    fn page_request(&'a self, page: u32) -> S;
    fn callback(&'a self, page: u32, pages: u32);
}

pub trait Pagination {
    fn pages(&self) -> u32;
}
