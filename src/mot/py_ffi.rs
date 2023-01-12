use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use super::*;

#[pyclass]
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct PyMotionSetInfo {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    mots: BTreeMap<u32, String>,
}

#[pyclass]
#[derive(Debug)]
pub struct PyMotionSetDatabase {
    #[pyo3(get)]
    pub signature: u32,
    #[pyo3(get, set)]
    pub sets: BTreeMap<u32, PyMotionSetInfo>,
    #[pyo3(get, set)]
    pub bones: Vec<String>,
}

impl<'a> From<MotionSetInfo<'a>> for PyMotionSetInfo {
    fn from(info: MotionSetInfo<'a>) -> Self {
        let MotionSetInfo { name, mots } = info;
        let mots = mots.into_iter().map(|(x, y)| (x, y.into())).collect();
        Self {
            name: name.into_owned(),
            mots,
        }
    }
}

impl<'a> From<MotionSetDatabase<'a>> for PyMotionSetDatabase {
    fn from(db: MotionSetDatabase<'a>) -> Self {
        let MotionSetDatabase {
            sets,
            bones,
            signature,
        } = db;
        let sets = sets.into_iter().map(|(x, y)| (x, y.into())).collect();
        let bones = bones.into_iter().map(Into::into).collect();
        Self {
            signature: db.signature,
            sets,
            bones,
        }
    }
}

#[pymethods]
impl PyMotionSetInfo {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PyMotionSetInfo: {}, {} mot(s)",
            self.name,
            self.mots.len()
        ))
    }
}

#[pymethods]
impl PyMotionSetDatabase {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PyMotionSetDatabase({:X}): {} sets, {} bones",
            self.signature,
            self.sets.len(),
            self.bones.len(),
        ))
    }
}

#[pyfunction]
fn read_db(path: String) -> PyResult<PyMotionSetDatabase> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut input = vec![];
    file.read_to_end(&mut input);
    let (_, mot_db) = MotionSetDatabase::read(&input).unwrap();
    Ok(mot_db.into())
}

pub(crate) fn motset(py: Python<'_>) -> PyResult<&PyModule> {
    use crate::bone;
    let m = PyModule::new(py, "motset")?;
    m.add_wrapped(wrap_pyfunction!(read_db));
    m.add_class::<PyMotionSetInfo>();
    m.add_class::<PyMotionSetDatabase>();
    Ok(m)
}
