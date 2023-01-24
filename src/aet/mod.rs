#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

mod io;
#[cfg(feature = "pyo3")]
pub mod py_ffi;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct AetDb {
    pub sets: BTreeMap<u32, AetDbSet>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct AetDbSet {
    pub index: i32,
    pub name: String,
    pub filename: String,
    pub spr_set_id: u32,
    pub scenes: BTreeMap<u32, AetDbScene>,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct AetDbScene {
    pub index: u16,
    pub name: String,
}
