use bamboo_metadata_podcast::episode::Episode as PodcastEpisode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Type {
    Podcast(PodcastEpisode),
}

#[derive(Serialize, Deserialize)]
pub struct Episode {
    pub order: Option<usize>,
    pub published: Option<NaiveDate>,
    pub r#type: Type,
}
