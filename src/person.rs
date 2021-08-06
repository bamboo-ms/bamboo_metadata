use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person<N: ToString + Sized, R: ToString + Sized> {
    pub names: Vec<N>,
    pub roles: Vec<R>,
}
