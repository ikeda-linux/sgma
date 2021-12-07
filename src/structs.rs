#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ConfigFile {
    pub outpath: String,
    pub srcpath: String,
    pub exclude: Vec<String>,
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            outpath: "out".to_string(),
            srcpath: "src".to_string(),
            exclude: vec![],
        }
    }
}