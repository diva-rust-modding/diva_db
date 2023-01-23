mod read;

#[derive(Debug, Clone)]
pub struct SprDb {
    pub sets: Vec<SprDbSet>,
}

#[derive(Debug, Clone)]
pub struct SprDbSet {
    pub id: u32,
    pub name: String,
    pub filename: String,
    pub sprites: Vec<SprDbEntry>,
    pub textures: Vec<SprDbEntry>,
}

#[derive(Debug, Clone)]
pub struct SprDbEntry {
    pub id: u32,
    pub name: String,
    pub index: u16,
}

impl SprDb {
    pub fn get(&self, id: u32) -> Option<&SprDbSet> {
        self.sets.iter().find(|set| set.id == id)
    }
}
