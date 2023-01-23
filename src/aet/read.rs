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
    pub fn read(path: String) -> Option<Self> {
        let bytes = std::fs::read(path).ok()?;
        let mut reader = Cursor::new(bytes);
        let mut aet_db: AetDbReader = reader.read_ne().ok()?;
        let mut out = BTreeMap::new();

        aet_db.sets.sort_by(|a, b| a.index.cmp(&b.index));
        for set in aet_db.sets.iter() {
            out.insert(
                set.id,
                AetDbSet {
                    name: set.name.to_string(),
                    filename: set.filename.to_string(),
                    spr_set_id: set.spr_set_id,
                    scenes: BTreeMap::new(),
                    index: set.index,
                },
            );
        }

        for scene in aet_db.scenes.iter() {
            let aet_set = match out
                .iter_mut()
                .find(|(_, v)| v.index == scene.set_index.into())
            {
                Some(aet_set) => aet_set,
                None => continue,
            };
            aet_set.1.scenes.insert(
                scene.id,
                AetDbScene {
                    name: scene.name.to_string(),
                    index: scene.index,
                },
            );
        }

        Some(Self { sets: out })
    }
}
