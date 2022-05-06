use reqwest::header::{HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::{header::HeaderMap, header::InvalidHeaderValue, Client};
use serde::de::DeserializeOwned;
use thiserror::Error;
use url::Url;

use crate::errors::QueryError;
use crate::request::{RequestMethod, SingleRequest};

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("Failed to build client for invalid base URL: {}", error)]
    InvalidURL {
        #[from]
        error: url::ParseError,
    },
    #[error("Failed to build client with invalid API token: {}", error)]
    InvalidToken {
        #[from]
        error: InvalidHeaderValue,
    },
}

#[derive(Debug)]
pub struct AnityaClientBuilder<'a> {
    url: &'a str,
    token: Option<&'a str>,
}

impl<'a> AnityaClientBuilder<'a> {
    pub fn new(url: &'a str) -> Self {
        AnityaClientBuilder { url, token: None }
    }

    pub fn with_token(mut self, token: &'a str) -> Self {
        self.token = Some(token);
        self
    }

    pub fn build(self) -> Result<AnityaClient, BuilderError> {
        let url = Url::parse(self.url)?;
        let user_agent = "anitya-rs";

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));

        let session = reqwest::ClientBuilder::new()
            .default_headers(headers)
            //.timeout()
            .build()
            .expect("Failed to initialize the network stack.");

        let auth_header = if let Some(token) = self.token {
            let mut value = HeaderValue::from_str(token)?;
            value.set_sensitive(true);
            Some(value)
        } else {
            None
        };

        Ok(AnityaClient {
            url,
            session,
            auth_header,
        })
    }
}

#[derive(Debug)]
pub struct AnityaClient {
    url: Url,
    session: Client,
    auth_header: Option<HeaderValue>,
}

impl AnityaClient {
    pub async fn request<S, P, T>(&self, request: &S) -> Result<T, QueryError>
    where
        S: SingleRequest<P, T>,
        T: DeserializeOwned,
    {
        let url = self
            .url
            .join(&request.path()?)
            .map_err(|e| QueryError::UrlParsingError { error: e })?;

        let response = match request.method() {
            RequestMethod::GET => match request.body()? {
                Some(body) => self.session.get(url).body(body).send().await?,
                None => self.session.get(url).send().await?,
            },
            RequestMethod::POST => {
                let auth_header = if let Some(ref token) = self.auth_header {
                    token
                } else {
                    return Err(QueryError::Unauthorized);
                };

                let mut headers = HeaderMap::new();
                headers.insert(AUTHORIZATION, auth_header.clone());

                match request.body()? {
                    Some(body) => self.session.post(url).body(body).send().await?,
                    None => self.session.post(url).send().await?,
                }
            },
        };

        let status = response.status();

        if status.is_success() {
            let string = response.text().await?;
            let page = request.parse(&string)?;
            Ok(request.extract(page))
        } else {
            todo!()
        }
    }
}
