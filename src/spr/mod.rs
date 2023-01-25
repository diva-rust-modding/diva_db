//! Sprite database
//!
//! This module contains all functionality related to reading sprite information
//! from the game's sprite database `spr_db.bin` (Pre-F2nd).
//!
//! The entry point for this module is the [SprDb] type
//!
//! # Examples
//! ```rust,ignore
//! fn replace_all_in_db(db: SprDb) -> Result<(), Error> {
//!     for set in db.sets {
//!         let mut file = std::fs::File::open(db_set.filename)?;
//!         let spr = Spr::read(&mut file);
//!         replace_sprites(spr, set.sprites)?;
//!         replace_textures(spr, set.textures)?;
//!         spr.write(&mut file)?;
//!     }
//!     Ok(())
//! }
//! ```
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

/// The Sprite Database
///
/// The information store which collects information about all [`SprDbSet`]s used by the game.
/// It is a simple key-value store where the keys are unsigned 32 bit integers.
///
/// # Usage
/// Most instances of the database are created through *reading* them via [`SprDb::read`] or [`SprDb::from_bytes`].
///
/// ```rust
/// use diva_db::spr::SprDb;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let reader = std::fs::File::open("assets/aft_spr_db.bin")?;
/// let db = SprDb::read(reader)?;
/// assert_eq!(db.sets.len(), 2983);
/// # Ok(())
/// # }
/// ```
///
/// # Limitations
/// Since ids are represented by [`u32`], this means that there can only be [`u32::MAX`]+1 entries.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct SprDb {
    pub sets: BTreeMap<u32, SprDbSet>,
}

/// Sprite set
///
/// Represents the collection of sprites and textures inside of a `.spr` file.
/// The files are usually found in `rom/2d` (Pre-F2nd).
///
/// # Safety
/// It is undefined behavior to store non-ASCII strings.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct SprDbSet {
    pub index: i32,
    /// The name of the sprite set. (Must be ASCII)
    pub name: String,
    /// The file which contains the sprite set. (Must be ASCII)
    pub filename: String,
    /// List of all sprites belonging to the set
    pub sprites: BTreeMap<u32, SprDbEntry>,
    /// List of all textures belonging to the set
    pub textures: BTreeMap<u32, SprDbEntry>,
}

/// Sprite database entry
///
/// This struct represents both sprites and textures.
///
/// # In-game representation
/// In game, the two are differentiated by the index of the parent set.
/// For textures the index is `OR`ed with `0x1000`.
///
/// ## Example
/// For an entry with set index `0` is a sprite belonging to the `0`-th sprite set.
///
/// For an entry with set index `0x1000` is a texture belonging to the `0`-th sprite set.
///
/// # Safety
/// It is undefined behavior to store non-ASCII strings.
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
pub struct SprDbEntry {
    pub index: u16,
    /// Name of the entry. (Must be in ASCII)
    pub name: String,
}
