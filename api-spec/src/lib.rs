use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct ApiSpec {
    pub service_name: String,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Endpoint {
    pub path: String,
    pub method: RequestMethod,
    pub operation_id: Option<String>,
}
