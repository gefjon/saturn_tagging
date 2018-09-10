#![feature(try_from, const_fn)]

crate mod type_tags;
crate mod bit_utils;
crate mod pointer_tags;

pub use crate::type_tags::{TypeId, ThinTypeId, TypeError, TypeIdTooLargeError};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
