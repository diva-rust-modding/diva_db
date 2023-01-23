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
    pub fn read(path: String) -> Option<Self> {
        let bytes = std::fs::read(path).ok()?;
        let mut reader = Cursor::new(bytes);
        let mut spr_db: SprDbReader = reader.read_ne().ok()?;
        let mut out = Vec::with_capacity(spr_db.set_count as usize);

        spr_db.sets.sort_by(|a, b| a.index.cmp(&b.index));
        for set in spr_db.sets.iter() {
            out.push(SprDbSet {
                id: set.id,
                name: set.name.to_string(),
                filename: set.filename.to_string(),
                sprites: vec![],
                textures: vec![],
            });
        }

        for sprite in spr_db.sprites.iter() {
            let spr_set_index = (sprite.set_index & 0xFFF) as usize;
            let entry = SprDbEntry {
                id: sprite.id,
                name: sprite.name.to_string(),
                index: sprite.index,
            };
            let spr_set = match out.get_mut(spr_set_index) {
                Some(spr_set) => spr_set,
                None => continue,
            };
            if sprite.set_index & 0x1000 == 0x1000 {
                spr_set.textures.push(entry);
            } else {
                spr_set.sprites.push(entry);
            }
        }

        Some(Self { sets: out })
    }
}
