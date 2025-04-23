#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error occured: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error occured making the request: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API returned JSON that did not match expected type")]
    DeserializeJson(serde_json::Error, String),
    #[error("API returned an error status: {} ({} {})", .0.status, .0.method, .0.url)]
    Api(ApiError),
    #[error("API returned an unexpected content type")]
    ContentType(ApiError),
    #[error("{0}")]
    Custom(String),
}
#[derive(Debug, Clone)]
pub struct ApiError {
    pub method: String,
    pub url: reqwest::Url,
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
    pub content: bytes::Bytes,
}
impl ApiError {
    pub async fn new(method: &str, res: reqwest::Response) -> Self {
        let url = res.url().clone();
        let status = res.status();
        let headers = res.headers().clone();
        let content = res.bytes().await.unwrap_or_default();
        Self {
            status,
            headers,
            content,
            url,
            method: method.into(),
        }
    }
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.content)
    }
}
