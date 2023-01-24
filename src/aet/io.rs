use super::*;

use binrw::io::{Read, Seek};
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

impl From<AetDbReader> for AetDb {
    fn from(db: AetDbReader) -> Self {
        let sets = db.sets.into_inner();
        let scenes = db.scenes.into_inner();
        let mut sets: BTreeMap<u32, AetDbSet> = sets.into_iter().map(Into::into).collect();
        for scene in scenes.into_iter() {
            if let Some(set) = sets.get_mut(&scene.set_index.into()) {
                set.scenes.insert(scene.id, scene.into());
            }
        }
        Self { sets }
    }
}

impl From<AetDbSetReader> for AetDbSet {
    fn from(set: AetDbSetReader) -> Self {
        Self {
            name: set.name.to_string(),
            filename: set.filename.to_string(),
            spr_set_id: set.spr_set_id,
            index: set.index,
            ..Default::default()
        }
    }
}

impl Into<(u32, AetDbSet)> for AetDbSetReader {
    fn into(self) -> (u32, AetDbSet) {
        (self.id, AetDbSet::from(self))
    }
}

impl From<AetDbSceneReader> for AetDbScene {
    fn from(scene: AetDbSceneReader) -> Self {
        AetDbScene {
            name: scene.name.to_string(),
            index: scene.index,
        }
    }
}

// Orphan rules pervent us from writing using `From<_>`
impl Into<(u32, AetDbScene)> for AetDbSceneReader {
    fn into(self) -> (u32, AetDbScene) {
        (self.id, AetDbScene::from(self))
    }
}

impl AetDb {
    pub fn read<R: Read + Seek>(mut reader: R) -> BinResult<Self> {
        reader.read_ne::<AetDbReader>().map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = include_bytes!("../../assets/aft_aet_db.bin");

    #[test]
    fn read() {
        let mut reader = std::io::Cursor::new(INPUT);
        let db = AetDb::read(reader).unwrap();
        assert_eq!(db.sets.len(), 1063);
        assert_eq!(db.sets.values().map(|x| x.scenes.len()).max(), Some(2));
    }
}
