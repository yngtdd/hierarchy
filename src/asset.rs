use crate::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub id: u32,
    pub components: Vec<Component>,
}

#[derive(Default)]
pub struct AssetBuilder {
    pub id: u32,
    pub components: Vec<Component>,
}

impl AssetBuilder {
    pub fn new() -> Self {
        AssetBuilder::default()
    }

    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn components(mut self, components: Vec<Component>) -> Self {
        self.components = components;
        self
    }

    pub fn build(self) -> Asset {
        Asset { id: self.id, components: self.components }
    }
}

