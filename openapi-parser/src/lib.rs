use thiserror::Error;
use models::openapi::OpenApi;

pub mod models;

#[derive(Error, Debug)]
pub enum ParseOpenApiError {
    #[error("Error parsing openapi spec")]
    ParseError(#[from] serde_yaml::Error)
}

pub fn parse(oas_spec: &str) -> Result<OpenApi, ParseOpenApiError> {
    let oas: OpenApi = serde_yaml::from_str(&oas_spec)?;

    Ok(oas)
}
