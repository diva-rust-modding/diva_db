use crate::spr;

mod read;

#[derive(Debug, Clone)]
pub struct AetDb {
    pub sets: Vec<AetDbSet>,
}

#[derive(Debug, Clone)]
pub struct AetDbSet {
    pub id: u32,
    pub name: String,
    pub filename: String,
    pub index: i32,
    pub spr_set: spr::SprDbSet,
    pub scenes: Vec<AetDbScene>,
}

#[derive(Debug, Clone)]
pub struct AetDbScene {
    pub id: u32,
    pub name: String,
    pub index: u16,
}
