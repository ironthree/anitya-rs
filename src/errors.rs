#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    /// failure to (de)serialize a JSON value
    ///
    /// If this error occurs, it is considered to be a bug in this crate.
    #[error("Failed to deserialize JSON response: {error}")]
    DeSerializationError {
        /// error returned by [`serde_json`]
        #[from]
        error: serde_json::Error,
    },
    /// request failed due to networking issues
    #[error("Failed to query bodhi service: {error}")]
    RequestError {
        /// error returned by [`reqwest`]
        #[from]
        error: reqwest::Error,
    },
    /// failure caused by an attempt to call authenticated API without token
    #[error("Unauthorized request: no API token supplied")]
    Unauthorized,
    /// failure to serialize x-www-urlencoded request string
    #[error("Failed to construct `x-www-urlencoded` query string: {error}")]
    UrlEncodedError {
        /// error returned by [`serde_url_params`]
        #[from]
        error: serde_url_params::Error,
    },
    /// error parsing a string into a URL
    #[error("Failed to compute request URL: {error}")]
    UrlParsingError {
        /// error returned from [`url`]
        #[from]
        error: url::ParseError,
    },
}
