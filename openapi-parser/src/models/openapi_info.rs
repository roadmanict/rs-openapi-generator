use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct OpenApiInfo {
    pub title: String
}
