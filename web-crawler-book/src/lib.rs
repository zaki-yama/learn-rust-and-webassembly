use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use thiserror::Error;
use url::ParseError as UrlParseError;
use url::Url;

#[derive(Debug, Error)]
pub enum GetLinksError {
    #[error("Failed to send a request")]
    SendRequest(#[source] reqwest::Error),
    #[error("Failed to read the response body")]
    ResponseBody(#[source] reqwest::Error),
    #[error("Failed to make the link URL absolute")]
    AbsolutizeUrl(#[source] url::ParseError),
    #[error("Server returned an error")]
    ServerError(#[source] reqwest::Error),
}

pub struct LinkExtractor {
    client: Client,
}

impl LinkExtractor {
    pub fn from_client(client: Client) -> Self {
        Self { client }
    }

    pub fn get_links(&self, url: Url) -> Result<Vec<Url>, GetLinksError> {
        log::info!("GET \"{}\"", url);
        let response = self
            .client
            .get(url)
            .send()
            .map_err(|e| GetLinksError::SendRequest(e))?;
        let response = response
            .error_for_status()
            .map_err(|e| GetLinksError::ServerError(e))?;
        let base_url = response.url().clone();
        let status = response.status();
        let body = response
            .text()
            .map_err(|e| GetLinksError::ResponseBody(e))?;
        let doc = Document::from(body.as_str());

        let mut links = Vec::new();
        log::info!("Retrieved {} \"{}\"", status, base_url);
        for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
            match Url::parse(href) {
                Ok(url) => {
                    links.push(url);
                }
                Err(UrlParseError::RelativeUrlWithoutBase) => {
                    // `href` を絶対URLに変換する
                    let url = base_url
                        .join(href)
                        .map_err(|e| GetLinksError::AbsolutizeUrl(e))?;
                    links.push(url);
                }
                Err(e) => {
                    println!("Error:{}", e);
                }
            };
        }
        Ok(links)
    }
}
