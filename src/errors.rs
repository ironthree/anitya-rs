#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    /// failure to (de)serialize a JSON value
    #[error("Failed to deserialize JSON response: {}", error)]
    DeSerialization {
        /// error returned by [`serde_json`]
        #[from]
        error: serde_json::Error,
    },
    /// failure to serialize x-www-urlencoded request string
    #[error("Failed to construct `x-www-urlencoded` query string: {error}")]
    InvalidQueryString {
        /// error returned by [`serde_url_params`]
        #[from]
        error: serde_url_params::Error,
    },
    /// error parsing a string into a URL
    #[error("Failed to compute request URL: {}", error)]
    InvalidURL {
        /// error returned from [`url`]
        #[from]
        error: url::ParseError,
    },
    /// request invalid or failed due to networking issues
    #[error("Failed to query anitya server: {}", error)]
    Networking {
        /// error returned by [`reqwest`]
        #[from]
        error: reqwest::Error,
    },
    /// server returned an error response to a request
    #[error("Server request resulted in an error: HTTP {} / {}", code, error)]
    Request { code: u16, error: String },
    /// failure caused by an attempt to call an authenticated API without a token
    #[error("Unauthorized request: no API token supplied")]
    Unauthorized,
}
