use serde::Deserialize;
use super::openapi_info::OpenApiInfo;

#[derive(Deserialize, PartialEq, Debug)]
pub struct OpenApi {
    pub info: OpenApiInfo
}
