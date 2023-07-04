use axum::{extract::rejection::JsonRejection, response::IntoResponse};
use thiserror::Error;
use validator::ValidationErrors;

use crate::{helper::validation::extract_validation_error, web::Web};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Generic error")]
    Generic,

    #[error("Json request parse error")]
    Json(#[from] JsonRejection),

    #[error("Database query error")]
    Query(#[from] mongodb::error::Error),

    #[error("Request error")]
    CannotReachProxies(#[from] reqwest::Error),

    #[error("Proxy request error")]
    CannotReachProxy,

    #[error("Proxy already exists")]
    ProxyAlreadyExists,

    #[error("Proxy not found")]
    ProxyNotFound,

    #[error("Cannot add new proxy")]
    CannotCreateProxy,

    #[error("Cannot delete proxy")]
    CannotDeleteProxy,

    #[error("Invalid input")]
    InvalidInput(#[from] ValidationErrors),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Generic => Web::internal_error("Server error", "Something wrong happened"),
            Error::Json(_) => Web::bad_request(
                "Invalid request body",
                "The request body sent to the server was incorrect.",
            ),
            Error::Query(_) => Web::bad_request(
                "Database query error",
                "The information provided could not be queried.",
            ),
            Error::CannotReachProxies(e) => Web::internal_error(
                "Request to proxies error",
                format!("A connection to one of the proxies could not be made. Error: {e}."),
            ),
            Error::CannotReachProxy => Web::bad_request(
                "Request to one proxy error",
                "The proxy provided is unreachable",
            ),
            Error::ProxyAlreadyExists => Web::bad_request(
                "Proxy already exists",
                "This proxy is already exists, please try another",
            ),
            Error::CannotCreateProxy => Web::internal_error(
                "Cannot create proxy",
                "This proxy could not be created, something went wrong",
            ),
            Error::CannotDeleteProxy => Web::bad_request(
                "Cannot delete proxy",
                "This proxy could not be deleted, something went wrong",
            ),
            Error::ProxyNotFound => Web::not_found(
                "Proxy not found",
                "The url provided cannot be found in the database",
            ),
            Error::InvalidInput(e) => {
                Web::bad_request("Invalid input", extract_validation_error(&e))
            }
        }
    }
}
