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

impl SprDb {
    pub fn read<R: Read + Seek>(mut reader: R) -> Option<Self> {
        let mut spr_db: SprDbReader = reader.read_ne().ok()?;
        let mut out = BTreeMap::new();

        spr_db.sets.sort_by(|a, b| a.index.cmp(&b.index));
        for set in spr_db.sets.iter() {
            out.insert(
                set.id,
                SprDbSet {
                    name: set.name.to_string(),
                    filename: set.filename.to_string(),
                    sprites: BTreeMap::new(),
                    textures: BTreeMap::new(),
                    index: set.index,
                },
            );
        }

        for sprite in spr_db.sprites.iter() {
            let spr_set_index = (sprite.set_index & 0xFFF) as usize;
            let entry = SprDbEntry {
                name: sprite.name.to_string(),
                index: sprite.index,
            };
            let spr_set = match out
                .iter_mut()
                .find(|(_, v)| v.index == spr_set_index as i32)
            {
                Some(spr_set) => spr_set,
                None => continue,
            };
            if sprite.set_index & 0x1000 == 0x1000 {
                spr_set.1.textures.insert(sprite.id, entry);
            } else {
                spr_set.1.sprites.insert(sprite.id, entry);
            }
        }

        Some(Self { sets: out })
    }
}
