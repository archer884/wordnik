use serde::Deserialize;

// Of course, there are a boatload of properties here that I have no clue about.
// For now, I have skipped deserialization of several of these.

#[derive(Clone, Debug, Deserialize)]
#[serde(rename = "camelCase")]
pub struct Definition {
    part_of_speech: Option<String>,
    attribution_url: Option<String>,
    attribution_text: Option<String>,
    source_dictionary: Option<String>,
    text: String,
    labels: Vec<String>,
    citations: Vec<String>,
    word: String,
    #[serde(default = "Vec::new")]
    related_words: Vec<String>,
    #[serde(default = "Vec::new")]
    example_uses: Vec<String>,
    notes: Vec<String>,
    wordnik_url: Option<String>,
}
