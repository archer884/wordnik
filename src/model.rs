use std::collections::HashMap;
use serde::Deserialize;

// Of course, there are a boatload of properties here that I have no clue about.
// For now, I have skipped deserialization of several of these.

#[derive(Clone, Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Definition {
    pub part_of_speech: Option<String>,
    pub attribution_url: Option<String>,
    pub attribution_text: Option<String>,
    pub source_dictionary: Option<String>,
    pub text: Option<String>,

    // FIXME: same comment as with citations. >.< I mean, I have no idea what labels exist.
    pub labels: Vec<HashMap<String, String>>,

    // FIXME: I have no freaking clue why this isn't just a list of strings, but apparently this
    // property is implemented as a list of maps. Maps of what? No idea. Citations are rare enough
    // that I don't have a lot to go on, and the docs are unclear.
    pub citations: Vec<HashMap<String, String>>,

    pub word: String,
    #[serde(default = "Vec::new")]
    pub related_words: Vec<String>,
    #[serde(default = "Vec::new")]
    pub example_uses: Vec<String>,
    #[serde(default = "Vec::new")]
    pub notes: Vec<String>,
    pub wordnik_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename = "camelCase", transparent)]
pub struct Etymology {
    pub etymology: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct RandomWord {
    pub canonical_form: Option<String>,
    pub id: i32,
    pub original_word: Option<String>,
    #[serde(default = "Vec::new")]
    pub suggestions: Vec<String>,
    pub vulgar: Option<String>,
    pub word: String
}