use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cml {
    pub id: u32,
}

impl Cml {
    fn new(id: u32) -> Self {
        Self { id }
    }
}
