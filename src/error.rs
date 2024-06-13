use std::fmt::{Display, Formatter, Result as FmtResult, write};
use std::error;
#[cfg(feature = "reqwest")]
use reqwest;
use url;

#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "reqwest")]
    NetworkError(reqwest::Error),
    UrlParseError(url::ParseError),
    // TokioError(tokio::task::JoinError),
    Unexpected,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            #[cfg(feature = "reqwest")]
            Error::NetworkError(ref e)   => write!(f, "NetworkError:  {}", e),
            Error::UrlParseError(ref e)  => write!(f, "UrlParseError:  {}", e),
            // Error::TokioError(ref e) => write!(f, "TokioError: {}", e),
            Error::Unexpected            => write!(f, "UnexpectedError"),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::NetworkError(err)
    }
}

// impl From<tokio::task::JoinError> for Error {
//     fn from(err: tokio::task::JoinError) -> Error {
//         Error::TokioError(err)
//     }
// }

impl error::Error for Error {
    fn description(&self) -> &str { "" }
}
