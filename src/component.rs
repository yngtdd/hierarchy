use crate::cml::Cml;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    pub id: u32,
    pub cmls: Vec<Cml>,
}

impl Component {
    pub fn new(id: u32, cmls: Vec<Cml>) -> Self {
        Self { id, cmls }
    }
}

#[derive(Default)]
pub struct ComponentBuilder {
    pub id: u32,
    pub cmls: Vec<Cml>,
}

impl ComponentBuilder {
    pub fn new() -> Self {
        ComponentBuilder::default()
    }

    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn cmls(mut self, cmls: Vec<Cml>) -> Self {
        self.cmls = cmls;
        self
    }

    pub fn build(self) -> Component {
        Component { id: self.id, cmls: self.cmls }
    }
}

