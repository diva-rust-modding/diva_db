use std::collections::BTreeMap;

mod io;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SprDb {
    pub sets: BTreeMap<u32, SprDbSet>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct SprDbSet {
    index: i32,
    pub name: String,
    pub filename: String,
    pub sprites: BTreeMap<u32, SprDbEntry>,
    pub textures: BTreeMap<u32, SprDbEntry>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct SprDbEntry {
    index: u16,
    pub name: String,
}
