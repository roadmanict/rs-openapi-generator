use std::fs;
use openapi_parser::parse;

#[test]
pub fn generate_typescript_client_test() {
    let example_oas_path = "resources/example.oas.yml";
    let example_oas = fs::read_to_string(example_oas_path).expect("Reading oas");

    let result = parse(&example_oas).expect("Parsing oas");
}
