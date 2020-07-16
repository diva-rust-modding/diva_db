use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use super::*;
use super::tex::py_ffi::*;
use super::mot::py_ffi::*;

#[pymodule]
fn diva_db(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use crate::bone::py_ffi::*;
    m.add_wrapped(wrap_pymodule!(bone));
    m.add_wrapped(wrap_pymodule!(tex));
    m.add_wrapped(wrap_pymodule!(mot));

    Ok(())
}
