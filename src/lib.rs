#![cfg_attr(feature = "no_std",  no_std)]
#![feature(try_from, const_fn)]

pub mod type_tags;
pub mod bit_utils;
pub mod pointer_tags;

pub use crate::type_tags::{TypeId, ThinTypeId, TypeError, TypeIdTooLargeError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
