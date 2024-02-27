use std::fs;

use api_spec::{ApiSpec, Endpoint, RequestMethod};
use openapi_parser::parse;

#[test]
fn test_parsing_example_openapi() {
    let example_oas_path = "resources/example.oas.yml";
    let example_oas = fs::read_to_string(example_oas_path).expect("Should open and read file");

    let mut result = parse(&example_oas).expect("Should parse openapi spec into api spec");

    result.endpoints.sort_by(|a, b| a.path.cmp(&b.path));

    println!("{:#?}", result);

    assert_eq!(result.endpoints.len(), 8);

    assert_eq!(
        result,
        ApiSpec {
            service_name: "Swagger Petstore - OpenAPI 3.0".to_string(),
            endpoints: vec![
                Endpoint {
                    path: "/pet/findByStatus".to_string(),
                    method: RequestMethod::Get,
                    operation_id: None,
                },
                Endpoint {
                    path: "/pet/findByTags".to_string(),
                    method: RequestMethod::Get,
                    operation_id: None,
                },
                Endpoint {
                    path: "/pet/{petId}".to_string(),
                    method: RequestMethod::Get,
                    operation_id: None,
                },
                Endpoint {
                    path: "/store/inventory".to_string(),
                    method: RequestMethod::Get,
                    operation_id: None,
                },
                Endpoint {
                    path: "/store/order/{orderId}".to_string(),
                    method: RequestMethod::Get,
                    operation_id: None,
                },
                Endpoint {
                    path: "/user/login".to_string(),
                    method: RequestMethod::Get,
                    operation_id: None,
                },
                Endpoint {
                    path: "/user/logout".to_string(),
                    method: RequestMethod::Get,
                    operation_id: None,
                },
                Endpoint {
                    path: "/user/{username}".to_string(),
                    method: RequestMethod::Get,
                    operation_id: None,
                },
            ],
        }
    );
}
