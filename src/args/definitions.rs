use super::{format_bool, format_csv, Args, PartOfSpeech, SourceDictionaries};

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

impl Default for DefinitionsArgs {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DefinitionsArgsIter<'a> {
    args: &'a DefinitionsArgs,
    idx: usize,
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

impl<'a> Args<'a> for DefinitionsArgs {
    type KeyValuePairs = DefinitionsArgsIter<'a>;

    fn args(&'a self) -> Self::KeyValuePairs {
        DefinitionsArgsIter { args: self, idx: 0 }
    }
}
