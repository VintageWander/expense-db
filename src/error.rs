use salvo::{prelude::StatusError, Piece};
use thiserror::Error;

use crate::{validation::extract_validation_error, web::Web};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Variable error: {0}")]
    Var(#[from] std::env::VarError),

    #[error("Env error: {0}")]
    Env(#[from] dotenv::Error),

    #[error("MongoDB error: {0}")]
    MongoDB(#[from] mongodb::error::Error),

    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Permissions error: {0}")]
    Permissions(String),

    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("HttpParse error: {0}")]
    HttpParse(#[from] salvo::http::ParseError),

    #[error("ObjectId parse error: {0}")]
    ObjectId(#[from] mongodb::bson::oid::Error),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Generic(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::Generic(s.to_string())
    }
}

impl Piece for Error {
    fn render(self, res: &mut salvo::Response) {
        let error_message = match self {
            Error::Var(ref e) => format!("Cannot load one of the environment variables {e}"),
            Error::Env(ref e) => format!("Env file error: {e}"),
            Error::MongoDB(ref e) => format!("MongoDB has encountered an error {e}"),
            Error::Generic(ref e) => format!("Generic error: {e}"),
            Error::Permissions(ref e) => format!("Permission error: {e}"),
            Error::Validation(ref e) => {
                format!("Validation error {}", extract_validation_error(e))
            }
            Error::HttpParse(ref e) => format!("Http Parse error: {e}"),
            Error::ObjectId(ref e) => format!("ObjectId parse error: {e}"),
            Error::IO(ref e) => format!("IO error {e}"),
        };

        let error_status = match self {
            Error::Permissions(_) => StatusError::forbidden(),
            Error::Generic(_) | Error::Validation(_) | Error::HttpParse(_) => {
                StatusError::bad_request()
            }
            _ => StatusError::internal_server_error(),
        };

        let error = match self {
            Error::Permissions(_) => Web::forbidden(error_message),
            Error::Generic(_) | Error::Validation(_) | Error::HttpParse(_) => {
                Web::bad_request(error_message)
            }
            _ => Web::internal_error(error_message),
        };

        res.render(error);
        res.set_status_error(error_status);
    }
}
