use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub name: String,
    pub pass: String,
}
