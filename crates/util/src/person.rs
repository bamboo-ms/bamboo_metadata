use crate::art::Art;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Person {
    id: Uuid,
    // All names the given person goes by
    pub names: Vec<String>,
    pub images: Vec<Art>,
}

// A cast member of a production
#[derive(Serialize, Deserialize)]
pub struct Cast {
    pub person: Person,
    // The role in the production played by this person
    pub role: String,
}
