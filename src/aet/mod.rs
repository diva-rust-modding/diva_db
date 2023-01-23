#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

mod io;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AetDb {
    pub sets: BTreeMap<u32, AetDbSet>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AetDbSet {
    #[cfg_attr(feature = "serde", serde(skip))]
    index: i32,
    pub name: String,
    pub filename: String,
    pub spr_set_id: u32,
    pub scenes: BTreeMap<u32, AetDbScene>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AetDbScene {
    #[cfg_attr(feature = "serde", serde(skip))]
    index: u16,
    pub name: String,
}
