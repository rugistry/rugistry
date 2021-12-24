use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub addr: String,

    pub repo: HashMap<String, Repo>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Repo {
    Crates {}
}
