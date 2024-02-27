use api_spec::ApiSpec;
use openapi_parser::{parse, ParseOpenApiError};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fs, path::Path};

use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
struct GenerateConfig {
    dest: String,
}

#[derive(Debug, Serialize)]
struct Context {
    config: GenerateConfig,
    api_spec: ApiSpec,
}

#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("Error reading from template")]
    FileSystemError(#[from] std::io::Error),
    #[error("Error parsing config")]
    ParseConfigError(#[from] toml::de::Error),
    #[error("Error in template engine")]
    TemplatingError(#[from] liquid::Error),
    #[error("Error parsing openapi spec")]
    ParseOpenApiError(#[from] ParseOpenApiError),
}

type GenerateResult<T> = Result<T, GenerateError>;

pub fn generate(template_dir: &Path, spec: ApiSpec) -> GenerateResult<()> {
    let config = fs::read_to_string(template_dir.join("template.toml"))?;
    let config: GenerateConfig = toml::from_str(&config)?;
    println!("config: {:?}", config);
    let context = Context {
        config,
        api_spec: spec,
    };

    let globals = liquid::object!({ "context": context });

    let parser = liquid::ParserBuilder::with_stdlib().build()?;

    let contents = fs::read_dir(template_dir)?;
    for item in contents {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            continue;
        }

        if Some(OsStr::new("liquid")) != path.extension() {
            println!("ignored by extension: {:?}", path);
            continue;
        }

        let template_string = fs::read_to_string(&path)?;
        let template = parser.parse(&template_string)?;
        let output = template.render(&globals)?;
        println!("output {}", output);
    }

    todo!();
}

pub fn test_template(template_dir: &str) -> GenerateResult<()> {
    let template_tests_dir = format!("{}/tests", template_dir);
    let api_spec = parse(&format!("{}/example.oas.yml", template_tests_dir))?;

    for endpoint in api_spec.endpoints {
        let file_contents = test_and_open_file(&format!("{}/expected/{}.ts", template_tests_dir, endpoint.operation_id))
    }
    todo!()
}

fn test_and_open_file(file_path: &str) -> GenerateResult<String> {
    let path = Path::new(file_path);
    assert!(path.exists(), "Error: File does not exist");

    Ok(fs::read_to_string(path)?)
}
