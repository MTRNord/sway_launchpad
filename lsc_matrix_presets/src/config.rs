use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::{fs, path::Path};
use tracing::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config<'a> {
    pub homeserver_url: Cow<'a, str>,
    pub mxid: Cow<'a, str>,
    pub password: Cow<'a, str>,
    pub store_path: Cow<'a, str>,
}

impl Config<'_> {
    #[instrument]
    pub fn load() -> Result<Self> {
        let contents =
            fs::read_to_string("./config.yml").expect("Something went wrong reading the file");
        let config: Self = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
