use super::{chapter::Chapter, Rating};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Episode {
    // MEDIA:RESTRICTION (5.24) is intentionally omitted as it is not applicable to the Bamboo side
    // of distribution - content restriction should be enforced at the publisher level.
    pub duration: Option<Duration>,
    // "Indicates the order in which the episode shall be played. If omitted, the [published] field
    // will be used to generate order with the oldest episode starting as the first list entry." -
    // 5.26
    pub rating: Option<Rating>,
    // "Creates a list of chapters for this episode. If present there must be at least one chapter
    // defined in the list." - 5.30
    pub chapters: Vec<Chapter>,
}
