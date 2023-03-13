use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cml {
    pub id: u32,
}

impl Cml {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
