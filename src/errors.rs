#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    /// failure to deserialize a JSON response
    ///
    /// If this error occurs, it is considered to be a bug in this crate.
    #[error("Failed to deserialize JSON response: {error}")]
    DeserializationError {
        /// error returned by [`serde_json`]
        error: serde_json::Error,
    },
    /// request failed due to networking issues
    #[error("Failed to query bodhi service: {error}")]
    RequestError {
        /// error returned by [`reqwest`]
        #[from]
        error: reqwest::Error,
    },
    /// failure to serialize JSON request data
    ///
    /// If this error occurs, it is considered to be a bug in this crate.
    #[error("Failed to serialize POST request data: {error}")]
    SerializationError {
        /// error returned by [`serde_json`]
        error: serde_json::Error,
    },
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
