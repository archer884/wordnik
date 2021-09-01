use std::fmt;

#[derive(Debug)]
pub enum Error {
    // No clue what I'll use this for
    Api(&'static str),
    Network(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(v: reqwest::Error) -> Self {
        Self::Network(v)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Api(e) => f.write_str(e),
            Error::Network(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}
