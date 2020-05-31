use log::*;

#[cfg(feature="pyo3")]
mod py_ffi;
pub mod bone;
pub mod mot;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
