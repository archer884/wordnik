use std::fmt;

mod error;
mod model;

use model::{Definition, Etymology};

static API_BASE: &str = "https://api.wordnik.com/v4";
static USER_AGENT: &str = concat!("wordnik rust client v", env!("CARGO_PKG_VERSION"));

#[cfg(test)]
static WORDNIK_API_KEY_NAME: &str = "WORDNIK_API_KEY";

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
            api_key: dotenv::var(WORDNIK_API_KEY_NAME).unwrap(),
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
        // https://api.wordnik.com/v4/word.json/fishbed/definitions?limit=200&includeRelated=false&api_key=YOURAPIKEY
        let url = format!(
            "{}/word.json/{}/definitions?limit=200&includeRelated=true&api_key={}",
            API_BASE, word, self.api_key
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
    }
    
    // get /word.json/{word}/etymologies
    pub fn etymologies(&self, word: &str) -> Result<Vec<Etymology>> {
        // https://api.wordnik.com/v4/word.json/fireplace/etymologies?useCanonical=true&api_key=YOURAPIKEY
        let url = format!(
            "{}/word.json/{}/etymologies?api_key={}",
            API_BASE, word, self.api_key
        );

        // I was wrong. I thought this came down as an xml blob, but it doesn't. No, sir: this
        // gets sent over the wire as a JSON array of escaped XML strings, for all have sinned 
        // and fall short of the glory of God. I can't imagine what anyone would want this for,
        // but here it is.
        Ok(self.inner.get(&url).send()?.json()?)
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
