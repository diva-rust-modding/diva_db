use std::collections::BTreeMap;

use crate::spr;

mod read;

#[derive(Debug, Clone)]
pub struct AetDb {
    pub sets: BTreeMap<u32, AetDbSet>,
}

#[derive(Debug, Clone)]
pub struct AetDbSet {
    pub name: String,
    pub filename: String,
    pub spr_set_id: u32,
    pub scenes: BTreeMap<u32, AetDbScene>,
    index: i32,
}

#[derive(Debug, Clone)]
pub struct AetDbScene {
    pub name: String,
    index: u16,
}
