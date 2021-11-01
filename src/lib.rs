use std::fmt;

mod args;
mod error;
mod model;

use args::DefinitionsArgs;
use model::{Definition, Etymology};

use crate::args::Args;

static USER_AGENT: &str = concat!("wordnik rust client v", env!("CARGO_PKG_VERSION"));
static API_BASE: &str = "https://api.wordnik.com/v4";

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct Client {
    inner: reqwest::blocking::Client,
    api_key: String,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("inner", &self.inner)
            .field("api_key", &"<api_key>")
            .finish()
    }
}

#[cfg(test)]
impl Client {
    fn test_client() -> Self {
        dotenv::dotenv().ok();
        Self {
            inner: build_inner_client().unwrap(),
            api_key: dotenv::var("API_KEY").unwrap(),
        }
    }
}

impl Client {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            inner: build_inner_client().unwrap(),
            api_key: api_key.into(),
        }
    }

    // get /word.json/{word}/audio
    // get /word.json/{word}/examples
    // get /word.json/{word}/frequency
    // get /word.json/{word}/hyphenation
    // get /word.json/{word}/phrases
    // get /word.json/{word}/pronunciations
    // get /word.json/{word}/relatedWords
    // get /word.json/{word}/scrabbleScore
    // get /word.json/{word}/topExample

    // get /word.json/{word}/definitions
    pub fn definitions(&self, word: &str) -> Result<Vec<Definition>> {
        let url = format!(
            "{}/word.json/{}/definitions?limit=200&includeRelated=true&api_key={}",
            API_BASE, word, self.api_key
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
    }

    // get /word.json/{word}/definitions
    pub fn definitions_with_args(
        &self,
        word: &str,
        params: &DefinitionsArgs,
    ) -> Result<Vec<Definition>> {
        let url = format!(
            "{}/word.json/{}/definitions?api_key={}&{}",
            API_BASE,
            word,
            self.api_key,
            &params.to_urlencoded(),
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
    }

    // get /word.json/{word}/etymologies
    pub fn etymologies(&self, word: &str) -> Result<Vec<Etymology>> {
        let url = format!(
            "{}/word.json/{}/etymologies?api_key={}",
            API_BASE, word, self.api_key
        );
        let _request = self.inner.get(&url);

        todo!("for some stupid reason, this returns an XML blob")
    }
}

#[inline]
fn build_inner_client() -> reqwest::Result<reqwest::blocking::Client> {
    use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .user_agent(USER_AGENT)
        .build()
}

#[cfg(test)]
mod tests {
    #[test]
    fn user_agent_is_correct() {
        assert_eq!(super::USER_AGENT, "wordnik rust client v0.1.0");
    }

    #[test]
    fn can_create_test_client() {
        let client = super::Client::test_client();
        assert!(!client.api_key.is_empty());
    }

    #[test]
    fn can_request_definition() {
        let client = super::Client::test_client();
        assert!(dbg!(client.definitions("fireplace")).is_ok())
    }

    #[test]
    fn can_request_etymology() {
        let client = super::Client::test_client();
        assert!(dbg!(client.etymologies("horse")).is_ok());
    }
}
