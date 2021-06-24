use super::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct LocalizedTitle {
    pub language: Option<Language>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Title {
    // The title in the language in which the media was produced
    pub local: Option<LocalizedTitle>,
    pub others: Option<Vec<LocalizedTitle>>,
}
