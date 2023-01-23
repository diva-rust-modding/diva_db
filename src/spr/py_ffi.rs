use super::*;
use crate::py_ffi::*;
use pyo3::prelude::*;

#[pymethods]
impl SprDb {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SprDb: {} set(s)", self.sets.len()))
    }
}

#[pymethods]
impl SprDbSet {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "SprDbSet({}: {} @ {:?}): {} texture(s), {} sprite(s)",
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
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SprDbEntry({}: {})", self.index, self.name))
    }
}

#[pyfunction]
fn read(bytes: &[u8]) -> PyResult<SprDb> {
    let mut cursor = std::io::Cursor::new(bytes);
    SprDb::read(cursor)
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
