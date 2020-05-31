use pyo3::prelude::*;
use pyo3::wrap_pymodule;

use super::*;

#[pymodule]
fn diva_db(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use crate::bone::py_ffi::*;
    m.add_wrapped(wrap_pymodule!(bone));

    Ok(())
}
