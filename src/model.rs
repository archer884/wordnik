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
    pub labels: Vec<String>,
    pub citations: Vec<String>,
    pub word: String,
    #[serde(default = "Vec::new")]
    pub related_words: Vec<String>,
    #[serde(default = "Vec::new")]
    pub example_uses: Vec<String>,
    pub notes: Vec<String>,
    pub wordnik_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename = "camelCase", transparent)]
pub struct Etymology {
    pub etymology: String,
}
