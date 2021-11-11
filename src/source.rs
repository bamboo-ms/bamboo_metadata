use crate::uri::Uri;
use serde::{Deserialize, Serialize};
use std::path::PathBuf as FilesystemPath;

// A path to a location of a file
#[derive(Serialize, Deserialize)]
pub enum Path {
    Local(FilesystemPath),
    Web(Uri),
}
