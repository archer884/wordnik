// From https://github.com/archer884/wordnik/blob/feature/queries-with-args/src/args.rs

pub trait Args {
  type KeyValuePairs: Iterator<Item = (&'static str, String)>;

  fn args(&self) -> Self::KeyValuePairs;

  fn to_urlencoded(&self) -> String {
      let args = self.args();
      let mut buf = String::new();

      args.for_each(|(key, value)| {
          buf += "&";
          buf += &key;
          buf += "=";
          buf += &value;
      });

      buf
  }
}

// Definitions

pub struct DefinitionsArgs {
  pub limit: u32,
  pub part_of_speech: Vec<PartOfSpeech>,
  pub include_related: bool,
  pub source_dictionaries: Vec<SourceDictionaries>,
  pub use_canonical: bool,
  pub include_tags: bool,
}

impl DefinitionsArgs {
  const DEFAULT_LIMIT: u32 = 200;

  pub fn new() -> Self {
      Self {
          limit: Self::DEFAULT_LIMIT,
          part_of_speech: Vec::new(),
          include_related: false,
          source_dictionaries: Vec::new(),
          use_canonical: false,
          include_tags: false,
      }
  }
}

pub struct DefinitionsArgsIter<'a> {
  args: &'a DefinitionsArgs,
  idx: usize
}

impl<'a> Iterator for DefinitionsArgsIter<'a> {
  type Item = (&'static str, String);

  fn next(&mut self) -> Option<Self::Item> {
      

      // Each time this is called, we'll return the first non-default parameter
      loop {
          match self.idx {
              0 => {
                  self.idx += 1;
                  if self.args.limit != DefinitionsArgs::DEFAULT_LIMIT {
                      return Some(("limit", self.args.limit.to_string()));
                  }
              }
              1 => {
                  self.idx += 1;
                  if !self.args.part_of_speech.is_empty() {
                      return Some(("partOfSpeech", format_csv(&self.args.part_of_speech)));
                  }
              }
              2 => {
                  self.idx += 1;
                  if self.args.include_related {
                      return Some(("includeRelated", format_bool(self.args.include_related)));
                  }
              }
              3 => {
                  self.idx += 1;
                  if !self.args.source_dictionaries.is_empty() {
                      return Some((
                          "sourceDictionaries",
                          format_csv(&self.args.source_dictionaries),
                      ));
                  }
              }
              4 => {
                  self.idx += 1;
                  if self.args.use_canonical {
                      return Some(("useCanonical", format_bool(self.args.use_canonical)));
                  }
              }
              5 => {
                  self.idx += 1;
                  if self.args.include_tags {
                      return Some(("includeTags", format_bool(self.args.include_tags)));
                  }
              }
              _ => return None,
          }
      }
  }
}

impl<'a> Args for &'a DefinitionsArgs {
  type KeyValuePairs = DefinitionsArgsIter<'a>;

  fn args(&self) -> Self::KeyValuePairs {
      DefinitionsArgsIter {
        args: self,
        idx: 0
      }
  }
}



// RandomWord

pub struct RandomWordArgs {
  pub has_dictionary_def: bool,
  pub include_part_of_speech: Vec<PartOfSpeech>,
  pub exclude_part_of_speech: Vec<PartOfSpeech>,
  pub min_corpus_count: Option<u32>,
  pub max_corpus_count: i32,
  pub min_dictionary_count: u32,
  pub max_dictionary_count: i32,
  pub min_length: u32,
  pub max_length: i32,
}

impl RandomWordArgs {
    const DEFAULT_MIN: u32 = 1;
    const DEFAULT_MAX: i32 = -1;
    const DEFAULT_MIN_LENGTH: u32 = 5;

    pub fn new() -> Self {
      Self {
        has_dictionary_def: true,
        include_part_of_speech: Vec::new(),
        exclude_part_of_speech: Vec::new(),
        min_corpus_count: None,
        max_corpus_count: Self::DEFAULT_MAX,
        min_dictionary_count: Self::DEFAULT_MIN,
        max_dictionary_count: Self::DEFAULT_MAX,
        min_length: Self::DEFAULT_MIN_LENGTH,
        max_length: Self::DEFAULT_MAX,
      }
    }
}

pub struct RandomWordArgsIter<'a> {
  args: &'a RandomWordArgs,
  idx: usize
}

impl<'a> Iterator for RandomWordArgsIter<'a> {
  type Item = (&'static str, String);

  fn next(&mut self) -> Option<Self::Item> {
      loop {
          match self.idx {
              0 => {
                  self.idx += 1;
                  if !self.args.has_dictionary_def {
                    return Some(("hasDictionaryDef", self.args.has_dictionary_def.to_string()));
                  }
              }
              1 => {
                  self.idx += 1;
                  if !self.args.include_part_of_speech.is_empty() {
                      return Some(("includePartOfSpeech", format_csv(&self.args.include_part_of_speech)));
                  }
              }
              2 => {
                self.idx += 1;
                if !self.args.exclude_part_of_speech.is_empty() {
                    return Some(("excludePartOfSpeech", format_csv(&self.args.exclude_part_of_speech)));
                }
              }
              3 => {
                self.idx += 1;
                if self.args.min_corpus_count.is_some() {
                  return Some(("minCorpusCount", self.args.min_corpus_count.unwrap().to_string()));
                }
              }
              4 => {
                self.idx += 1;
                if self.args.max_corpus_count != RandomWordArgs::DEFAULT_MAX {
                  return Some(("maxCorpusCount", self.args.max_corpus_count.to_string()));
                }
              }
              5 => {
                self.idx += 1;
                if self.args.min_dictionary_count != RandomWordArgs::DEFAULT_MIN {
                  return Some(("minDictionaryCount", self.args.min_dictionary_count.to_string()));
                }
              }
              6 => {
                self.idx += 1;
                if self.args.max_dictionary_count != RandomWordArgs::DEFAULT_MAX {
                  return Some(("maxDictionaryCount", self.args.max_dictionary_count.to_string()));
                }
              }
              7 => {
                self.idx += 1;
                if self.args.min_length != RandomWordArgs::DEFAULT_MIN_LENGTH {
                  return Some(("minLength", self.args.min_length.to_string()));
                }
              }
              8 => {
                self.idx += 1;
                if self.args.max_length != RandomWordArgs::DEFAULT_MAX {
                  return Some(("maxLength", self.args.max_length.to_string()));
                }
              }
              _ => return None,
          }
      }
  }    
}

impl<'a> Args for &'a RandomWordArgs {
    type KeyValuePairs = RandomWordArgsIter<'a>;

    fn args(&self) -> Self::KeyValuePairs {
        RandomWordArgsIter {
          args: self,
          idx: 0
        }
    }
}

// RandomWords

pub struct RandomWordsArgs {
  pub has_dictionary_def: bool,
  pub include_part_of_speech: Vec<PartOfSpeech>,
  pub exclude_part_of_speech: Vec<PartOfSpeech>,
  pub min_corpus_count: Option<u32>,
  pub max_corpus_count: i32,
  pub min_dictionary_count: u32,
  pub max_dictionary_count: i32,
  pub min_length: u32,
  pub max_length: i32,

  // We set those two as options cause they are optionl parameters in the API
  pub sort_by: Option<SortType>,
  pub sort_order: Option<SortOrder>,
  
  pub limit: u32
}

impl RandomWordsArgs {
  const DEFAULT_MIN: u32 = 1;
  const DEFAULT_MAX: i32 = -1;
  const DEFAULT_MIN_LENGTH: u32 = 5;
  const DEFAULT_LIMIT: u32 = 10;

  pub fn new() -> Self {
    Self {
      has_dictionary_def: true,
      include_part_of_speech: Vec::new(),
      exclude_part_of_speech: Vec::new(),
      min_corpus_count: None,
      max_corpus_count: Self::DEFAULT_MAX,
      min_dictionary_count: Self::DEFAULT_MIN,
      max_dictionary_count: Self::DEFAULT_MAX,
      min_length: Self::DEFAULT_MIN_LENGTH,
      max_length: Self::DEFAULT_MAX,
      sort_by: None,
      sort_order: None,
      limit: Self::DEFAULT_LIMIT
    }
  }
}

pub struct RandomWordsArgsIter<'a> {
  args: &'a RandomWordsArgs,
  idx: usize
}

impl<'a> Iterator for RandomWordsArgsIter<'a> {
  type Item = (&'static str, String);

  fn next(&mut self) -> Option<Self::Item> {
      loop {
          match self.idx {
              0 => {
                  self.idx += 1;
                  if !self.args.has_dictionary_def {
                    return Some(("hasDictionaryDef", format_bool(self.args.has_dictionary_def)));
                  }
              }
              1 => {
                  self.idx += 1;
                  if !self.args.include_part_of_speech.is_empty() {
                      return Some(("includePartOfSpeech", format_csv(&self.args.include_part_of_speech)));
                  }
              }
              2 => {
                self.idx += 1;
                if !self.args.exclude_part_of_speech.is_empty() {
                    return Some(("excludePartOfSpeech", format_csv(&self.args.exclude_part_of_speech)));
                }
              }
              3 => {
                self.idx += 1;
                if self.args.min_corpus_count.is_some() {
                  return Some(("minCorpusCount", self.args.min_corpus_count.unwrap().to_string()));
                }
              }
              4 => {
                self.idx += 1;
                if self.args.max_corpus_count != RandomWordsArgs::DEFAULT_MAX {
                  return Some(("maxCorpusCount", self.args.max_corpus_count.to_string()));
                }
              }
              5 => {
                self.idx += 1;
                if self.args.min_dictionary_count != RandomWordsArgs::DEFAULT_MIN {
                  return Some(("minDictionaryCount", self.args.min_dictionary_count.to_string()));
                }
              }
              6 => {
                self.idx += 1;
                if self.args.max_dictionary_count != RandomWordsArgs::DEFAULT_MAX {
                  return Some(("maxDictionaryCount", self.args.max_dictionary_count.to_string()));
                }
              }
              7 => {
                self.idx += 1;
                if self.args.min_length != RandomWordsArgs::DEFAULT_MIN_LENGTH {
                  return Some(("minLength", self.args.min_length.to_string()));
                }
              }
              8 => {
                self.idx += 1;
                if self.args.max_length != RandomWordsArgs::DEFAULT_MAX {
                  return Some(("maxLength", self.args.max_length.to_string()));
                }
              }
              9 => {
                self.idx += 1;
                if self.args.sort_by.is_some() {
                  return Some(("sortBy", format_enum(&self.args.sort_by.unwrap())));
                }
              }
              10 => {
                self.idx += 1;
                if self.args.sort_order.is_some() {
                  return Some(("sortOrder", format_enum(&self.args.sort_order.unwrap())));
                }
              }
              11 => {
                self.idx += 1;
                if self.args.limit != RandomWordsArgs::DEFAULT_LIMIT {
                  return Some(("limit", self.args.limit.to_string()));
                }
              }
              _ => return None,
          }
      }
  }    
}

impl<'a> Args for &'a RandomWordsArgs {
  type KeyValuePairs = RandomWordsArgsIter<'a>;

  fn args(&self) -> Self::KeyValuePairs {
      RandomWordsArgsIter {
        args: self,
        idx: 0
      }
  }
}

// Special parameters/structs

#[derive(Copy, Clone, Debug)]
pub enum SortType {
    Alpha,
    Count
}

impl StringParam for SortType {
  fn as_str(self) -> &'static str {
    match self {
      SortType::Alpha => "alpha",
      SortType::Count => "count"
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub enum SortOrder {
    Asc,
    Desc
}

impl StringParam for SortOrder {
  fn as_str(self) -> &'static str {
    match self {
      SortOrder::Asc => "asc",
      SortOrder::Desc => "desc"
    }
  }
}

trait StringParam: Copy {
  fn as_str(self) -> &'static str;
}

#[derive(Copy, Clone, Debug)]
pub enum PartOfSpeech {
  Abbreviation,
  Adjective,
  Adverb,
  Affix,
  Article,
  AuxiliaryVerb, // auxiliary-verb
  Conjunction,
  DefiniteArticle, // definite-article
  FamilyName,      // family-name
  GivenName,       // given-name
  Idiom,
  Imperative,
  Interjection,
  Noun,
  NounPlural,      // noun-plural
  NounPossessitve, // noun-possessive
  PastParticiple,  // past-participle
  PhrasalPrefix,   // phrasal-prefix
  Preposition,
  Pronoun,
  ProperNoun,           // proper-noun
  ProperNounPlural,     //proper-noun-plural
  ProperNounPossessive, // proper-noun-possessive
  Suffix,
  Verb,
  VerbIntransitive, // verb-intransitive
  VerbTransitive,   // verb-transitive
}

impl StringParam for PartOfSpeech {
  fn as_str(self) -> &'static str {
    match self {
      PartOfSpeech::Abbreviation => "abbreviation",
      PartOfSpeech::Adjective => "adjective",
      PartOfSpeech::Adverb => "adverb",
      PartOfSpeech::Affix => "affix",
      PartOfSpeech::Article => "article",
      PartOfSpeech::AuxiliaryVerb => "auxiliary-verb",
      PartOfSpeech::Conjunction => "conjunction",
      PartOfSpeech::DefiniteArticle => "definite-article",
      PartOfSpeech::FamilyName => "family-name",
      PartOfSpeech::GivenName => "given-name",
      PartOfSpeech::Idiom => "idiom",
      PartOfSpeech::Imperative => "imperative",
      PartOfSpeech::Interjection => "interjection",
      PartOfSpeech::Noun => "noun",
      PartOfSpeech::NounPlural => "noun-plural",
      PartOfSpeech::NounPossessitve => "noun-possessive",
      PartOfSpeech::PastParticiple => "past-participle",
      PartOfSpeech::PhrasalPrefix => "phrasal-prefix",
      PartOfSpeech::Preposition => "preposition",
      PartOfSpeech::Pronoun => "pronoun",
      PartOfSpeech::ProperNoun => "proper-noun",
      PartOfSpeech::ProperNounPlural => "proper-noun-plural",
      PartOfSpeech::ProperNounPossessive => "proper-noun-possessive",
      PartOfSpeech::Suffix => "suffix",
      PartOfSpeech::Verb => "verb",
      PartOfSpeech::VerbIntransitive => "verb-intransitive",
      PartOfSpeech::VerbTransitive => "verb-transitive",
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub enum SourceDictionaries {
  All,
  AmericanHeritage,
  Century,
  Cmu,
  Macmillan,
  Webster,
  Wiktionary,
  Wordnet,
}

impl StringParam for SourceDictionaries {
  fn as_str(self) -> &'static str {
      match self {
          SourceDictionaries::All => "all",
          SourceDictionaries::AmericanHeritage => "ahd-5",
          SourceDictionaries::Century => "century",
          SourceDictionaries::Cmu => "cmu",
          SourceDictionaries::Macmillan => "macmillan",
          SourceDictionaries::Webster => "webster",
          SourceDictionaries::Wiktionary => "wiktionary",
          SourceDictionaries::Wordnet => "wordnet",
      }
  }
}

fn format_csv(params: &[impl StringParam]) -> String {
  if params.is_empty() {
      return String::new();
  }

  if params.len() == 1 {
      return params[0].as_str().to_string();
  }

  let mut buf = params[0].as_str().to_string();
  for param in &params[1..] {
      buf += ",";
      buf += param.as_str();
  }
  buf
}

fn format_enum(param: &impl StringParam) -> String {
  return param.as_str().to_string();
}

fn format_bool(param: bool) -> String {
  if param {
      String::from("true")
  } else {
      String::from("false")
  }
}