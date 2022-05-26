use super::{format_bool, format_csv, format_enum, Args, PartOfSpeech, SortOrder, SortType};

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

impl Default for RandomWordArgs {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RandomWordArgsIter<'a> {
    args: &'a RandomWordArgs,
    idx: usize,
}

impl<'a> Iterator for RandomWordArgsIter<'a> {
    type Item = (&'static str, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.idx {
                0 => {
                    self.idx += 1;
                    if !self.args.has_dictionary_def {
                        return Some((
                            "hasDictionaryDef",
                            self.args.has_dictionary_def.to_string(),
                        ));
                    }
                }
                1 => {
                    self.idx += 1;
                    if !self.args.include_part_of_speech.is_empty() {
                        return Some((
                            "includePartOfSpeech",
                            format_csv(&self.args.include_part_of_speech),
                        ));
                    }
                }
                2 => {
                    self.idx += 1;
                    if !self.args.exclude_part_of_speech.is_empty() {
                        return Some((
                            "excludePartOfSpeech",
                            format_csv(&self.args.exclude_part_of_speech),
                        ));
                    }
                }
                3 => {
                    self.idx += 1;
                    if self.args.min_corpus_count.is_some() {
                        return Some((
                            "minCorpusCount",
                            self.args.min_corpus_count.unwrap().to_string(),
                        ));
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
                        return Some((
                            "minDictionaryCount",
                            self.args.min_dictionary_count.to_string(),
                        ));
                    }
                }
                6 => {
                    self.idx += 1;
                    if self.args.max_dictionary_count != RandomWordArgs::DEFAULT_MAX {
                        return Some((
                            "maxDictionaryCount",
                            self.args.max_dictionary_count.to_string(),
                        ));
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

impl<'a> Args<'a> for RandomWordArgs {
    type KeyValuePairs = RandomWordArgsIter<'a>;

    fn args(&'a self) -> Self::KeyValuePairs {
        RandomWordArgsIter { args: self, idx: 0 }
    }
}

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

    // We set these two as optional because they are optional parameters in the API
    pub sort_by: Option<SortType>,
    pub sort_order: Option<SortOrder>,

    pub limit: u32,
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
            limit: Self::DEFAULT_LIMIT,
        }
    }
}

impl Default for RandomWordsArgs {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RandomWordsArgsIter<'a> {
    args: &'a RandomWordsArgs,
    idx: usize,
}

impl<'a> Iterator for RandomWordsArgsIter<'a> {
    type Item = (&'static str, String);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.idx {
                0 => {
                    self.idx += 1;
                    if !self.args.has_dictionary_def {
                        return Some((
                            "hasDictionaryDef",
                            format_bool(self.args.has_dictionary_def),
                        ));
                    }
                }
                1 => {
                    self.idx += 1;
                    if !self.args.include_part_of_speech.is_empty() {
                        return Some((
                            "includePartOfSpeech",
                            format_csv(&self.args.include_part_of_speech),
                        ));
                    }
                }
                2 => {
                    self.idx += 1;
                    if !self.args.exclude_part_of_speech.is_empty() {
                        return Some((
                            "excludePartOfSpeech",
                            format_csv(&self.args.exclude_part_of_speech),
                        ));
                    }
                }
                3 => {
                    self.idx += 1;
                    if self.args.min_corpus_count.is_some() {
                        return Some((
                            "minCorpusCount",
                            self.args.min_corpus_count.unwrap().to_string(),
                        ));
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
                        return Some((
                            "minDictionaryCount",
                            self.args.min_dictionary_count.to_string(),
                        ));
                    }
                }
                6 => {
                    self.idx += 1;
                    if self.args.max_dictionary_count != RandomWordsArgs::DEFAULT_MAX {
                        return Some((
                            "maxDictionaryCount",
                            self.args.max_dictionary_count.to_string(),
                        ));
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

impl<'a> Args<'a> for RandomWordsArgs {
    type KeyValuePairs = RandomWordsArgsIter<'a>;

    fn args(&'a self) -> Self::KeyValuePairs {
        RandomWordsArgsIter { args: self, idx: 0 }
    }
}
