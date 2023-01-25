//! Python bindings for `spr`
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
//! bytes = open("spr_db.bin", "rb").read()
//! db = diva_db.spr.read(bytes)
//! for set in db.sets:
//!     print(set)
//! ```

use super::*;
use crate::py_ffi::*;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pymethods]
impl SprDb {
    #[new]
    fn py_new(sets: BTreeMap<u32, SprDbSet>) -> Self {
        Self { sets }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SprDb: {} set(s)", self.sets.len()))
    }
}

#[pymethods]
impl SprDbSet {
    #[new]
    fn py_new(
        index: i32,
        name: String,
        filename: String,
        sprites: BTreeMap<u32, SprDbEntry>,
        textures: BTreeMap<u32, SprDbEntry>,
    ) -> Self {
        Self {
            index,
            name,
            filename,
            sprites,
            textures,
        }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "SprDbSet({}: '{}' @ {:?}): {} texture(s), {} sprite(s)",
            self.index,
            self.name,
            self.filename,
            self.textures.len(),
            self.sprites.len()
        ))
    }
}

#[pymethods]
impl SprDbEntry {
    #[new]
    fn py_new(index: u16, name: String) -> Self {
        Self { index, name }
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SprDbEntry({}: '{}')", self.index, self.name))
    }
}

/// Read a `SprDb` from `bytes`.
#[pyfunction]
fn read(bytes: &PyBytes) -> PyResult<SprDb> {
    SprDb::from_bytes(bytes.as_bytes())
        .map_err(PyBinrwError)
        .map_err(Into::into)
}

pub(crate) fn spr(py: Python<'_>) -> PyResult<&PyModule> {
    use crate::bone;
    let m = PyModule::new(py, "spr")?;
    m.add_wrapped(wrap_pyfunction!(read));
    m.add_class::<SprDb>();
    m.add_class::<SprDbSet>();
    m.add_class::<SprDbEntry>();
    Ok(m)
}
