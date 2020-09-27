//! A Rust library for converting the primitive and non-zero integer types to a `Result` through comparison.

#![no_std]

/// `use` this trait to access `ok_less`, `ok_equal`, and `ok_greater` for all primitive and non-zero integer types.
pub trait Ext
where
  Self: Sized + Eq + Ord,
{
  /// Returns an `Ok` holding the integer if less than the other integer `val`.
  /// Otherwise, returns and `Err` holding the integer.
  ///
  /// # Examples
  /// ```
  /// use integer_result::Ext;
  ///
  /// assert_eq!(1.ok_less(2), Ok(1));
  /// assert_eq!(1.ok_less(0), Err(1));
  /// ```
  #[inline]
  fn ok_less(self, val: Self) -> Result<Self, Self> {
    if self < val {
      Ok(self)
    } else {
      Err(self)
    }
  }

  /// Returns an `Ok` holding the integer if equal to the other integer `val`.
  /// Otherwise, returns an `Err` holding the integer.
  ///
  /// # Examples
  /// ```
  /// use integer_result::Ext;
  ///
  /// assert_eq!(1.ok_equal(1), Ok(1));
  /// assert_eq!(1.ok_equal(0), Err(1));
  /// ```
  #[inline]
  fn ok_equal(self, val: Self) -> Result<Self, Self> {
    if self == val {
      Ok(self)
    } else {
      Err(self)
    }
  }

  /// Returns an `Ok` holding the integer if greater than the other integer `val`.
  /// Otherwise, returns and `Err` holding the integer.
  ///
  /// # Examples
  /// ```
  /// use integer_result::Ext;
  ///
  /// assert_eq!(1.ok_greater(-1), Ok(1));
  /// assert_eq!(1.ok_greater(1), Err(1));
  /// ```
  #[inline]
  fn ok_greater(self, val: Self) -> Result<Self, Self> {
    if self > val {
      Ok(self)
    } else {
      Err(self)
    }
  }
}

impl Ext for i8 {}
impl Ext for i16 {}
impl Ext for i32 {}
impl Ext for i64 {}
impl Ext for i128 {}
impl Ext for isize {}
impl Ext for u8 {}
impl Ext for u16 {}
impl Ext for u32 {}
impl Ext for u64 {}
impl Ext for u128 {}
impl Ext for usize {}

mod non_zero_impl {
  use core::num::*;
  use crate::Ext;

  impl Ext for NonZeroI8 {}
  impl Ext for NonZeroI16 {}
  impl Ext for NonZeroI32 {}
  impl Ext for NonZeroI64 {}
  impl Ext for NonZeroI128 {}
  impl Ext for NonZeroIsize {}
  impl Ext for NonZeroU8 {}
  impl Ext for NonZeroU16 {}
  impl Ext for NonZeroU32 {}
  impl Ext for NonZeroU64 {}
  impl Ext for NonZeroU128 {}
  impl Ext for NonZeroUsize {}
}

#[cfg(feature="non_zero_impl")]
pub use non_zero_impl::*;

