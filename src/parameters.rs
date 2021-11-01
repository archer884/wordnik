/// A macro rule to quickly write parameters struct for API calls with built-in uri getter.
/// 
/// Note: Since there are no camelCase to snake_case conversion, non_snake_case is allowed when using this macro.
/// 
/// Usage :
/// 
/// ```Rust
/// macro_args! {
///     struct_name,
///     [
///         optionalField1: Option<String> = Some("default value"),
///         optionalField2: Option<String> = None,
///     ], [
///         requiredField3: i32,
///         requiredField4: bool, 
///     ]
/// }
/// ```
macro_rules! macro_args{
    ($name:ident,
        [$($field_opt_name:ident: Option<$field_opt_type:ty> = $val_opt:expr,)*], 
        [$($field_name:ident: $field_type:ty,)*]
    ) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        pub struct $name {
            $(pub $field_opt_name: Option<$field_opt_type>,)*
            $(pub $field_name: $field_type,)*
        }

        impl $name {
            pub(crate) fn to_uri(&self) -> String {
                let mut uri = String::new();
                $(uri = format!("{}&{}={}", uri, stringify!($field_name), self.$field_name);)*
                $(if self.$field_opt_name.is_some() {
                    uri = format!("{}&{}={}", uri, stringify!($field_opt_name), self.$field_opt_name.as_ref().unwrap());
                })*
                return uri;
            }

            pub fn default($($field_name: $field_type)*) -> Self {
                Self {
                    $($field_opt_name: $val_opt,)*
                    $($field_name,)*
                }
            }
        }
    }
}

// Word API args //

macro_args! {
    DefinitionArgs, [
        limit: Option<i32> = Some(200),
        partOfSpeech: Option<bool> = None,
        includeRelated: Option<String> = None,
        sourceDictionaries: Option<String> = None,
        useCanonical: Option<bool> = None,
        includeTags: Option<bool> = None,
    ], []
}

macro_args! {
    EtymologiesArgs, [
        useCanonical: Option<bool> = None,
    ], []
}

// Words API args //

macro_args! {
    RandomWordArgs, [
        hasDictionaryDef: Option<bool> = None,
        includePartOfSpeech: Option<String> = None,
        excludePartOfSpeech: Option<String> = None,
        minCorpusCount: Option<i32> = None,
        maxCorpusCount: Option<i32> = None,
        minDictionaryCount: Option<i32> = None,
        maxDictionaryCount: Option<i32> = None,
        minLength: Option<i32> = None,
        maxLength: Option<i32> = None,
    ], []
}

macro_args! {
    RandomWordsArgs, [
        hasDictionaryDef: Option<bool> = None,
        includePartOfSpeech: Option<String> = None,
        excludePartOfSpeech: Option<String> = None,
        minCorpusCount: Option<i32> = None,
        maxCorpusCount: Option<i32> = None,
        minDictionaryCount: Option<i32> = None,
        maxDictionaryCount: Option<i32> = None,
        minLength: Option<i32> = None,
        maxLength: Option<i32> = None,
        sortBy: Option<String> = None,
        sortOrder: Option<String> = None,
        limit: Option<i32> = Some(10),
    ], []
}

#[cfg(test)]
mod test {
    #[test]
    fn test_word_endpoint_args() {
        let args = super::DefinitionArgs {
            includeTags: Some(true),
            ..super::DefinitionArgs::default()
        };
        assert_eq!(dbg!(args.to_uri().as_str()), "&limit=200&includeTags=true");

        let args = super::EtymologiesArgs {
            useCanonical: Some(true),
            ..super::EtymologiesArgs::default()
        };
        assert_eq!(dbg!(args.to_uri().as_str()), "&useCanonical=true");
    }

    #[test]
    fn test_words_args() {
        let args = super::RandomWordArgs {
            minCorpusCount: Some(10),
            ..super::RandomWordArgs::default()
        };
        assert_eq!(dbg!(args.to_uri().as_str()), "&minCorpusCount=10");

        let args = super::RandomWordsArgs {
            minCorpusCount: Some(10),
            ..super::RandomWordsArgs::default()
        };
        assert_eq!(dbg!(args.to_uri().as_str()), "&minCorpusCount=10&limit=10");
    }
}