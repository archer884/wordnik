mod definitions;
mod random;

pub use definitions::DefinitionsArgs;
pub use random::{RandomWordArgs, RandomWordsArgs};

pub trait Args<'a> {
    type KeyValuePairs: Iterator<Item = (&'static str, String)> + 'a;

    fn args(&'a self) -> Self::KeyValuePairs;

    fn to_get_query_str(&'a self) -> String {
        let mut args = self.args();
        let mut buf = match args.next() {
            Some((key, value)) => format!("{}={}", key, value),
            None => return String::new(),
        };

        args.for_each(|(key, value)| {
            buf += "&";
            buf += key;
            buf += "=";
            buf += &value;
        });

        buf
    }
}

// Special parameters/structs

#[derive(Copy, Clone, Debug)]
pub enum SortType {
    Alpha,
    Count,
}

impl StringParam for SortType {
    fn as_str(self) -> &'static str {
        match self {
            SortType::Alpha => "alpha",
            SortType::Count => "count",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl StringParam for SortOrder {
    fn as_str(self) -> &'static str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
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
    let mut params = params.iter();
    let mut buf = match params.next() {
        Some(first) => String::from(first.as_str()),
        None => return String::new(),
    };

    for param in params {
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
