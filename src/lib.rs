#![cfg_attr(feature = "no_std",  no_std)]

pub mod type_tags;
pub mod bit_utils;
pub mod pointer_tags;

pub use crate::type_tags::{TypeId, ThinTypeId, TypeError, TypeIdTooLargeError};
