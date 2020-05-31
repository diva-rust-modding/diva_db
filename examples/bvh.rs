use bstr::BStr;
use bvh_anim::builder::*;
use bvh_anim::ChannelType::*;
use diva_db::bone::*;
use mint::Vector3;

use std::fs::File;
use std::time::Duration;

const FPS: f64 = 60.;
const INPUT: &[u8] = include_bytes!("../assets/d2nd_bonedb.bin");

fn main() {
    let db = BoneDatabase::read(INPUT).unwrap().1;
    let skel = &db.skeletons[0];
    let root = &skel.bones[0];
    let channel = &[];
    let mut builder = Builder::with_root_joint(
        (&root.name[..]).into(),
        Vector3::from_slice(&skel.pos[0]),
        channel,
    );
    println!("{}: {:?}", root.name, Vector3::from_slice(&skel.pos[0]));
    println!("{:?}", builder);
    for (id, bone) in skel.bones.iter().enumerate().skip(1) {
        let par_pos = &skel.pos[id - 1];
        let pos = &skel.pos[id];
        let new_pos = [
            pos[0] - par_pos[0],
            pos[1] - par_pos[1],
            pos[2] - par_pos[2],
        ];
        let vec = Vector3::from_slice(&new_pos);
        let par = std::cmp::min(bone.unk[0] as usize, id - 1);
        println!("{} {:?} par: {}", id, bone, par);
        // println!("{:3} {}: {:?}", id, bone.name, vec);
        // builder = builder.push_child(id, (&bone.name[..]).into(), vec, channel);
        builder = builder.push_child_with_parent(
            0,
            (&bone.name[..]).into(),
            vec,
            channel,
            bone.unk[0] as usize,
        );
    }
    // let builder = builder.push_end(Vector3 {
    //     x: 1.,
    //     y: 2.,
    //     z: 3.,
    // });
    let mut mot = builder.with_motion(0, Duration::from_secs_f64(1.0 / FPS));
    let bvh = mot.build().unwrap();
    let mut file = File::create("./test.bvh").unwrap();
    bvh.write_to(&mut file).unwrap();
}
