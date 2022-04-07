#![allow(non_snake_case)]

#[macro_use] extern crate serde;

#[derive(Clone, Serialize)]
pub struct Lake {
    pub id: usize,
    pub name: String,
    pub fish: Vec<Fish>
}

#[derive(Clone, Serialize)]
pub struct Fish {
    pub id: usize,
    pub name: String,
}

