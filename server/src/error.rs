use reqwest::{Error as HttpError, Url};
use serde_json::Error as JsonError;
use sqlx::Error as DatabaseError;
use std::convert::From;
use std::str::Utf8Error;
use url::ParseError as UrlError;

#[derive(Debug)]
pub enum Error {
    Http(Url, HttpError),
    Json(JsonError),
    Database(DatabaseError),
    Url(UrlError),
    Utf8(Utf8Error),
}

impl From<DatabaseError> for Error {
    fn from(err: DatabaseError) -> Error {
        Error::Database(err)
    }
}

impl From<(Url, HttpError)> for Error {
    fn from((url, err): (Url, HttpError)) -> Error {
        Error::Http(url, err)
    }
}

impl From<UrlError> for Error {
    fn from(err: UrlError) -> Error {
        Error::Url(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
