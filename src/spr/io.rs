use binrw::io::{Read, Seek};
use binrw::prelude::*;
use binrw::*;
use std::io::Cursor;

use super::*;

#[derive(Debug, BinRead)]
struct SprDbReader {
    set_count: i32,
    #[br(count = set_count)]
    sets: FilePtr32<Vec<SprDbSetReader>>,
    sprite_count: i32,
    #[br(count = sprite_count)]
    sprites: FilePtr32<Vec<SprDbSpriteReader>>,
}

#[derive(Debug, BinRead)]
struct SprDbSetReader {
    id: u32,
    name: FilePtr32<NullString>,
    filename: FilePtr32<NullString>,
    index: i32,
}

#[derive(Debug, BinRead)]
struct SprDbSpriteReader {
    id: u32,
    name: FilePtr32<NullString>,
    index: u16,
    set_index: u16,
}

impl From<SprDbReader> for SprDb {
    fn from(db: SprDbReader) -> Self {
        let sets = db.sets.into_inner();
        let mut sets: BTreeMap<u32, _> = sets.into_iter().map(Into::into).collect();
        let sprites = db.sprites.into_inner();
        for sprite in sprites.into_iter() {
            let spr_set_index = (sprite.set_index & 0xFFF) as i32;
            let set = sets.values_mut().find(|x| x.index == spr_set_index);
            if let Some(set) = set {
                if sprite.is_texture() {
                    set.textures.insert(sprite.id, sprite.into());
                } else {
                    set.sprites.insert(sprite.id, sprite.into());
                }
            }
        }
        Self { sets }
    }
}

impl SprDbSpriteReader {
    const fn is_texture(&self) -> bool {
        self.set_index & 0x1000 == 0x1000
    }
}

impl From<SprDbSetReader> for SprDbSet {
    fn from(set: SprDbSetReader) -> Self {
        Self {
            index: set.index,
            name: set.name.to_string(),
            filename: set.filename.to_string(),
            ..Default::default()
        }
    }
}

impl Into<(u32, SprDbSet)> for SprDbSetReader {
    fn into(self) -> (u32, SprDbSet) {
        (self.id, self.into())
    }
}

impl From<SprDbSpriteReader> for SprDbEntry {
    fn from(sprite: SprDbSpriteReader) -> Self {
        Self {
            index: sprite.index,
            name: sprite.name.to_string(),
        }
    }
}

impl Into<(u32, SprDbEntry)> for SprDbSpriteReader {
    fn into(self) -> (u32, SprDbEntry) {
        (self.id, self.into())
    }
}

impl SprDb {
    /// Read a [`SprDb`] from a [`Read`] instance
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
    pub fn read<R: Read + Seek>(mut reader: R) -> BinResult<Self> {
        reader.read_ne::<SprDbReader>().map(Into::into)
    }

    /// Read a [`SprDb`] from a byte slice
    ///
    /// This function takes in anything that can be cheaply converted into a `&[u8]` and reads it.
    /// It is an ergonomic wrapper around [`SprDb::read`]
    ///
    /// ```rust
    /// use diva_db::spr::SprDb;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let bytes = std::fs::read("assets/aft_spr_db.bin")?;
    /// let db = SprDb::from_bytes(bytes)?;
    /// assert_eq!(db.sets.len(), 2983);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes<R: AsRef<[u8]>>(bytes: R) -> BinResult<Self> {
        Self::read(std::io::Cursor::new(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = include_bytes!("../../assets/aft_spr_db.bin");

    #[test]
    fn read() {
        let mut reader = std::io::Cursor::new(INPUT);
        let db = SprDb::read(reader).unwrap();
        assert_eq!(db.sets.len(), 2983);
        assert_eq!(db.sets.values().map(|x| x.sprites.len()).max(), Some(344));
        assert_eq!(db.sets.values().map(|x| x.textures.len()).max(), Some(99));
    }
}
