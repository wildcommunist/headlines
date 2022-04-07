use serde::Deserialize;
use url::Url;

#[cfg(feature = "async")]
use reqwest::Method;

const BASE_URL: &str = "https://newsapi.org/v2/";

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed to fetch articles")]
    RequestFailed(#[from] ureq::Error),

    #[error("Failed to convert response to string")]
    ConversionFailed(#[from] std::io::Error),

    #[error("Atricle parsing failed")]
    ResponseJsonFailed(#[from] serde_json::Error),

    #[error("Failed to parse URL")]
    URLParseError(#[from] url::ParseError),

    #[error("request failed: {0}")]
    BadRequest(&'static str),

    #[error("Failed to fetch async url")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

pub struct NewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

pub enum Endpoint {
    TopHeadlines
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string()
        }
    }
}

pub enum Country {
    US {},
    AU {},
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::US {} => "us".to_string(),
            Self::AU {} => "au".to_string()
        }
    }
}

impl NewsAPI {
    pub fn new(key: &str) -> Self {
        NewsAPI {
            api_key: key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::AU {},
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsAPI {
        self.endpoint = endpoint;
        self
    }

    pub fn country(&mut self, country: Country) -> &mut NewsAPI {
        self.country = country;
        self
    }

    pub fn prepare_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());

        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }

    pub fn fetch(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let req = ureq::get(&url).set("Authorization", &self.api_key);
        let response: NewsAPIResponse = req.call()?.into_json()?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_error(response.code))
        }
    }

    #[cfg(feature = "async")]
    pub async fn fetch_async(&self) -> Result<NewsAPIResponse, NewsApiError> {
        let url = self.prepare_url()?;
        let client = reqwest::Client::new();
        let request = client.request(Method::GET, url)
            .header("Authorization", &self.api_key)
            .build()
            .map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        let response: NewsAPIResponse = client.execute(request).await?.json().await.
            map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_error(response.code))
        }
    }
}

fn map_response_error(ec: Option<String>) -> NewsApiError {
    if let Some(code) = ec {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your key has been disabled"),
            _ => NewsApiError::BadRequest("Unhabdled error")
        }
    } else {
        NewsApiError::BadRequest("Unknown error")
    }
}

#[derive(Deserialize, Debug)]
pub struct NewsAPIResponse {
    status: String,
    pub articles: Vec<Article>,
    code: Option<String>,
}

impl NewsAPIResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Deserialize, Debug)]
pub struct Article {
    author: Option<String>,
    title: String,
    url: String,
    content: Option<String>,
    description: Option<String>,
    url_to_image: Option<String>,
}

impl Article {
    pub fn Author(&self) -> &Option<String> {
        &self.author
    }

    pub fn Title(&self) -> &str {
        &self.title
    }

    pub fn URL(&self) -> &str {
        &self.url
    }
    pub fn Content(&self) -> &Option<String> {
        &self.content
    }
    pub fn Description(&self) -> &Option<String> {
        &self.description
    }
    pub fn Image(&self) -> &Option<String> {
        &self.url_to_image
    }
}