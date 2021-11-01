pub trait Args {
    type KeyValuePairs: Iterator<Item = (&'static str, String)>;

    fn args(&self) -> Self::KeyValuePairs;

    fn to_urlencoded(&self) -> String {
        let mut args = self.args();
        let mut buf = String::new();

        if let Some((key, value)) = args.next() {
            buf += &key;
            buf += "=";
            buf += &value;
        }

        args.for_each(|(key, value)| {
            buf += "&";
            buf += &key;
            buf += "=";
            buf += &value;
        });

        buf
    }
}

pub struct DefinitionsArgs {
    pub limit: u32,
    pub part_of_speech: Vec<PartOfSpeech>,
    pub include_related: bool,
    pub source_dictionaries: Vec<SourceDictionaries>,
    pub use_canonical: bool,
    pub include_tags: bool,
}

impl DefinitionsArgs {
    pub fn new() -> Self {
        Self {
            limit: 200,
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
    idx: usize,
}

impl<'a> Iterator for DefinitionsArgsIter<'a> {
    type Item = (&'static str, String);

    fn next(&mut self) -> Option<Self::Item> {
        const DEFAULT_LIMIT: u32 = 200;

        // Each time this is called, we'll return the first non-default parameter
        loop {
            match self.idx {
                0 => {
                    self.idx += 1;
                    if self.args.limit != DEFAULT_LIMIT {
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
        todo!()
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

fn format_bool(param: bool) -> String {
    if param {
        String::from("true")
    } else {
        String::from("false")
    }
}
