use crate::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ThumbnailType {
    Album,
    Artist,
    Back,
    Background,
    Banner,
    CharacterArt,
    ClearArt,
    ClearLogo,
    DiscArt,
    FanArt,
    KeyArt,
    Headshot,
    Landscape,
    Other(String),
    Poster,
    Spine,
    Thumb, // Still excerpts of a Movie or TV Show Episode itself, commonly used for TV Show episode play previews
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnail {
    pub r#type: ThumbnailType,
    pub paths: Vec<Path>,
}
