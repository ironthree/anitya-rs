use reqwest::header::{HeaderValue, USER_AGENT};
use reqwest::{header::HeaderMap, Client};
use serde::de::DeserializeOwned;
use url::Url;

use crate::errors::QueryError;
use crate::request::SingleRequest;

#[derive(Debug)]
pub struct AnityaClientBuilder<'a> {
    url: &'a str,
}

impl<'a> AnityaClientBuilder<'a> {
    pub fn new(url: &'a str) -> Self {
        AnityaClientBuilder { url }
    }

    pub fn build(self) -> Result<AnityaClient, url::ParseError> {
        let url = Url::parse(self.url)?;
        let user_agent = "anitya-rs";

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(user_agent));

        let session = reqwest::ClientBuilder::new()
            .default_headers(headers)
            //.timeout()
            .build()
            .expect("Failed to initialize the network stack.");

        Ok(AnityaClient { url, session })
    }
}

#[derive(Debug)]
pub struct AnityaClient {
    url: Url,
    session: Client,
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

        let response = match request.body()? {
            Some(body) => self.session.get(url).body(body).send().await,
            None => self.session.get(url).send().await,
        }?;

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
