// https://release-monitoring.org/static/docs/api.html

#![warn(missing_debug_implementations)]

mod client;
pub use client::{AnityaClient, ClientBuildError, ClientBuilder};

mod errors;
pub use errors::QueryError;

mod request;
pub use request::*;

// HTTP API v2
pub mod v2;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
