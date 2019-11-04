use diva_db::bone::*;
use serde_json::*;

const INPUT: &[u8] = include_bytes!("../assets/bone_data.bin");

fn main() {
    let db = BoneDatabase::read(INPUT).unwrap().1;
    println!("{}", serde_json::to_string_pretty(&db).unwrap());
    let fst = &db.skeletons[0];
    println!("bones, pos, parents, obj, mot");
    println!("len of fst entry {} {} {} {} {}", fst.bones.len(), fst.pos.len(), fst.parent_ids.len(), fst.object_bone_names.len(), fst.motion_bone_names.len())
}
