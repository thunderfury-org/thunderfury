use std::collections::HashMap;

use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, FromJsonQueryResult, Serialize, Deserialize)]
pub struct StringVec(pub Vec<String>);

impl StringVec {
    pub fn contains(&self, s: &str) -> bool {
        self.0.iter().any(|x| x == s)
    }
}

impl From<Vec<String>> for StringVec {
    fn from(val: Vec<String>) -> Self {
        StringVec(val)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, FromJsonQueryResult, Serialize, Deserialize)]
pub struct StringMap(pub HashMap<String, String>);

impl From<HashMap<String, String>> for StringMap {
    fn from(val: HashMap<String, String>) -> Self {
        StringMap(val)
    }
}

impl StringMap {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}
