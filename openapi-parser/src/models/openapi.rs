use std::collections::HashMap;

use crate::ParseOpenApiError;

use super::openapi_info::OpenApiInfo;
use api_spec::{endpoint::Endpoint, ApiSpec, RequestMethod};
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct OpenApi {
    pub info: OpenApiInfo,
    pub paths: HashMap<String, Operations>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Operations {
    pub get: Option<Operation>,
    pub post: Option<Operation>,
    pub put: Option<Operation>,
    pub patch: Option<Operation>,
    pub delete: Option<Operation>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Operation {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub schemes: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub operation_id: Option<String>,
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Parameter {
    pub name: String,
    pub location: Option<String>,
    pub required: Option<bool>,
    pub unique_items: Option<bool>,
    pub param_type: Option<String>,
    pub format: Option<String>,
}

impl TryFrom<OpenApi> for ApiSpec {
    type Error = ParseOpenApiError;

    fn try_from(value: OpenApi) -> Result<Self, Self::Error> {
        let endpoints_wrapper: EndpointsWrapper = PathsWrapper(value.paths).try_into()?;
        let api_spec = ApiSpec::builder()
            .service_name(value.info.title)
            .endpoints(endpoints_wrapper.0)
            .build();

        Ok(api_spec)
    }
}

struct EndpointsWrapper(Vec<Endpoint>);
struct PathsWrapper(HashMap<String, Operations>);
struct OperationsWrapper(String, Operations);

impl TryFrom<PathsWrapper> for EndpointsWrapper {
    type Error = ParseOpenApiError;

    fn try_from(value: PathsWrapper) -> Result<Self, Self::Error> {
        let mut endpoints: Vec<Endpoint> = vec![];
        for (path, operations) in value.0 {
            let ow = OperationsWrapper(path, operations);
            let mut ew: EndpointsWrapper = ow.try_into()?;
            endpoints.append(&mut ew.0);
        }

        Ok(EndpointsWrapper(endpoints))
    }
}

impl TryFrom<OperationsWrapper> for EndpointsWrapper {
    type Error = ParseOpenApiError;

    fn try_from(value: OperationsWrapper) -> Result<Self, Self::Error> {
        let path = value.0;
        let operations = value.1;

        let mut endpoints: Vec<Endpoint> = vec![];
        if let Some(operation) = operations.get {
            let method: RequestMethod = MethodWrapper(String::from("get")).try_into()?;

            endpoints.push(
                Endpoint::builder()
                    .operation_id(operation.operation_id)
                    .method(method)
                    .path(path)
                    .build(),
            );
        }

        Ok(EndpointsWrapper(endpoints))
    }
}

struct MethodWrapper(String);
impl TryFrom<MethodWrapper> for RequestMethod {
    type Error = ParseOpenApiError;

    fn try_from(value: MethodWrapper) -> Result<Self, Self::Error> {
        match value.0.to_lowercase().as_str() {
            "get" => Ok(RequestMethod::Get),
            "post" => Ok(RequestMethod::Post),
            _ => Err(ParseOpenApiError::InvalidMethod(value.0)),
        }
    }
}
