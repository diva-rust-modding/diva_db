use crate::bone::*;
use crate::*;

mod read {
    use crate::bone::*;
    use crate::*;
    const INPUT: &[u8] = include_bytes!("../../assets/f_bonedb.bin");
    const NAMES: &[&str] = &[
        "CMN", "HAK", "KAI", "LEN", "LUK", "MEI", "MIK", "NER", "RIN", "SAK", "TET",
    ];

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn bonedb_read() {
        init();

        let db = BoneDatabase::read(INPUT).unwrap().1;
        let db_names = db.skeletons.iter().map(|s| &s.name).collect::<Vec<_>>();

        assert_eq!(db_names, NAMES);
    }

    #[test]
    fn skel_read() {
        init();

        let skel = Skeleton::read(INPUT)(&INPUT[0x180BC..]).unwrap().1;
        let bone = Bone {
            name: "n_hara_cp".into(),
            mode: BoneType::Type3,
            parent: None,
            pole_target: None,
            mirror: None,
            unk2: 0
        };
        assert_eq!(skel.bones[0], bone);
        assert_eq!(skel.bones.len(), 177);
        assert_eq!(skel.pos[0], [0., 0., 0.]);
        assert_eq!(skel.parent_ids[0], -1);
    }

    #[test]
    fn bone_read() {
        let bone = Bone::read(INPUT)(&INPUT[0x16774..]).unwrap().1;
        let bone_res = Bone {
            name: "n_hara_cp".into(),
            mode: BoneType::Type3,
            parent: None,
            pole_target: None,
            mirror: None,
            unk2: 0
        };
        assert_eq!(bone, bone_res);
    }
}
