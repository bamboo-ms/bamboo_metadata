use bamboo_metadata_util::uri::Uri;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    // "A Normal Play Time (NMP) timestamp - a single point in time relative to the beginning of
    // the episode audio file." - 5.31
    // "Web address of the associated page referred to in the segment." - 5.31
    pub uri: Option<Uri>,
}
