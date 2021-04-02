use std::{convert::From, error::Error as StdError, fmt::Display};

#[derive(Debug)]
pub enum Error {
    NotFound,
    JWE(id_contact_jwt::Error),
    Postgres(postgres::Error),
}

impl<'r, 'o: 'r> rocket::response::Responder<'r, 'o> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let debug_error = rocket::response::Debug::from(self);
        debug_error.respond_to(request)
    }
}

impl From<id_contact_jwt::Error> for Error {
    fn from(e: id_contact_jwt::Error) -> Self {
        Error::JWE(e)
    }
}

impl From<postgres::Error> for Error {
    fn from(e: postgres::Error) -> Self {
        Error::Postgres(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => f.write_str("Not found"),
            Error::JWE(e) => e.fmt(f),
            Error::Postgres(e) => e.fmt(f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::JWE(e) => Some(e),
            Error::Postgres(e) => Some(e),
            _ => None,
        }
    }
}
