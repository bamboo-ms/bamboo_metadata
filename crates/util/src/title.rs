use super::language::Language;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Localized {
    // None indicates that the language is unknown
    pub language: Option<Language>,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Title {
    // The title in the language in which the media was produced
    pub local: Option<Localized>,
    pub others: Vec<Localized>,
}
