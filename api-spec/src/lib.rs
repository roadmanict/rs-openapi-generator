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

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct Endpoint {
    pub path: String,
    pub method: RequestMethod,
    pub operation_id: Option<String>,
}

impl Endpoint {
    pub fn builder() -> EndpointBuilder<NoPath, NoMethod> {
        EndpointBuilder::new()
    }
}

pub mod builder {
    use super::*;

    pub struct NoPath;
    pub struct NoMethod;

    pub struct EndpointBuilder<Path, Method> {
        path: Path,
        method: Method,
        operation_id: Option<String>,
    }

    impl EndpointBuilder<NoPath, NoMethod> {
        pub fn new() -> Self {
            Self {
                path: NoPath,
                method: NoMethod,
                operation_id: None,
            }
        }
    }

    impl<Path, Method> EndpointBuilder<Path, Method> {
        pub fn operation_id(&mut self, operation_id: impl Into<Option<String>>) -> &mut Self {
            self.operation_id = operation_id.into();

            self
        }

        pub fn path(self, path: impl Into<String>) -> EndpointBuilder<String, Method> {
            EndpointBuilder {
                path: path.into(),
                method: self.method,
                operation_id: self.operation_id,
            }
        }

        pub fn method(
            self,
            method: impl Into<RequestMethod>,
        ) -> EndpointBuilder<Path, RequestMethod> {
            EndpointBuilder {
                path: self.path,
                method: method.into(),
                operation_id: self.operation_id,
            }
        }
    }

    impl EndpointBuilder<String, RequestMethod> {
        pub fn build(&mut self) -> Endpoint {
            Endpoint {
                path: self.path.clone(),
                method: self.method.clone(),
                operation_id: self.operation_id.clone(),
            }
        }
    }
}
