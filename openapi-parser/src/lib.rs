use thiserror::Error;
use models::openapi::OpenApi;

pub mod models;

#[derive(Error, Debug)]
pub enum ParseOpenApiError {
    #[error("Error parsing openapi spec")]
    ParseError
}

pub fn parse(oas_spec: &str) -> Result<OpenApi, ParseOpenApiError> {
    Err(ParseOpenApiError::ParseError)
}
