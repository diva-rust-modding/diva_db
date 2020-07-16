use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::collections::BTreeMap;

use super::*;

#[pyclass]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PyTextureDatabase {
    #[pyo3(get, set)]
    pub entries: BTreeMap<usize, String>,
}

impl<'a> From<TextureDatabase<'a>> for PyTextureDatabase {
    fn from(db: TextureDatabase<'a>) -> Self {
        let entries = db.entries.into_iter().map(|(k, v)| (k, v.into_owned())).collect();
        Self { entries }
    }
}

impl<'a> From<PyTextureDatabase> for TextureDatabase<'a> {
    fn from(db: PyTextureDatabase) -> Self {
        let entries = db.entries.into_iter().map(|(k, v)| (k, v.into())).collect();
        Self { entries }
    }
}

#[pyfunction]
fn read_db(path: String) -> PyResult<PyTextureDatabase> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut input = vec![];
    file.read_to_end(&mut input)?;
    let (_, bone_db) = TextureDatabase::parse(&input).unwrap();
    Ok(bone_db.into())
}

#[pymodule]
fn tex(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use crate::bone;
    m.add_wrapped(wrap_pyfunction!(read_db));
    m.add_class::<PyTextureDatabase>();

    Ok(())
}
