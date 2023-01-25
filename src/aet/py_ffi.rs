//! Python bindings for `aet`
//!
//! All functions in this module are exposed to python via [`pyo3`].
//!
//! See [`diva_db::py_ffi`][crate::py_ffi] for more information.
//!
//! # Usage
//!
//! ```python
//! import diva_db
//!
//! bytes = open("aet_db.bin", "rb").read()
//! db = diva_db.aet.read(bytes)
//! for set in db.sets:
//!     print(set)
//! ```

use pyo3::prelude::*;
use pyo3::types::PyBytes;

use super::*;
use crate::py_ffi::*;
use crate::spr::{SprDb, SprDbSet};

#[pymethods]
impl AetDb {
    #[new]
    fn py_new(sets: BTreeMap<u32, AetDbSet>) -> Self {
        Self { sets }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("AetDb: {} set(s)", self.sets.len()))
    }
}

#[pymethods]
impl AetDbSet {
    #[new]
    fn new(
        index: i32,
        name: String,
        filename: String,
        scenes: BTreeMap<u32, AetDbScene>,
        spr_set_id: u32,
    ) -> Self {
        Self {
            index,
            name,
            filename,
            scenes,
            spr_set_id,
        }
    }
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
    #[new]
    fn py_new(index: u16, name: String) -> Self {
        Self { index, name }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("AetDbScene({}: '{}')", self.index, self.name))
    }
}

/// Read an `AetDb` from `bytes`.
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
