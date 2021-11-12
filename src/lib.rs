use serde::{Deserialize, Serialize};

pub mod chapter;
pub mod episode;
pub mod media;
pub mod series;

#[derive(Serialize, Deserialize)]
pub struct Tag(String);
