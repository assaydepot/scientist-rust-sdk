#[derive(Deserialize, Debug)]
pub struct SciInfo {
    pub version: String,
    pub message: Option<String>,
    pub api_version: String,
    pub environment: String,
}
