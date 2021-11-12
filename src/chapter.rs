use bamboo_metadata_podcast::chapter::Chapter as PodcastChapter;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Type {
    Podcast(PodcastChapter),
}

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    // TODO: consider replacing with a proper type to represent timestamps
    // Time from beginning of episode in seconds
    pub start: i32,
    pub r#type: Type,
}
