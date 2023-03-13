use crate::asset::Asset;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub assets: Vec<Asset>,
}

#[derive(Default)]
pub struct UnitBuilder {
    pub id: u32,
    pub assets: Vec<Asset>,
}

impl UnitBuilder {
    pub fn new() -> Self {
        UnitBuilder::default()
    }

    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn components(mut self, assets: Vec<Asset>) -> Self {
        self.assets = assets;
        self
    }

    pub fn build(self) -> Unit {
        Unit { id: self.id, assets: self.assets }
    }
}

