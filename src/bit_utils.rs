//! NaN-boxing relies on the fact that, in the ANSI double-width
//! floating specification (`f64`), all values with the exponent set
//! to all ones are reserved. Only two of these values are used -
//! `0x1` denotes `NaN` and `0x0` denotes `Infinity` (actually, any
//! non-zero bit sequence is a `NaN`, but only `0x1` is used by Intel
//! and ARM chips). That leaves `2^52 -2` available bit combinations,
//! not counting the sign bit. We use 4 of those bits (`TAG_MASK`) to
//! mark the type of value, leaving the low 48 bits (plus the sign
//! bit) to hold the payload. This means that `2^4 = 16` different
//! 48-bit immediate types can be stored inline (as long as `0x0` and
//! `0x1` are not valid instances of type `0x0`, to avoid collision
//! with NaN and Infinity).
//!
//! If even more polymorphism is needed, the low 3 bits
//! (`POINTER_TAG_MASK`) can be used in pointer types as an additional
//! tag, since modern allocators only allocate 8-byte aligned blocks.

pub const NAN_MASK: u64 = 0x7ff << 52;
pub const TAG_SHIFT: usize = 48;
pub const TAG_MASK: u64 = 0xf << 48;
pub const RESERVED_BITS_MASK: u64 = NAN_MASK ^ TAG_MASK;
// pub const POINTER_TAG_MASK: u64 = 0b111;
pub const SIGN_MASK: u64 = 1 << 63;
pub const RESERVED_BITS_AND_SIGN: u64 = RESERVED_BITS_MASK | SIGN_MASK;

/// Panics (debug only) if `self` is not a valid taggable object. A
/// no-op in release builds.
pub fn assert_is_clean(n: u64) {
    debug_assert!(reserved_bits_clean(n), "0b{:064b}", n);
}

/// `true` for any value which does not store information in the
/// reserved bits. This means that positive values (sign bit of 0)
/// must have all reserved bits set to 0 and negative values (sign bit
/// of 1) must have all reserved bits set to 1.
pub fn reserved_bits_clean(n: u64) -> bool {
    let masked = n & RESERVED_BITS_AND_SIGN;
    masked == 0 || masked == RESERVED_BITS_AND_SIGN
}

/// I *think* this method is equivalent to `f64::is_nan` (except that
/// `f64::is_nan` returns `false` for ±infinity, which should be
/// irrelevant), but I'm not sure enough to switch to using that.
pub const fn is_a_nan(n: u64) -> bool {
    (n & NAN_MASK) == NAN_MASK
}

/// `true` iff `n` is the bit-pattern for one of:
///
/// * positive Infinity
/// * negative Infinity
/// * the NaN used by modern chips
/// * negative the NaN used by modern chips
pub fn is_the_nan_or_ifty(n: u64) -> bool {
    f64::from_bits(n).is_infinite() || ((n & !SIGN_MASK) == core::f64::NAN.to_bits())
}

pub fn is_nanbox(n: u64) -> bool {
    is_a_nan(n) && !is_the_nan_or_ifty(n)
}

pub const fn tag_of(n: u64) -> u8 {
    (((n & TAG_MASK) >> TAG_SHIFT) as u8) & 0x0f
}

/// Clear `RESERVED_BITS_MASK` from `n`, preserving the sign
/// bit. `has_reserved_bits_set` will return `false` for the returned
/// value, but its sign bit depends on the input.
pub fn signed_untag(n: u64) -> i64 {
    if (n as i64) < 0 {
        (n | RESERVED_BITS_MASK) as i64
    } else {
        (n & !RESERVED_BITS_MASK) as i64
    }
}

/// Clear `RESERVED_BITS_MASK` from `n`, clobbering the sign. This
/// will produce a `u64` whose high 16 bits are zero'd.
pub const fn unsigned_untag(n: u64) -> u64 {
    n & !(RESERVED_BITS_MASK | SIGN_MASK)
}

pub const fn nan_tag(n: u64) -> u64 {
    n | NAN_MASK
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn special_case_nans() {
        assert!(!is_nanbox(std::f64::NAN.to_bits()));
        assert!(!is_nanbox(std::f64::INFINITY.to_bits()));
        assert!(!is_nanbox(std::f64::NEG_INFINITY.to_bits()));
        assert!(!is_nanbox((-std::f64::NAN).to_bits()));
    }

    #[test]
    fn simple_nanboxes() {
        let dead_beef = nan_tag(0xdeadbeef);
        assert!(is_nanbox(dead_beef));
        assert_eq!(tag_of(dead_beef), 0);
    }

    #[test]
    fn signed_nanboxes() {
        let signed_int = -12345i64;
        
        let nan_tagged = nan_tag(signed_int as u64);

        assert_eq!(signed_untag(nan_tagged), signed_int);
    }
}
