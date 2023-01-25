//! Python bindings for [`diva_db`](crate)
//!
//! # Limitations
//! Due to limitations in [`pyo3`], the following is not possible directly,
//!
//! ## Submodules
//! According to Issues [#759](https://github.com/PyO3/pyo3/issues/759) and [#1517](https://github.com/PyO3/pyo3/issues/1517#issuecomment-808664021),
//! it is currently not possible to define python packages.
//!
//! For users of `diva_db`, this means that it is not possible to directly import a submodule.
//! That is,
//!
//! ```python
//! >>> import diva_db.aet # this doesn't work
//! >>> from diva_db.aet import AetDb # this doesn't work as well
//! >>> from diva_db import aet # this works
//! ````
//!
//! ## Updating `dict`
//! currently updating dictionaries has to be explicit.
//! That is,
//!
//! ```python
//! >>> import diva_db
//!
//! >>> db = diva_db.aet.AetDb({})
//! >>> set = diva_db.aet.AetDbSet(0, "test", "test.bin", {}, 0)
//! >>> db.sets = { 39: set }
//! >>> db.sets[39]
//! AetDbSet(0: 'test' @ "test.bin"): 0 scene(s)
//! >>> # This doesn't work
//! >>> db.sets[10] = set
//! >>> db.sets[10]
//! Traceback (most recent call last):
//!   File "<stdin>", line 1, in <module>
//! KeyError: 10
//! ````

use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::PyErr;

use super::mot::py_ffi::*;
use super::tex::py_ffi::*;
use crate::aet::py_ffi::*;
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
    m.add_submodule(aet(py)?);

    Ok(())
}
