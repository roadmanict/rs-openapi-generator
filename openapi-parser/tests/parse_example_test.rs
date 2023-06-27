use std::fs;

use openapi_parser::parse;

#[test]
fn test_parsing_example_openapi() {
    let example_oas_path = "tests/files/example.oas.yml";
    let example_oas = fs::read_to_string(example_oas_path).expect("Reading example oas");

    let result = parse(&example_oas).expect("Parsing example oas");
    
    assert_eq!(result.info.title, "Swagger Petstore - OpenAPI 3.0");
}
