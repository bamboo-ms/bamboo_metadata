use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Person {
    id: Uuid,
    // All names the given person goes by
    pub names: Vec<String>,
}
