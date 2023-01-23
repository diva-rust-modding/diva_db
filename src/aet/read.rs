use super::*;

use binrw::prelude::*;
use binrw::*;
use std::io::Cursor;

#[derive(Debug, BinRead)]
struct AetDbReader {
    set_count: u32,
    #[br(count = set_count)]
    sets: FilePtr32<Vec<AetDbSetReader>>,
    scene_count: u32,
    #[br(count = scene_count)]
    scenes: FilePtr32<Vec<AetDbSceneReader>>,
}

#[derive(Debug, BinRead)]
struct AetDbSetReader {
    id: u32,
    name: FilePtr32<NullString>,
    filename: FilePtr32<NullString>,
    index: i32,
    spr_set_id: u32,
}

#[derive(Debug, BinRead)]
struct AetDbSceneReader {
    id: u32,
    name: FilePtr32<NullString>,
    index: u16,
    set_index: u16,
}

impl AetDb {
    pub fn read(path: String, spr_db: &spr::SprDb) -> Option<Self> {
        let bytes = std::fs::read(path).ok()?;
        let mut reader = Cursor::new(bytes);
        let mut aet_db: AetDbReader = reader.read_ne().ok()?;
        let mut out = Vec::with_capacity(aet_db.set_count as usize);

        aet_db.sets.sort_by(|a, b| a.index.cmp(&b.index));
        for set in aet_db.sets.iter() {
            let spr_set = match spr_db.get(set.spr_set_id) {
                Some(spr_set) => spr_set,
                None => continue,
            };
            let spr_set = spr_set.to_owned();
            out.push(AetDbSet {
                id: set.id,
                name: set.name.to_string(),
                filename: set.filename.to_string(),
                index: set.index,
                spr_set: spr_set,
                scenes: vec![],
            });
        }

        for scene in aet_db.scenes.iter() {
            let aet_set = match out.get_mut(scene.set_index as usize) {
                Some(aet_set) => aet_set,
                None => continue,
            };
            aet_set.scenes.push(AetDbScene {
                id: scene.id,
                name: scene.name.to_string(),
                index: scene.index,
            });
        }

        Some(Self { sets: out })
    }

    pub fn get(&self, id: u32) -> Option<&AetDbSet> {
        self.sets.iter().find(|set| set.id == id)
    }
}
