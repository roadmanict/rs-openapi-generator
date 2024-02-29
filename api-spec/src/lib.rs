pub mod endpoint;

use builder::{EndpointBuilder, NoMethod, NoPath};
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct ApiSpec {
    pub service_name: String,
    pub endpoints: Vec<Endpoint>,
}

impl ApiSpec {
    pub fn builder() -> ApiSpecBuilder<NoServiceName> {
        ApiSpecBuilder::new()
    }
}

pub struct NoServiceName;

pub struct ApiSpecBuilder<ServiceName> {
    service_name: ServiceName,
    endpoints: Vec<Endpoint>,
}

impl<ServiceName> ApiSpecBuilder<ServiceName> {
    pub fn endpoint(&mut self, endpoint: impl Into<Endpoint>) -> &mut Self {
        self.endpoints.push(endpoint.into());

        self
    }

    pub fn endpoints(&mut self, endpoints: impl Into<Vec<Endpoint>>) -> &mut Self {
        self.endpoints = endpoints.into();

        self
    }
}

impl ApiSpecBuilder<NoServiceName> {
    pub fn new() -> Self {
        Self {
            service_name: NoServiceName,
            endpoints: vec![],
        }
    }

    pub fn service_name(self, service_name: impl Into<String>) -> ApiSpecBuilder<String> {
        ApiSpecBuilder {
            service_name: service_name.into(),
            endpoints: self.endpoints,
        }
    }
}

impl ApiSpecBuilder<String> {
    pub fn build(&mut self) -> ApiSpec {
        let service_name = self.service_name.clone();
        let endpoints = self.endpoints.clone();

        ApiSpec {
            service_name,
            endpoints,
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
}
