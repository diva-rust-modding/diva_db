use std::collections::BTreeMap;

mod io;

#[derive(Debug, Clone)]
pub struct SprDb {
    pub sets: BTreeMap<u32, SprDbSet>,
}

#[derive(Debug, Clone)]
pub struct SprDbSet {
    pub name: String,
    pub filename: String,
    pub sprites: BTreeMap<u32, SprDbEntry>,
    pub textures: BTreeMap<u32, SprDbEntry>,
    index: i32,
}

#[derive(Debug, Clone)]
pub struct SprDbEntry {
    pub name: String,
    index: u16,
}
