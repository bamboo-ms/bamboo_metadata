use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Id {
    pub database_id: String,
    pub database_value: String,
}
