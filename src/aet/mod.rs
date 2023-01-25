//! Aet database
//!
//! This module contains all functionality related to reading information about
//! `auth2d`s from the game's aet database `aet_db.bin` (Pre-F2nd).
//!
//! The entry point for this module is the [AetDb] type
//!
//! # Naming
//! The name `aet` presumably stands for **A**fter **E**ffects **T**emplate,
//! the format that aets are based on.
//!
//! # Examples
//!
//! ```rust,ignore
//! use diva_db::aet::AetDb;
//!
//! fn patch_aets(db: AetDb) {
//!     for set in db.sets {
//!         let mut file = std::fs::File::open(set.filename);
//!         process_aet(&mut file);
//!         info!("Patched: AET {:?} in file {:?}", set.name, set.filename);
//!         for scene in set.scenes {
//!             trace!(scene.id, scene.name);
//!         }
//!     }
//!     info!("Patched {} aet(s)", set.sets.len());
//! }
//! ```
//!
//! # Safety
//! It is undefined behavior to store non-ASCII strings inside of the database.

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

mod io;
#[cfg(feature = "pyo3")]
pub mod py_ffi;

/// The Aet Database
///
/// The information store which collects information about all [`AetDbSet`]s used by the game.
/// It is a simple key-value store where the keys are unsigned 32 bit integers.
///
/// # Usage
/// Most instances of the database are created through *reading* them via [`AetDb::read`] or [`AetDb::from_bytes`].
///
/// ```rust
/// use diva_db::aet::AetDb;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let reader = std::fs::File::open("assets/aft_aet_db.bin")?;
/// let db = AetDb::read(reader)?;
/// assert_eq!(db.sets.len(), 1063);
/// # Ok(())
/// # }
/// ```
///
/// # Limitations
/// Since ids are represented by [`u32`], this means that there can only be [`u32::MAX`]+1 entries.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct AetDb {
    pub sets: BTreeMap<u32, AetDbSet>,
}

/// Aet set
///
/// Represents the collection of aet scenes inside of an `.aet` auth 2d file.
/// The files are usually found in `rom/2d` (Pre-F2nd).
///
/// # Safety
/// It is undefined behavior to store non-ASCII strings.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct AetDbSet {
    pub index: i32,
    /// Name of the set. (Must be in ASCII)
    pub name: String,
    /// Name of the file containing the set. (Must be in ASCII)
    pub filename: String,
    /// Id of the corresponding [crate::spr::SprDbSet]. (Must be in ASCII)
    pub spr_set_id: u32,
    /// List of all scenes contained in the set.
    pub scenes: BTreeMap<u32, AetDbScene>,
}

/// Aet database entry
///
/// # Safety
/// It is undefined behavior to store non-ASCII strings.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct AetDbScene {
    pub index: u16,
    /// Name of the set
    pub name: String,
}
