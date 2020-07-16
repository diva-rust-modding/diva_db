#[cfg(feature = "serde")]
use serde::*;

use std::borrow::Cow;
use std::collections::BTreeMap;

#[cfg(feature="pyo3")]
pub mod py_ffi;
pub mod read;

// #[derive(Debug, Default, PartialEq, Clone)]
// pub struct TextureInfo<'a> {
//     pub id: usize,
//     pub name: Cow<'a, str>,
// }

#[derive(Debug, Default, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextureDatabase<'a> {
    pub entries: BTreeMap<usize, Cow<'a, str>>,
}
