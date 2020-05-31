use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use super::*;

#[pyclass(module = "bones")]
#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct PyBoneDatabase {
    #[pyo3(get, set)]
    pub signature: u32,
    #[pyo3(get, set)]
    pub skeletons: Vec<PySkeleton>,
}

#[pyclass(module = "bones")]
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct PySkeleton {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub bones: Vec<PyBone>,
    #[pyo3(get, set)]
    pub pos: Vec<Vec3>,
    #[pyo3(get, set)]
    pub parent_ids: Vec<i16>,

    #[pyo3(get, set)]
    pub object_bone_names: Vec<String>,
    #[pyo3(get, set)]
    pub motion_bone_names: Vec<String>,
}

#[pyclass(module = "bones")]
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct PyBone {
    // pub mode: BoneType,
    #[pyo3(get, set)]
    pub parent: u8,
    #[pyo3(get, set)]
    pub pole_target: u8, //only set in type 5 bones
    #[pyo3(get, set)]
    pub mirror: u8,
    #[pyo3(get, set)]
    pub unk2: u8,
    #[pyo3(get, set)]
    pub name: String
}

impl From<BoneDatabase<'_>> for PyBoneDatabase {
    fn from(bonedb: BoneDatabase<'_>) -> Self {
        let BoneDatabase { signature, skeletons } = bonedb;

        let skeletons = skeletons.into_iter().map(Into::into).collect();

        Self { signature, skeletons }
    }
}
impl From<Skeleton<'_>> for PySkeleton {
    fn from(skel: Skeleton<'_>) -> Self {
        let Skeleton { name, bones, pos, parent_ids, object_bone_names, motion_bone_names } = skel;

        let name = name.into_owned();
        let bones = bones.into_iter().map(Into::into).collect();
        let object_bone_names = object_bone_names.into_iter().map(Into::into).collect();
        let motion_bone_names = motion_bone_names.into_iter().map(Into::into).collect();

        Self { name, bones, pos, parent_ids, object_bone_names, motion_bone_names }
    }
}
impl From<Bone<'_>> for PyBone {
    fn from(bone: Bone<'_>) -> Self {
        let Bone { parent, pole_target, mirror, unk2, name, mode } = bone;

        let parent = parent.unwrap_or(255);
        let pole_target = pole_target.unwrap_or(255);
        let mirror = mirror.unwrap_or(255);

        let name = name.into_owned();

        Self { parent, pole_target, mirror, name, unk2 }
    }
}

use std::fs::File;
use std::io::Read;

#[pyfunction]
fn read_db(path: String) -> PyResult<PyBoneDatabase> {
    let mut file = File::open(path)?;
    let mut input = vec![];
    file.read_to_end(&mut input);
    let (_, bone_db) = BoneDatabase::read(&input).unwrap();
    Ok(bone_db.into())
}

#[pymodule]
fn bone(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use crate::bone;
    m.add_wrapped(wrap_pyfunction!(read_db));
    m.add_class::<PyBoneDatabase>();
    m.add_class::<PySkeleton>();
    m.add_class::<PyBone>();

    Ok(())
}
