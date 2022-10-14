use std::fmt::{Debug, Formatter};
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::Client;
use serde::de::DeserializeOwned;
use url::Url;

use crate::errors::QueryError;
use crate::request::{PaginatedRequest, Pagination, RequestMethod, SingleRequest};

pub struct ClientBuilder<'a> {
    url: &'a str,
    token: Option<&'a str>,
    delay: Option<Duration>,
}

impl<'a> Debug for ClientBuilder<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let token = self.token.as_ref().map(|_| "(Authorization token)");
        let delay = self.delay.as_ref().map(|_| "(Paginated query delay)");

        f.debug_struct("ClientBuilder")
            .field("url", &self.url)
            .field("token", &token)
            .field("delay", &delay)
            .finish()
    }
}

impl<'a> ClientBuilder<'a> {
    pub fn new(url: &'a str) -> Self {
        ClientBuilder {
            url,
            token: None,
            delay: None,
        }
    }

    pub fn with_token(mut self, token: &'a str) -> Self {
        self.token = Some(token);
        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn build(self) -> Result<AnityaClient, ClientBuildError> {
        let url = Url::parse(self.url)?;
        let user_agent = "anitya-rs";

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));

        let session = reqwest::ClientBuilder::new()
            .default_headers(headers)
            //.timeout()
            .build()?;

        let auth_header = if let Some(token) = self.token {
            let mut value = HeaderValue::from_str(&format!("token {token}"))?;
            value.set_sensitive(true);
            Some(value)
        } else {
            None
        };

        Ok(AnityaClient {
            url,
            session,
            auth_header,
            delay: self.delay,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ClientBuildError {
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
    #[error("Failed to initialize HTTP client: {}", error)]
    Initialization {
        #[from]
        error: reqwest::Error,
    },
}

pub struct AnityaClient {
    url: Url,
    session: Client,
    auth_header: Option<HeaderValue>,
    delay: Option<Duration>,
}

impl Debug for AnityaClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let auth_header = self.auth_header.as_ref().map(|_| "(Authorization header)");
        let delay = self.delay.as_ref().map(|_| "(Paginated query delay)");

        f.debug_struct("AnityaClient")
            .field("url", &self.url)
            .field("session", &self.session)
            .field("auth_header", &auth_header)
            .field("delay", &delay)
            .finish()
    }
}

impl AnityaClient {
    pub async fn request<R, P, T>(&self, request: &R) -> Result<T, QueryError>
    where
        R: SingleRequest<P, T>,
        T: DeserializeOwned,
    {
        let url = self.url.join(&request.path()?)?;

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
                    Some(body) => {
                        let request = self.session.post(url).headers(headers).body(body).build()?;
                        self.session.execute(request).await?
                    },
                    None => {
                        let request = self.session.post(url).headers(headers).build()?;
                        self.session.execute(request).await?
                    },
                }
            },
        };

        let status = response.status();

        if status.is_success() {
            let string = response.text().await?;
            let page = request.parse(&string)?;
            Ok(request.extract(page))
        } else {
            Err(QueryError::Request {
                code: status.as_u16(),
                error: response.text().await?,
            })
        }
    }

    pub async fn paginated_request<'a, R, S, P, V, T>(&self, request: &'a R) -> Result<Vec<T>, QueryError>
    where
        R: PaginatedRequest<'a, P, V, S>,
        S: SingleRequest<P, V> + 'a,
        P: Pagination,
        V: IntoIterator<Item = T> + DeserializeOwned,
        T: DeserializeOwned,
    {
        let mut results: Vec<T> = Vec::new();

        // initialize progress callback with "zero progress"
        request.callback(0, 1);

        let first_request = request.page_request(1);
        let first_page = self.page_request(&first_request).await?;

        let mut page = 2u32;
        let mut pages = first_page.pages();

        // update progress callback with actual total pages
        request.callback(1, pages);

        results.extend(first_request.extract(first_page));

        while page <= pages {
            let page_request = request.page_request(page);
            let next_page = self.page_request(&page_request).await?;

            request.callback(page, pages);

            page += 1;
            pages = next_page.pages();

            results.extend(page_request.extract(next_page));

            if let Some(delay) = self.delay {
                tokio::time::sleep(delay).await;
            }
        }

        Ok(results)
    }

    async fn page_request<R, P, T>(&self, request: &R) -> Result<P, QueryError>
    where
        R: SingleRequest<P, T>,
        P: Pagination,
        T: DeserializeOwned,
    {
        debug_assert!(request.method() == RequestMethod::GET);

        let url = self.url.join(&request.path()?)?;

        let response = match request.body()? {
            Some(body) => self.session.get(url).body(body).send().await?,
            None => self.session.get(url).send().await?,
        };

        let status = response.status();

        if status.is_success() {
            let string = response.text().await?;
            let page = request.parse(&string)?;
            Ok(page)
        } else {
            Err(QueryError::Request {
                code: status.as_u16(),
                error: response.text().await?,
            })
        }
    }
}
