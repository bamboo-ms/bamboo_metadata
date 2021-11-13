use crate::Tag;
use crate::{chapter::Chapter, episode::Episode, series::Series};
use bamboo_metadata_util::{art::Art, id::Id, LocalizedString as String};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Type {
    Collection(Vec<Media>),
    Series(Series),
    Episode(Episode),
    Chapter(Chapter),
}

#[derive(Serialize, Deserialize)]
pub struct Media {
    pub identifiers: Vec<Id>,
    pub tags: Vec<Tag>,
    pub title: String,
    // TODO: Consider making description a localized string to allow multiple langs
    pub description: Option<String>,
    pub art: Vec<Art>,
    pub r#type: Type,
}
