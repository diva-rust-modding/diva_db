use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use super::*;

#[pyclass]
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct PyMotionInfo {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    id: u32,
}

#[pyclass]
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct PyMotionSetInfo {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    id: u32,
    #[pyo3(get, set)]
    mots: Vec<PyMotionInfo>,
}

#[pyclass]
#[derive(Debug)]
pub struct PyMotionSetDatabase {
    #[pyo3(get, set)]
    pub sets: Vec<PyMotionSetInfo>,
    #[pyo3(get, set)]
    pub bones: Vec<String>,
}

impl<'a> From<MotionInfo<'a>> for PyMotionInfo {
    fn from(info: MotionInfo<'a>) -> Self {
        let MotionInfo { name, id } = info;
        Self { id, name: name.into_owned() }
    }
}

impl<'a> From<MotionSetInfo<'a>> for PyMotionSetInfo {
    fn from(info: MotionSetInfo<'a>) -> Self {
        let MotionSetInfo { name, id, mots } = info;
        let mots = mots.into_iter().map(Into::into).collect();
        Self { id, name: name.into_owned(), mots }
    }
}

impl<'a> From<MotionSetDatabase<'a>> for PyMotionSetDatabase {
    fn from(db: MotionSetDatabase<'a>) -> Self {
        let MotionSetDatabase { sets, bones } = db;
        let sets = sets.into_iter().map(Into::into).collect();
        let bones = bones.into_iter().map(Into::into).collect();
        Self { sets, bones }
    }
}

#[pyfunction]
fn read_db(path: String) -> PyResult<PyMotionSetDatabase> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut input = vec![];
    file.read_to_end(&mut input);
    let (_, mot_db) = MotionSetDatabase::read(Endianness::Little)(&input).unwrap();
    Ok(mot_db.into())
}

#[pymodule]
fn mot(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use crate::bone;
    m.add_wrapped(wrap_pyfunction!(read_db));
    m.add_class::<PyMotionInfo>();
    m.add_class::<PyMotionSetInfo>();
    m.add_class::<PyMotionSetDatabase>();

    Ok(())
}
