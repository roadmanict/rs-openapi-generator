use api_spec::ApiSpec;
use serde::Deserialize;
use std::{
    ffi::OsStr,
    fs,
    path::Path,
};

use thiserror::Error;

#[derive(Deserialize, Debug)]
struct GenerateConfig {
    dest: String,
}

#[derive(Debug)]
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
}

type GenerateResult<T> = Result<T, GenerateError>;

pub fn generate(template_dir: &Path, openapi: ApiSpec) -> GenerateResult<()> {
    let config = fs::read_to_string(template_dir.join("template.toml"))?;
    let config: GenerateConfig = toml::from_str(&config)?;
    println!("config: {:?}", config);
    let context = Context {
        config,
        api_spec: openapi,
    };

    let parser = liquid::ParserBuilder::with_stdlib().build()?;

    let contents = fs::read_dir(template_dir)?;
    for item in contents {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            continue;
        }

        if Some(OsStr::new("liquid")) != path.extension() {
            println!("endswith: {:?}", path);
            continue;
        }

        let template_string = fs::read_to_string(&path)?;
        let template = parser.parse(&template_string)?;
        let output = template.render(&liquid::Object::new())?;
        println!("output {}", output);
    }

    panic!();
}

