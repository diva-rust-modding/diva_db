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
    #[pyo3(get, set)]
    pub mode: u8,
    #[pyo3(get, set)]
    pub parent: Option<u8>,
    #[pyo3(get, set)]
    pub pole_target: Option<u8>, //only set in type 5 bones
    #[pyo3(get, set)]
    pub mirror: Option<u8>,
    #[pyo3(get, set)]
    pub unk2: u8,
    #[pyo3(get, set)]
    pub name: String,
}

impl From<BoneDatabase<'_>> for PyBoneDatabase {
    fn from(bonedb: BoneDatabase<'_>) -> Self {
        let BoneDatabase {
            signature,
            skeletons,
        } = bonedb;

        let skeletons = skeletons.into_iter().map(Into::into).collect();

        Self {
            signature,
            skeletons,
        }
    }
}
impl From<Skeleton<'_>> for PySkeleton {
    fn from(skel: Skeleton<'_>) -> Self {
        let Skeleton {
            name,
            bones,
            pos,
            parent_ids,
            object_bone_names,
            motion_bone_names,
        } = skel;

        let name = name.into_owned();
        let bones = bones.into_iter().map(Into::into).collect();
        let object_bone_names = object_bone_names.into_iter().map(Into::into).collect();
        let motion_bone_names = motion_bone_names.into_iter().map(Into::into).collect();

        Self {
            name,
            bones,
            pos,
            parent_ids,
            object_bone_names,
            motion_bone_names,
        }
    }
}
impl From<Bone<'_>> for PyBone {
    fn from(bone: Bone<'_>) -> Self {
        let Bone {
            parent,
            pole_target,
            mirror,
            unk2,
            name,
            mode,
        } = bone;

        // let parent = parent.unwrap_or(255);
        // let pole_target = pole_target.unwrap_or(255);
        // let mirror = mirror.unwrap_or(255);

        let mode = mode as u8;
        let name = name.into_owned();

        Self {
            parent,
            pole_target,
            mirror,
            name,
            unk2,
            mode,
        }
    }
}

#[pymethods]
impl PyBoneDatabase {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PyBoneDatabase({:X}): {} skeletons",
            self.signature,
            self.skeletons.len()
        ))
    }
}

#[pymethods]
impl PySkeleton {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PySkeleton: {}, {} bone(s)",
            self.name,
            self.bones.len(),
        ))
    }
}

impl PyBone {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("PyBone({} type {})", self.name, self.mode,))
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

pub(crate) fn bone(py: Python<'_>) -> PyResult<&PyModule> {
    use crate::bone;
    let m = PyModule::new(py, "bone")?;
    m.add_wrapped(wrap_pyfunction!(read_db));
    m.add_class::<PyBoneDatabase>();
    m.add_class::<PySkeleton>();
    m.add_class::<PyBone>();

    Ok(m)
}
