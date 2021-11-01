pub mod parameters;
mod error;
mod model;

use std::fmt;
use model::*;
use parameters::*;

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

    // Word API endpoint //

    // get /word.json/{word}/audio
    
    // get /word.json/{word}/definitions
    pub fn definitions(&self, word: &str, args: Option<DefinitionArgs>) -> Result<Vec<Definition>> {
        let args = args.unwrap_or(DefinitionArgs::default()); 
        let url = format!(
            "{}/word.json/{}/definitions?api_key={}{}",
            API_BASE, word, self.api_key, args.to_uri()
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
    }
    
    // get /word.json/{word}/etymologies
    pub fn etymologies(&self, word: &str, args: Option<EtymologiesArgs>) -> Result<Vec<Etymology>> {
        let args = args.unwrap_or(EtymologiesArgs::default());
        let url = format!(
            "{}/word.json/{}/etymologies?api_key={}{}",
            API_BASE, word, self.api_key, args.to_uri()
        );
        let _request = self.inner.get(&url);
        
        todo!("for some stupid reason, this returns an XML blob")
    }

    // get /word.json/{word}/examples
    // get /word.json/{word}/frequency
    // get /word.json/{word}/hyphenation
    // get /word.json/{word}/phrases
    // get /word.json/{word}/pronunciations
    // get /word.json/{word}/relatedWords
    // get /word.json/{word}/scrabbleScore
    // get /word.json/{word}/topExample

    // Words API endpoint //

    // get /words.json/randomWord
    pub fn random_word(&self, args: Option<RandomWordArgs>) -> Result<RandomWord> {
        let args = args.unwrap_or(RandomWordArgs::default()); 
        let url = format!(
           "{}/words.json/randomWord?api_key={}{}",
           API_BASE, self.api_key, args.to_uri()
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
    }

    // get /words.json/randomWords
    pub fn random_words(&self, args: Option<RandomWordsArgs>) -> Result<Vec<RandomWord>> {
        let args = args.unwrap_or(RandomWordsArgs::default()); 
        let url = format!(
           "{}/words.json/randomWords?api_key={}{}",
           API_BASE, self.api_key, args.to_uri()
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
    }

    // get /words.json/reverseDictionary
    // get /words.json/search/{query} (!! Deprecated for wordnik api v4 !!)
    // get /words.json/wordOfTheDay 
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
        assert!(dbg!(client.definitions("fireplace", None)).is_ok())
    }

    #[test]
    fn can_request_random_word() {
        let client = super::Client::test_client();
        assert!(dbg!(client.random_word(None)).is_ok())
    }

    #[test]
    fn can_request_random_words() {
        let client = super::Client::test_client();
        assert!(dbg!(client.random_words(None)).is_ok())
    }

    #[test]
    fn can_request_etymology() {
        let client = super::Client::test_client();
        assert!(dbg!(client.etymologies("horse", None)).is_ok());
    }
}
