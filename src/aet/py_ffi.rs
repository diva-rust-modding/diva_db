use super::*;
use crate::py_ffi::*;
use pyo3::prelude::*;

#[pymethods]
impl AetDb {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("AetDb: {} set(s)", self.sets.len()))
    }
}

#[pymethods]
impl AetDbSet {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "AetDbSet({}: {} @ {:?}): {} scene(s)",
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
        Ok(format!("AetDbScene({}: {})", self.index, self.name))
    }
}

#[pyfunction]
fn read(bytes: &[u8]) -> PyResult<AetDb> {
    let mut cursor = std::io::Cursor::new(bytes);
    AetDb::read(cursor)
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
