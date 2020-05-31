use super::*;

const INPUT: &[u8] = include_bytes!("../../assets/aft_mot_db.bin");

#[test]
fn read_test() {
    let (_, mot_db) = MotionSetDatabase::read(Endianness::Little)(INPUT).unwrap();
    println!("{:#?}", mot_db);
    println!("{:#?}", mot_db.bones.len());
    //find all ude
    let udes = mot_db.bones.iter().enumerate().filter(|(_, n)| n.contains("ude"));
    for (i, ude) in udes {
        println!("{}: {}", i, ude);
    }
}
