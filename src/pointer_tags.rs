use crate::bit_utils;

//#[derive(Copy, Clone, Debug, PartialEq, Eq)]
// A tag suitable for 
//pub struct PointerTag(u8);

#[cfg(test)]
mod test {
    #[test]
    /// A `u8` has an align of 1, but heap allocations even of 1 byte
    /// will always be 8-byte aligned on x86.
    fn pointer_align_8() {
        use std::boxed::Box;
        for n in 0..=255 {
            let box_n: Box<u8> = Box::new(n);
            let ptr = Box::into_raw(box_n);
            assert_eq!((ptr as usize) & 0b111, 0);
            let box_again = unsafe { Box::from_raw(ptr) };
        }
    }
    #[test]
    /// A `u8` has an align of 1, but heap allocations even for 1 byte
    /// will always be 16-byte aligned on x86_64.
    fn pointer_align_16() {
        use std::boxed::Box;
        for n in 0..=255 {
            let box_n: Box<u8> = Box::new(n);
            let ptr = Box::into_raw(box_n);
            assert_eq!((ptr as usize) & 0xf, 0);
            let box_again = unsafe { Box::from_raw(ptr) };
        }
    }
}
