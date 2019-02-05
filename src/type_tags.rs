use crate::bit_utils;
use core::convert::TryFrom;
use failure::Fail;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
/// A type tag. When stored in this form, it must have its high 16
/// bits clean - that is, `bit_utils::has_reserved_bits(self.0)` must
/// return `false`.
pub struct TypeId(u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
/// A type tag used in single-word tagging for pass-by-value types
/// such as pointers.
///
/// Though it is stored as a `u8`, this will always fit in the low
/// four bits.
pub struct ThinTypeId(u8);

#[derive(Copy, Clone, Debug, Fail)]
#[repr(transparent)]
#[fail(display = "The TypeId {:?} does not fit into a ThinTypeId", id)]
/// The error produced by `ThinTypeId as convert::TryFrom<TypeId>`;
/// denotes the attempt to convert a `TypeId` which does not fit into
/// 4 bits.
pub struct TypeIdTooLargeError {
    id: TypeId,
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "Expected a value of TypeId {:?} but found one of TypeId {:?}", expected, found)]
pub struct TypeError {
    expected: TypeId,
    found: TypeId,
}

impl TryFrom<TypeId> for ThinTypeId {
    type Error = TypeIdTooLargeError;
    /// A `TypeId` can become a `ThinTypeId` iff it fits in 4 bits.
    fn try_from(id: TypeId) -> Result<ThinTypeId, Self::Error> {
        if id.0 <= 0xf {
            Ok(ThinTypeId(id.0 as _))
        } else {
            Err(TypeIdTooLargeError { id })
        }
    }
}

impl From<ThinTypeId> for TypeId {
    fn from(ThinTypeId(id): ThinTypeId) -> TypeId {
        TypeId(id.into())
    }
}

impl ThinTypeId {
    /// Panics (debug only) if `self` is malformed. A no-op in release
    /// builds.
    fn assert_size(self) {
        debug_assert!((self.0 & 0xf0) == 0);
    }
    /// Produces a `u64` suitable for tagging a `Word`.
    fn shift_for_tagging(self) -> u64 {
        self.assert_size();

        u64::from(self.0) << bit_utils::TAG_SHIFT
    }
    pub fn matches(self, n: u64) -> bool {
        bit_utils::is_nanbox(n) && (bit_utils::tag_of(n) == self.0)
    }
    /// Given a 48 bit + sign u64, designate it as a NaN-box and mark
    /// it as being of this type.
    pub fn tag(self, to_tag: u64) -> u64 {
        bit_utils::assert_is_clean(to_tag);

        (bit_utils::nan_tag(to_tag) & !bit_utils::TAG_MASK) | self.shift_for_tagging()
    }

    /// If `to_untag` is a nanbox whose type mask matches `self`,
    /// untag and return the wrapped value. Otherwise, error.
    pub fn try_unsigned_untag(self, to_untag: u64) -> Result<u64, TypeError> {
        if self.matches(to_untag) {
            Ok(bit_utils::unsigned_untag(to_untag))
        } else {
            Err(TypeError {
                expected: self.into(),
                found: ThinTypeId(bit_utils::tag_of(to_untag)).into(),
            })
        }
    }

    pub fn try_signed_untag(self, to_untag: u64) -> Result<i64, TypeError> {
        if self.matches(to_untag) {
            Ok(bit_utils::signed_untag(to_untag))
        } else {
            Err(TypeError {
                expected: self.into(),
                found: ThinTypeId(bit_utils::tag_of(to_untag)).into(),
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn fail_thintypeid_conv() {
        use std::convert::TryInto;
        let thick_type_id = TypeId(0xffff);
        assert!(TryInto::<ThinTypeId>::try_into(thick_type_id).is_err());
    }
    #[test]
    fn thintypeid_conv() {
        use std::convert::TryInto;
        let type_id = TypeId(0x2);
        assert_eq!(
            TryInto::<ThinTypeId>::try_into(type_id).unwrap(),
            ThinTypeId(0x2)
        );
    }
    #[test]
    fn tagged_nan() {
        use crate::bit_utils;

        let type_id = ThinTypeId(0xa);

        let tagged_nan = type_id.tag(12345);

        assert_eq!(bit_utils::tag_of(tagged_nan), 0xa);
        assert_eq!(bit_utils::unsigned_untag(tagged_nan), 12345);
    }

    #[test]
    fn try_untag() {
        let type_id = ThinTypeId(0xa);
        let tagged_nan = type_id.tag(12345);

        assert_eq!(type_id.try_unsigned_untag(tagged_nan).unwrap(), 12345);

        let wrong_type_id = ThinTypeId(0xc);

        assert!(wrong_type_id.try_unsigned_untag(tagged_nan).is_err());
    }

    #[test]
    fn signed_number() {
        let signed = -666i64;
        let type_id = ThinTypeId(0x2);
        let boxed = type_id.tag(signed as u64);
        assert_eq!(type_id.try_signed_untag(boxed).unwrap(), signed);
    }
}
