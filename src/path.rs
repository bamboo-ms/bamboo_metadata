use serde::{Deserialize, Serialize};
use crate::uri::Uri;

// A path to a location of a file
#[derive(Serialize, Deserialize)]
pub enum Source {
    Local(String),
    Web(Uri),
}
