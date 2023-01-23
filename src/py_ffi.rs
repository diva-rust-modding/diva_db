use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::PyErr;

use super::mot::py_ffi::*;
use super::tex::py_ffi::*;
use crate::bone::py_ffi::*;
use crate::spr::py_ffi::*;

pub(crate) struct PyBinrwError(pub binrw::Error);

impl From<PyBinrwError> for PyErr {
    fn from(err: PyBinrwError) -> Self {
        PyErr::new::<PyIOError, _>(err.0.to_string())
    }
}

impl From<binrw::Error> for PyBinrwError {
    fn from(err: binrw::Error) -> Self {
        Self(err)
    }
}

#[pymodule]
fn diva_db(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_submodule(bone(py)?);
    m.add_submodule(tex(py)?);
    m.add_submodule(motset(py)?);
    m.add_submodule(spr(py)?);

    Ok(())
}
