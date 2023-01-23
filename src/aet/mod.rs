#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

mod io;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AetDb {
    pub sets: BTreeMap<u32, AetDbSet>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AetDbSet {
    pub name: String,
    pub filename: String,
    pub spr_set_id: u32,
    pub scenes: BTreeMap<u32, AetDbScene>,
    #[cfg_attr(feature = "serde", serde(skip))]
    index: i32,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AetDbScene {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    index: u16,
}
