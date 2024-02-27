use api_spec::ApiSpec;
use models::openapi::OpenApi;
use thiserror::Error;

pub mod models;

#[derive(Error, Debug)]
pub enum ParseOpenApiError {
    #[error("Error parsing openapi spec")]
    ParseError(#[from] serde_yaml::Error),
    #[error("Invalid method {0}")]
    InvalidMethod(String),
    #[error("Missing property {0}")]
    MissingField(String),
}

pub fn parse(oas_spec: &str) -> Result<ApiSpec, ParseOpenApiError> {
    let oas: OpenApi = serde_yaml::from_str(&oas_spec)?;

    oas.try_into()
}
