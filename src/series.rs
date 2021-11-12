use bamboo_metadata_podcast::show::Show;
use bamboo_metadata_util::{person::Cast, Language};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Type {
    Podcast(Show),
}

#[derive(Serialize, Deserialize)]
pub enum Complete {
    Complete,
    Incomplete,
}

#[derive(Serialize, Deserialize)]
pub enum Chronology {
    Episodic,
    Serial,
}

#[derive(Serialize, Deserialize)]
pub struct Series {
    pub collaborators: Vec<Cast>,
    pub language: Option<Language>,
    pub status: Option<Complete>,
    pub chronology: Option<Chronology>,
    pub r#type: Type,
}
