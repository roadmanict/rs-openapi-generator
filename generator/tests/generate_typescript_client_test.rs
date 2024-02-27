use generator::{generate, test_template};
use openapi_parser::parse;
use std::{fs, path::Path};

#[test]
pub fn generate_typescript_client_test() {
    let template_dir = "resources/typescript_template_example";
    let example_oas_path = "resources/example.oas.yml";
    let example_oas = fs::read_to_string(example_oas_path).expect("Should read oas spec file");

    let spec = parse(&example_oas).expect("Parsing oas");

    generate(Path::new("resources/typescript_template_example"), spec)
        .expect("Should generate templates");

    test_template(Path::new(template_dir)).expect("Should be valid");
}
