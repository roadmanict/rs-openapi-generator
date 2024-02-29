use serde::Serialize;

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
        pub fn operation_id(mut self, operation_id: impl Into<Option<String>>) -> Self {
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
        pub fn build(self) -> Endpoint {
            Endpoint {
                path: self.path,
                method: self.method,
                operation_id: self.operation_id,
            }
        }
    }
}
