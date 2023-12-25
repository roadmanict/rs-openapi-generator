use serde::Deserialize;
use api_spec::ApiSpec;
use super::openapi_info::OpenApiInfo;

#[derive(Deserialize, PartialEq, Debug)]
pub struct OpenApi {
    pub info: OpenApiInfo
}


impl From<OpenApi> for ApiSpec {
    fn from(value: OpenApi) -> Self {
        ApiSpec {
            service_name: value.info.title,
        }
    }
}
