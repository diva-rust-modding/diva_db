#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

mod io;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SprDb {
    pub sets: BTreeMap<u32, SprDbSet>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SprDbSet {
    #[cfg_attr(feature = "serde", serde(skip))]
    index: i32,
    pub name: String,
    pub filename: String,
    pub sprites: BTreeMap<u32, SprDbEntry>,
    pub textures: BTreeMap<u32, SprDbEntry>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct SprDbEntry {
    #[cfg_attr(feature = "serde", serde(skip))]
    index: u16,
    pub name: String,
}
