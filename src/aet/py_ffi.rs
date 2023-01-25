use pyo3::prelude::*;
use pyo3::types::PyBytes;

use super::*;
use crate::py_ffi::*;
use crate::spr::{SprDb, SprDbSet};

#[pymethods]
impl AetDb {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("AetDb: {} set(s)", self.sets.len()))
    }
}

#[pymethods]
impl AetDbSet {
    fn get_spr_set(&self, db: SprDb) -> Option<SprDbSet> {
        db.sets.get(&self.spr_set_id).cloned()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "AetDbSet({}: '{}' @ {:?}): {} scene(s)",
            self.index,
            self.name,
            self.filename,
            self.scenes.len()
        ))
    }
}

#[pymethods]
impl AetDbScene {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("AetDbScene({}: '{}')", self.index, self.name))
    }
}

#[pyfunction]
pub fn read(bytes: &PyBytes) -> PyResult<AetDb> {
    AetDb::from_bytes(bytes.as_bytes())
        .map_err(PyBinrwError)
        .map_err(Into::into)
}

pub(crate) fn aet(py: Python<'_>) -> PyResult<&PyModule> {
    use crate::bone;
    let m = PyModule::new(py, "aet")?;
    m.add_wrapped(wrap_pyfunction!(read));
    m.add_class::<AetDb>();
    m.add_class::<AetDbScene>();
    m.add_class::<AetDbSet>();
    Ok(m)
}
