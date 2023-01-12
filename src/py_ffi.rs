use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use super::mot::py_ffi::*;
use super::tex::py_ffi::*;
use crate::bone::py_ffi::*;

#[pymodule]
fn diva_db(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_submodule(bone(py)?);
    m.add_submodule(tex(py)?);
    m.add_submodule(motset(py)?);

    Ok(())
}
