use log::*;

#[cfg(feature="pyo3")]
mod py_ffi;
pub mod aet;
pub mod bone;
pub mod tex;
pub mod mot;
pub mod spr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
