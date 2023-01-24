#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

mod io;
#[cfg(feature = "pyo3")]
pub mod py_ffi;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct SprDb {
    pub sets: BTreeMap<u32, SprDbSet>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct SprDbSet {
    pub index: i32,
    pub name: String,
    pub filename: String,
    pub sprites: BTreeMap<u32, SprDbEntry>,
    pub textures: BTreeMap<u32, SprDbEntry>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct SprDbEntry {
    pub index: u16,
    pub name: String,
}
