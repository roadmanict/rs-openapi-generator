use generator::generate;
use openapi_parser::parse;
use std::{fs, path::Path};

#[test]
pub fn generate_typescript_client_test() {
    let example_oas_path = "resources/example.oas.yml";
    let example_oas = fs::read_to_string(example_oas_path).expect("Reading oas");

    let openapi_spec = parse(&example_oas).expect("Parsing oas");

    generate(
        Path::new("resources/typescript_template_example"),
        openapi_spec.into(),
    )
    .expect("Generator to complete");
}
