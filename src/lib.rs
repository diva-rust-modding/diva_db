use log::*;

pub mod aet;
pub mod bone;
pub mod mot;
#[cfg(feature = "pyo3")]
pub mod py_ffi;
pub mod spr;
pub mod tex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
