use serde::Serialize;
use std::fmt;

#[derive(Clone, Serialize)]
pub struct MyNode {
    pub name: String,
    pub from: String,
    pub to: String,
}

impl fmt::Debug for MyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {} -> {}", self.name, self.from, self.to)
    }
}

pub struct ParseHomepageNode {
    pub category_url: String,
    pub category_title: String,
}
