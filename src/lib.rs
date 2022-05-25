mod args;
mod error;
mod model;

use std::fmt;

use args::{Args, DefinitionsArgs, RandomWordArgs, RandomWordsArgs};
use model::{Definition, Etymology, RandomWord};

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

    // Word API endpoint //

    // get /word.json/{word}/audio

    // get /word.json/{word}/definitions
    pub fn definitions(&self, word: &str) -> Result<Vec<Definition>> {
        let url = format!(
            "{}/word.json/{}/definitions?api_key={}",
            API_BASE, word, self.api_key
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
    }

    pub fn definitions_args(&self, word: &str, args: &DefinitionsArgs) -> Result<Vec<Definition>> {
        let url = format!(
            "{}/word.json/{}/definitions?api_key={}&{}",
            API_BASE,
            word,
            self.api_key,
            args.to_get_query_str()
        );
        let request = self.inner.get(&url);
        Ok(dbg!(request.send()?).json()?)
    }

    // get /word.json/{word}/etymologies
    pub fn etymologies(&self, word: &str) -> Result<Vec<Etymology>> {
        let url = format!(
            "{}/word.json/{}/etymologies?api_key={}",
            API_BASE, word, self.api_key
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
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
    pub fn random_word(&self) -> Result<RandomWord> {
        let url = format!(
            "{}/words.json/randomWord?api_key={}",
            API_BASE, self.api_key
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
    }

    pub fn random_word_args(&self, args: &RandomWordArgs) -> Result<RandomWord> {
        let url = format!(
            "{}/words.json/randomWord?api_key={}&{}",
            API_BASE,
            self.api_key,
            args.to_get_query_str()
        );
        let request = self.inner.get(&url);
        Ok(request.send()?.json()?)
    }

    // get /words.json/randomWords
    pub fn random_words(&self) -> Result<Vec<RandomWord>> {
        let url = format!(
            "{}/words.json/randomWords?api_key={}",
            API_BASE, self.api_key
        );

        Ok(self.inner.get(&url).send()?.json()?)
    }

    pub fn random_words_args(&self, args: &RandomWordsArgs) -> Result<Vec<RandomWord>> {
        let url = format!(
            "{}/words.json/randomWords?api_key={}&{}",
            API_BASE,
            self.api_key,
            args.to_get_query_str()
        );

        // I was wrong. I thought this came down as an xml blob, but it doesn't. No, sir: this
        // gets sent over the wire as a JSON array of escaped XML strings, for all have sinned
        // and fall short of the glory of God. I can't imagine what anyone would want this for,
        // but here it is.
        //
        // Dw, it's all good, not like we know when the strings are actually XML blobs in the API documentation anyway
        Ok(self.inner.get(&url).send()?.json()?)
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
    use crate::args::{DefinitionsArgs, PartOfSpeech, RandomWordArgs, RandomWordsArgs};

    #[test]
    fn user_agent_is_correct() {
        assert_eq!(super::USER_AGENT, "wordnik rust client v0.1.1");
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
    fn can_request_definition_with_args() {
        let client = super::Client::test_client();
        let mut args: DefinitionsArgs = DefinitionsArgs::new();
        args.limit = 1;
        let res = dbg!(client.definitions_args("test", &args));
        assert!(res.is_ok() && res.unwrap().len() == 1);
    }

    #[test]
    fn can_request_random_word() {
        let client = super::Client::test_client();
        assert!(dbg!(client.random_word()).is_ok())
    }

    #[test]
    fn can_request_random_word_with_args() {
        let client = super::Client::test_client();
        let mut args: RandomWordArgs = RandomWordArgs::new();
        args.include_part_of_speech.push(PartOfSpeech::Verb);
        args.min_length = 2;
        assert!(dbg!(client.random_word_args(&args)).is_ok())
    }

    #[test]
    fn can_request_random_words() {
        let client = super::Client::test_client();
        assert!(dbg!(client.random_words()).is_ok())
    }

    #[test]
    fn can_request_random_words_with_args() {
        let client = super::Client::test_client();
        let mut args: RandomWordsArgs = RandomWordsArgs::new();
        args.include_part_of_speech.push(PartOfSpeech::Noun);
        args.limit = 3;

        let res = dbg!(client.random_words_args(&args));
        assert!(res.is_ok() && res.unwrap().len() == 3);
    }

    #[test]
    fn can_request_etymology() {
        let client = super::Client::test_client();
        assert!(dbg!(client.etymologies("horse")).is_ok());
    }
}
