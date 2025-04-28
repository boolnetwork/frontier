//! This module contains an 256-bit signed integer implementation.
//!
//! This module was derived for ethers-core via <https://github.com/gnosis/ethcontract-rs/>

#![warn(clippy::missing_const_for_fn)]

use sp_core::U256;
use sp_std::{
    cmp,
    fmt::{self, Write},
    iter, ops,
};

/// The error type that is returned when conversion to or from a 256-bit integer fails.
#[derive(Clone, Copy, Debug)]
pub struct TryFromBigIntError;

/// Enum to represent the sign of a 256-bit signed integer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Sign {
    /// Greater than or equal to zero.
    Positive,
    /// Less than zero.
    Negative,
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self, f.sign_plus()) {
            (Self::Positive, false) => Ok(()),
            _ => f.write_char(self.as_char()),
        }
    }
}

impl Sign {
    /// Returns whether the sign is positive.
    #[inline(always)]
    pub const fn is_positive(&self) -> bool {
        matches!(self, Self::Positive)
    }

    /// Returns whether the sign is negative.
    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        matches!(self, Self::Negative)
    }

    /// Returns the sign character.
    #[inline(always)]
    pub const fn as_char(&self) -> char {
        match self {
            Self::Positive => '+',
            Self::Negative => '-',
        }
    }

    /// Computes the `Sign` given a signum.
    #[inline(always)]
    const fn from_signum64(sign: i64) -> Self {
        match sign {
            0 | 1 => Sign::Positive,
            -1 => Sign::Negative,
            _ => unreachable!(),
        }
    }
}

/// Little-endian 256-bit signed integer.
///
/// ## Diversion from standard numeric types
///
/// The right shift operator on I256 doesn't act in the same manner as standard numeric types
/// (e.g. `i8`, `i16` etc). On standard types if the number is negative right shift will perform
/// an arithmetic shift, whereas on I256 this will perform a bit-wise shift.
/// Arithmetic shift on I256 is done via the [asr](I256::asr) and [asl](I256::asl) functions.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct I256(U256);

impl I256 {
    /// Maximum value.
    pub const MAX: Self = Self(U256([u64::MAX, u64::MAX, u64::MAX, i64::MAX as _]));

    /// Minimum value.
    pub const MIN: Self = Self(U256([0, 0, 0, i64::MIN as _]));

    /// Zero (additive identity) of this type.
    #[inline(always)]
    pub const fn zero() -> Self {
        Self(U256::zero())
    }

    /// One (multiplicative identity) of this type.
    #[inline(always)]
    pub const fn one() -> Self {
        Self(U256::one())
    }

    /// Minus one (multiplicative inverse) of this type.
    #[inline(always)]
    pub const fn minus_one() -> Self {
        Self(U256::MAX)
    }

    /// The maximum value which can be inhabited by this type.
    #[inline(always)]
    pub const fn max_value() -> Self {
        Self::MAX
    }

    /// The minimum value which can be inhabited by this type.
    #[inline(always)]
    pub const fn min_value() -> Self {
        Self::MIN
    }

    /// Creates an I256 from a sign and an absolute value. Returns the value and a bool that is true
    /// if the conversion caused an overflow.
    #[inline(always)]
    pub fn overflowing_from_sign_and_abs(sign: Sign, abs: U256) -> (Self, bool) {
        let value = Self(match sign {
            Sign::Positive => abs,
            Sign::Negative => twos_complement(abs),
        });
        (value, value.sign() != sign)
    }

    /// Creates an I256 from an absolute value and a negative flag. Returns `None` if it would
    /// overflow an `I256`.
    #[inline(always)]
    pub fn checked_from_sign_and_abs(sign: Sign, abs: U256) -> Option<Self> {
        let (result, overflow) = Self::overflowing_from_sign_and_abs(sign, abs);
        if overflow {
            None
        } else {
            Some(result)
        }
    }

    /// Splits a I256 into its absolute value and negative flag.
    #[inline(always)]
    pub fn into_sign_and_abs(self) -> (Sign, U256) {
        let sign = self.sign();
        let abs = match sign {
            Sign::Positive => self.0,
            Sign::Negative => twos_complement(self.0),
        };
        (sign, abs)
    }

    /// Returns the sign of self.
    #[inline(always)]
    pub const fn sign(self) -> Sign {
        let most_significant_word = (self.0).0[3];
        match most_significant_word & (1 << 63) {
            0 => Sign::Positive,
            _ => Sign::Negative,
        }
    }

    /// Coerces an unsigned integer into a signed one. If the unsigned integer
    /// is greater than the greater than or equal to `1 << 255`, then the result
    /// will overflow into a negative value.
    #[inline(always)]
    pub const fn from_raw(raw: U256) -> Self {
        Self(raw)
    }

    /// Returns the signed integer as a unsigned integer. If the value of `self` negative, then the
    /// two's complement of its absolute value will be returned.
    #[inline(always)]
    pub const fn into_raw(self) -> U256 {
        self.0
    }

    /// Returns a number representing sign of `self`.
    ///
    /// - `0` if the number is zero
    /// - `1` if the number is positive
    /// - `-1` if the number is negative
    #[inline(always)]
    pub fn signum(self) -> Self {
        self.signum64().into()
    }

    /// Returns an `i64` representing the sign of the number.
    #[inline(always)]
    const fn signum64(self) -> i64 {
        match self.sign() {
            Sign::Positive => (!self.is_zero()) as i64,
            Sign::Negative => -1,
        }
    }

    /// Returns `true` if `self` is positive and `false` if the number is zero
    /// or negative.
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        self.signum64().is_positive()
    }

    /// Returns `true` if `self` is negative and `false` if the number is zero
    /// or positive.
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        self.signum64().is_negative()
    }

    /// Returns `true` if `self` is zero and `false` if the number is negative
    /// or positive.
    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// Return the least number of bits needed to represent the number.
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        let unsigned = self.unsigned_abs();
        let unsigned_bits = unsigned.bits();

        // NOTE: We need to deal with two special cases:
        //   - the number is 0
        //   - the number is a negative power of `2`. These numbers are written as `0b11..1100..00`.
        //   In the case of a negative power of two, the number of bits required
        //   to represent the negative signed value is equal to the number of
        //   bits required to represent its absolute value as an unsigned
        //   integer. This is best illustrated by an example: the number of bits
        //   required to represent `-128` is `8` since it is equal to `i8::MIN`
        //   and, therefore, obviously fits in `8` bits. This is equal to the
        //   number of bits required to represent `128` as an unsigned integer
        //   (which fits in a `u8`).  However, the number of bits required to
        //   represent `128` as a signed integer is `9`, as it is greater than
        //   `i8::MAX`.  In the general case, an extra bit is needed to
        //   represent the sign.
        let bits = if self.count_zeros() == self.trailing_zeros() {
            // `self` is zero or a negative power of two
            unsigned_bits
        } else {
            unsigned_bits + 1
        };

        bits as _
    }

    /// Return if specific bit is set.
    ///
    /// # Panics
    ///
    /// If index exceeds the bit width of the number.
    #[inline(always)]
    #[track_caller]
    pub const fn bit(&self, index: usize) -> bool {
        self.0.bit(index)
    }

    /// Return specific byte.
    ///
    /// # Panics
    ///
    /// If index exceeds the byte width of the number.
    #[inline(always)]
    #[track_caller]
    pub const fn byte(&self, index: usize) -> u8 {
        self.0.byte(index)
    }

    /// Returns the number of ones in the binary representation of `self`.
    #[inline(always)]
    pub fn count_ones(&self) -> u32 {
        (self.0).0.iter().map(|word| word.count_ones()).sum()
    }

    /// Returns the number of zeros in the binary representation of `self`.
    #[inline(always)]
    pub fn count_zeros(&self) -> u32 {
        (self.0).0.iter().map(|word| word.count_zeros()).sum()
    }

    /// Returns the number of leading zeros in the binary representation of
    /// `self`.
    #[inline(always)]
    pub fn leading_zeros(&self) -> u32 {
        self.0.leading_zeros()
    }

    /// Returns the number of leading zeros in the binary representation of
    /// `self`.
    #[inline(always)]
    pub fn trailing_zeros(&self) -> u32 {
        self.0.trailing_zeros()
    }

    /// Write to the slice in big-endian format.
    ///
    /// # Panics
    ///
    /// If the given slice is not exactly 32 bytes long.
    #[inline(always)]
    #[track_caller]
    pub fn to_big_endian(&self, bytes: &mut [u8]) {
        self.0.to_big_endian(bytes)
    }

    /// Write to the slice in little-endian format.
    ///
    /// # Panics
    ///
    /// If the given slice is not exactly 32 bytes long.
    #[inline(always)]
    #[track_caller]
    pub fn to_little_endian(&self, bytes: &mut [u8]) {
        self.0.to_little_endian(bytes)
    }
}

// ops impl
impl I256 {
    /// Computes the absolute value of `self`.
    ///
    /// # Overflow behavior
    ///
    /// The absolute value of `I256::MIN` cannot be represented as an `I256` and attempting to
    /// calculate it will cause an overflow. This means that code in debug mode will trigger a panic
    /// on this case and optimized code will return `I256::MIN` without a panic.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn abs(self) -> Self {
        handle_overflow(self.overflowing_abs())
    }

    /// Computes the absolute value of `self`.
    ///
    /// Returns a tuple of the absolute version of self along with a boolean indicating whether an
    /// overflow happened. If self is the minimum value then the minimum value will be returned
    /// again and true will be returned for an overflow happening.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_abs(self) -> (Self, bool) {
        if self == Self::MIN {
            (self, true)
        } else {
            (Self(self.unsigned_abs()), false)
        }
    }

    /// Checked absolute value. Computes `self.abs()`, returning `None` if `self == MIN`.
    #[inline(always)]
    #[must_use]
    pub fn checked_abs(self) -> Option<Self> {
        match self.overflowing_abs() {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating absolute value. Computes `self.abs()`, returning `MAX` if `self == MIN` instead
    /// of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_abs(self) -> Self {
        match self.overflowing_abs() {
            (value, false) => value,
            _ => Self::MAX,
        }
    }

    /// Wrapping absolute value. Computes `self.abs()`, wrapping around at the boundary of the type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_abs(self) -> Self {
        self.overflowing_abs().0
    }

    /// Computes the absolute value of `self` without any wrapping or panicking.
    #[inline(always)]
    #[must_use]
    pub fn unsigned_abs(self) -> U256 {
        self.into_sign_and_abs().1
    }

    /// Negates self, overflowing if this is equal to the minimum value.
    ///
    /// Returns a tuple of the negated version of self along with a boolean indicating whether an
    /// overflow happened. If `self` is the minimum value, then the minimum value will be returned
    /// again and `true` will be returned for an overflow happening.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_neg(self) -> (Self, bool) {
        if self == Self::MIN {
            (self, true)
        } else {
            (Self(twos_complement(self.0)), false)
        }
    }

    /// Checked negation. Computes `-self`, returning `None` if `self == MIN`.
    #[inline(always)]
    #[must_use]
    pub fn checked_neg(self) -> Option<Self> {
        match self.overflowing_neg() {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating negation. Computes `-self`, returning `MAX` if `self == MIN` instead of
    /// overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_neg(self) -> Self {
        match self.overflowing_neg() {
            (value, false) => value,
            _ => Self::MAX,
        }
    }

    /// Wrapping (modular) negation. Computes `-self`, wrapping around at the boundary of the type.
    ///
    /// The only case where such wrapping can occur is when one negates `MIN` on a signed type
    /// (where `MIN` is the negative minimal value for the type); this is a positive value that is
    /// too large to represent in the type. In such a case, this function returns `MIN` itself.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_neg(self) -> Self {
        self.overflowing_neg().0
    }

    /// Calculates `self` + `rhs`
    ///
    /// Returns a tuple of the addition along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (unsigned, _) = self.0.overflowing_add(rhs.0);
        let result = Self(unsigned);

        // NOTE: Overflow is determined by checking the sign of the operands and
        //   the result.
        let overflow = matches!(
            (self.sign(), rhs.sign(), result.sign()),
            (Sign::Positive, Sign::Positive, Sign::Negative) |
                (Sign::Negative, Sign::Negative, Sign::Positive)
        );

        (result, overflow)
    }

    /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.overflowing_add(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating integer addition. Computes `self + rhs`, saturating at the numeric bounds instead
    /// of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_add(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_add(rhs);
        if overflow {
            match result.sign() {
                Sign::Positive => Self::MIN,
                Sign::Negative => Self::MAX,
            }
        } else {
            result
        }
    }

    /// Wrapping (modular) addition. Computes `self + rhs`, wrapping around at the boundary of the
    /// type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_add(self, rhs: Self) -> Self {
        self.overflowing_add(rhs).0
    }

    /// Calculates `self` - `rhs`
    ///
    /// Returns a tuple of the subtraction along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        // NOTE: We can't just compute the `self + (-rhs)` because `-rhs` does
        //   not always exist, specifically this would be a problem in case
        //   `rhs == Self::MIN`

        let (unsigned, _) = self.0.overflowing_sub(rhs.0);
        let result = Self(unsigned);

        // NOTE: Overflow is determined by checking the sign of the operands and
        //   the result.
        let overflow = matches!(
            (self.sign(), rhs.sign(), result.sign()),
            (Sign::Positive, Sign::Negative, Sign::Negative) |
                (Sign::Negative, Sign::Positive, Sign::Positive)
        );

        (result, overflow)
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.overflowing_sub(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating integer subtraction. Computes `self - rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_sub(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_sub(rhs);
        if overflow {
            match result.sign() {
                Sign::Positive => Self::MIN,
                Sign::Negative => Self::MAX,
            }
        } else {
            result
        }
    }

    /// Wrapping (modular) subtraction. Computes `self - rhs`, wrapping around at the boundary of
    /// the type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        self.overflowing_sub(rhs).0
    }

    /// Calculates `self` * `rhs`
    ///
    /// Returns a tuple of the multiplication along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would have occurred then the wrapped value is returned.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let sign = Sign::from_signum64(self.signum64() * rhs.signum64());
        let (unsigned, overflow_mul) = self.unsigned_abs().overflowing_mul(rhs.unsigned_abs());
        let (result, overflow_conv) = Self::overflowing_from_sign_and_abs(sign, unsigned);

        (result, overflow_mul || overflow_conv)
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning None if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        match self.overflowing_mul(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Saturating integer multiplication. Computes `self * rhs`, saturating at the numeric bounds
    /// instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_mul(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_mul(rhs);
        if overflow {
            match Sign::from_signum64(self.signum64() * rhs.signum64()) {
                Sign::Positive => Self::MAX,
                Sign::Negative => Self::MIN,
            }
        } else {
            result
        }
    }

    /// Wrapping (modular) multiplication. Computes `self * rhs`, wrapping around at the boundary of
    /// the type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }

    /// Calculates `self` / `rhs`
    ///
    /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would occur then self is returned.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        // Panic early when with division by zero while evaluating sign.
        let sign = Sign::from_signum64(self.signum64() / rhs.signum64());
        // Note, signed division can't overflow!
        let unsigned = self.unsigned_abs() / rhs.unsigned_abs();
        let (result, overflow_conv) = Self::overflowing_from_sign_and_abs(sign, unsigned);

        (result, overflow_conv && !result.is_zero())
    }

    /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs == 0` or the
    /// division results in overflow.
    #[inline(always)]
    #[must_use]
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::min_value() && rhs == Self::minus_one()) {
            None
        } else {
            Some(self.overflowing_div(rhs).0)
        }
    }

    /// Saturating integer division. Computes `self / rhs`, saturating at the numeric bounds instead
    /// of overflowing.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn saturating_div(self, rhs: Self) -> Self {
        match self.overflowing_div(rhs) {
            (value, false) => value,
            // MIN / -1 is the only possible saturating overflow
            _ => Self::MAX,
        }
    }

    /// Wrapping (modular) division. Computes `self / rhs`, wrapping around at the boundary of the
    /// type.
    ///
    /// The only case where such wrapping can occur is when one divides `MIN / -1` on a signed type
    /// (where `MIN` is the negative minimal value for the type); this is equivalent to `-MIN`, a
    /// positive value that is too large to represent in the type. In such a case, this function
    /// returns `MIN` itself.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn wrapping_div(self, rhs: Self) -> Self {
        self.overflowing_div(rhs).0
    }

    /// Calculates `self` % `rhs`
    ///
    /// Returns a tuple of the remainder after dividing along with a boolean indicating whether an
    /// arithmetic overflow would occur. If an overflow would occur then 0 is returned.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
        if self == Self::MIN && rhs == Self::minus_one() {
            (Self::zero(), true)
        } else {
            let div_res = self / rhs;
            (self - div_res * rhs, false)
        }
    }

    /// Checked integer remainder. Computes `self % rhs`, returning `None` if `rhs == 0` or the
    /// division results in overflow.
    #[inline(always)]
    #[must_use]
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::MIN && rhs == Self::minus_one()) {
            None
        } else {
            Some(self.overflowing_rem(rhs).0)
        }
    }

    /// Wrapping (modular) remainder. Computes `self % rhs`, wrapping around at the boundary of the
    /// type.
    ///
    /// Such wrap-around never actually occurs mathematically; implementation artifacts make `x % y`
    /// invalid for `MIN / -1` on a signed type (where `MIN` is the negative minimal value). In such
    /// a case, this function returns `0`.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn wrapping_rem(self, rhs: Self) -> Self {
        self.overflowing_rem(rhs).0
    }

    /// Calculates the quotient of Euclidean division of `self` by `rhs`.
    ///
    /// This computes the integer `q` such that `self = q * rhs + r`, with
    /// `r = self.rem_euclid(rhs)` and `0 <= r < abs(rhs)`.
    ///
    /// In other words, the result is `self / rhs` rounded to the integer `q` such that `self >= q *
    /// rhs`.
    /// If `self > 0`, this is equal to round towards zero (the default in Rust);
    /// if `self < 0`, this is equal to round towards +/- infinity.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0 or the division results in overflow.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn div_euclid(self, rhs: Self) -> Self {
        let q = self / rhs;
        if (self % rhs).is_negative() {
            if rhs.is_positive() {
                q - Self::one()
            } else {
                q + Self::one()
            }
        } else {
            q
        }
    }

    /// Calculates the quotient of Euclidean division `self.div_euclid(rhs)`.
    ///
    /// Returns a tuple of the divisor along with a boolean indicating whether an arithmetic
    /// overflow would occur. If an overflow would occur then `self` is returned.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
        if self == Self::min_value() && rhs == Self::minus_one() {
            (self, true)
        } else {
            (self.div_euclid(rhs), false)
        }
    }

    /// Checked Euclidean division. Computes `self.div_euclid(rhs)`, returning `None` if `rhs == 0`
    /// or the division results in overflow.
    #[inline(always)]
    #[must_use]
    pub fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::min_value() && rhs == Self::minus_one()) {
            None
        } else {
            Some(self.div_euclid(rhs))
        }
    }

    /// Wrapping Euclidean division. Computes `self.div_euclid(rhs)`,
    /// wrapping around at the boundary of the type.
    ///
    /// Wrapping will only occur in `MIN / -1` on a signed type (where `MIN` is the negative minimal
    /// value for the type). This is equivalent to `-MIN`, a positive value that is too large to
    /// represent in the type. In this case, this method returns `MIN` itself.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn wrapping_div_euclid(self, rhs: Self) -> Self {
        self.overflowing_div_euclid(rhs).0
    }

    /// Calculates the least nonnegative remainder of `self (mod rhs)`.
    ///
    /// This is done as if by the Euclidean division algorithm -- given `r = self.rem_euclid(rhs)`,
    /// `self = rhs * self.div_euclid(rhs) + r`, and `0 <= r < abs(rhs)`.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0 or the division results in overflow.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        let r = self % rhs;
        if r < Self::zero() {
            if rhs < Self::zero() {
                r - rhs
            } else {
                r + rhs
            }
        } else {
            r
        }
    }

    /// Overflowing Euclidean remainder. Calculates `self.rem_euclid(rhs)`.
    ///
    /// Returns a tuple of the remainder after dividing along with a boolean indicating whether an
    /// arithmetic overflow would occur. If an overflow would occur then 0 is returned.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
        if self == Self::min_value() && rhs == Self::minus_one() {
            (Self::zero(), true)
        } else {
            (self.rem_euclid(rhs), false)
        }
    }

    /// Wrapping Euclidean remainder. Computes `self.rem_euclid(rhs)`, wrapping around at the
    /// boundary of the type.
    ///
    /// Wrapping will only occur in `MIN % -1` on a signed type (where `MIN` is the negative minimal
    /// value for the type). In this case, this method returns 0.
    ///
    /// # Panics
    ///
    /// If `rhs` is 0.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn wrapping_rem_euclid(self, rhs: Self) -> Self {
        self.overflowing_rem_euclid(rhs).0
    }

    /// Checked Euclidean remainder. Computes `self.rem_euclid(rhs)`, returning `None` if `rhs == 0`
    /// or the division results in overflow.
    #[inline(always)]
    #[must_use]
    pub fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
        if rhs.is_zero() || (self == Self::min_value() && rhs == Self::minus_one()) {
            None
        } else {
            Some(self.rem_euclid(rhs))
        }
    }

    /// Returns the sign of `self` to the exponent `exp`.
    ///
    /// Note that this method does not actually try to compute the `self` to the
    /// exponent `exp`, but instead uses the property that a negative number to
    /// an odd exponent will be negative. This means that the sign of the result
    /// of exponentiation can be computed even if the actual result is too large
    /// to fit in 256-bit signed integer.
    #[inline(always)]
    const fn pow_sign(self, exp: u32) -> Sign {
        let is_exp_odd = exp % 2 != 0;
        if is_exp_odd && self.is_negative() {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    /// Create `10**n` as this type.
    ///
    /// # Panics
    ///
    /// If the result overflows the type.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn exp10(n: usize) -> Self {
        U256::exp10(n).try_into().expect("overflow")
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// # Panics
    ///
    /// If the result overflows the type in debug mode.
    #[inline(always)]
    #[track_caller]
    #[must_use]
    pub fn pow(self, exp: u32) -> Self {
        handle_overflow(self.overflowing_pow(exp))
    }

    /// Raises self to the power of `exp`, using exponentiation by squaring.
    ///
    /// Returns a tuple of the exponentiation along with a bool indicating whether an overflow
    /// happened.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_pow(self, exp: u32) -> (Self, bool) {
        let sign = self.pow_sign(exp);
        let (unsigned, overflow_pow) = self.unsigned_abs().overflowing_pow(exp.into());
        let (result, overflow_conv) = Self::overflowing_from_sign_and_abs(sign, unsigned);

        (result, overflow_pow || overflow_conv)
    }

    /// Checked exponentiation. Computes `self.pow(exp)`, returning `None` if overflow occurred.
    #[inline(always)]
    #[must_use]
    pub fn checked_pow(self, exp: u32) -> Option<Self> {
        let (result, overflow) = self.overflowing_pow(exp);
        if overflow {
            None
        } else {
            Some(result)
        }
    }

    /// Saturating integer exponentiation. Computes `self.pow(exp)`, saturating at the numeric
    /// bounds instead of overflowing.
    #[inline(always)]
    #[must_use]
    pub fn saturating_pow(self, exp: u32) -> Self {
        let (result, overflow) = self.overflowing_pow(exp);
        if overflow {
            match self.pow_sign(exp) {
                Sign::Positive => Self::MAX,
                Sign::Negative => Self::MIN,
            }
        } else {
            result
        }
    }

    /// Raises self to the power of `exp`, wrapping around at the
    /// boundary of the type.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_pow(self, exp: u32) -> Self {
        self.overflowing_pow(exp).0
    }

    /// Shifts self left by `rhs` bits.
    ///
    /// Returns a tuple of the shifted version of self along with a boolean indicating whether the
    /// shift value was larger than or equal to the number of bits.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_shl(self, rhs: usize) -> (Self, bool) {
        if rhs >= 256 {
            (Self::zero(), true)
        } else {
            (Self(self.0 << rhs), false)
        }
    }

    /// Checked shift left. Computes `self << rhs`, returning `None` if `rhs` is larger than or
    /// equal to the number of bits in `self`.
    #[inline(always)]
    #[must_use]
    pub fn checked_shl(self, rhs: usize) -> Option<Self> {
        match self.overflowing_shl(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Wrapping shift left. Computes `self << rhs`, returning 0 if larger than or equal to the
    /// number of bits in `self`.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_shl(self, rhs: usize) -> Self {
        self.overflowing_shl(rhs).0
    }

    /// Shifts self right by `rhs` bits.
    ///
    /// Returns a tuple of the shifted version of self along with a boolean indicating whether the
    /// shift value was larger than or equal to the number of bits.
    #[inline(always)]
    #[must_use]
    pub fn overflowing_shr(self, rhs: usize) -> (Self, bool) {
        if rhs >= 256 {
            (Self::zero(), true)
        } else {
            (Self(self.0 >> rhs), false)
        }
    }

    /// Checked shift right. Computes `self >> rhs`, returning `None` if `rhs` is larger than or
    /// equal to the number of bits in `self`.
    #[inline(always)]
    #[must_use]
    pub fn checked_shr(self, rhs: usize) -> Option<Self> {
        match self.overflowing_shr(rhs) {
            (value, false) => Some(value),
            _ => None,
        }
    }

    /// Wrapping shift right. Computes `self >> rhs`, returning 0 if larger than or equal to the
    /// number of bits in `self`.
    #[inline(always)]
    #[must_use]
    pub fn wrapping_shr(self, rhs: usize) -> Self {
        self.overflowing_shr(rhs).0
    }

    /// Arithmetic shift right operation. Computes `self >> rhs` maintaining the original sign. If
    /// the number is positive this is the same as logic shift right.
    #[inline(always)]
    #[must_use]
    pub fn asr(self, rhs: usize) -> Self {
        // Avoid shifting if we are going to know the result regardless of the value.
        match (rhs, self.sign()) {
            (0, _) => self,

            // Perform the shift.
            (1..=254, Sign::Positive) => self.wrapping_shr(rhs),
            (1..=254, Sign::Negative) => {
                // We need to do: `for 0..shift { self >> 1 | 2^255 }`
                // We can avoid the loop by doing: `self >> shift | ~(2^(255 - shift) - 1)`
                // where '~' represents ones complement
                const TWO: U256 = U256([2, 0, 0, 0]);
                let bitwise_or = Self::from_raw(!(TWO.pow(U256::from(255 - rhs)) - U256::one()));
                (self.wrapping_shr(rhs)) | bitwise_or
            }

            // It's always going to be zero (i.e. 00000000...00000000)
            (255.., Sign::Positive) => Self::zero(),
            // It's always going to be -1 (i.e. 11111111...11111111)
            (255.., Sign::Negative) => Self::minus_one(),

            // Rust cannot prove that the above is exhaustive for usize (works for any other int)
            _ => unreachable!(),
        }
    }

    /// Arithmetic shift left operation. Computes `self << rhs`, checking for overflow on the final
    /// result.
    ///
    /// Returns `None` if the operation overflowed (most significant bit changes).
    #[inline(always)]
    #[must_use]
    pub fn asl(self, rhs: usize) -> Option<Self> {
        if rhs == 0 {
            Some(self)
        } else {
            let result = self.wrapping_shl(rhs);
            if result.sign() != self.sign() {
                // Overflow occurred
                None
            } else {
                Some(result)
            }
        }
    }

    /// Compute the [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) of this number.
    #[inline(always)]
    #[must_use]
    pub fn twos_complement(self) -> U256 {
        let abs = self.into_raw();
        match self.sign() {
            Sign::Positive => abs,
            Sign::Negative => twos_complement(abs),
        }
    }
}

// conversions
macro_rules! impl_conversions {
    ($(
        $u:ty [$actual_low_u:ident -> $low_u:ident, $as_u:ident],
        $i:ty [$actual_low_i:ident -> $low_i:ident, $as_i:ident];
    )+) => {
        // low_*, as_*
        impl I256 {
            $(
                impl_conversions!(@impl_fns $u, $actual_low_u $low_u $as_u);
                impl_conversions!(@impl_fns $i, $actual_low_i $low_i $as_i);
            )+
        }

        // From<$>, TryFrom
        $(
            impl From<$u> for I256 {
                #[inline(always)]
                fn from(value: $u) -> Self {
                    Self(<U256 as From<$u>>::from(value))
                }
            }

            impl From<$i> for I256 {
                #[inline(always)]
                fn from(value: $i) -> Self {
                    let uint: $u = value as $u;
                    Self(if value.is_negative() {
                        let abs = (!uint).wrapping_add(1);
                        twos_complement(<U256 as From<$u>>::from(abs))
                    } else {
                        <U256 as From<$u>>::from(uint)
                    })
                }
            }

            impl TryFrom<I256> for $u {
                type Error = TryFromBigIntError;

                #[inline(always)]
                fn try_from(value: I256) -> Result<$u, Self::Error> {
                    if value.is_negative() || value > I256::from(<$u>::MAX) {
                        return Err(TryFromBigIntError);
                    }

                    Ok(value.$actual_low_u() as $u)
                }
            }

            impl TryFrom<I256> for $i {
                type Error = TryFromBigIntError;

                #[inline(always)]
                fn try_from(value: I256) -> Result<$i, Self::Error> {
                    if value < I256::from(<$i>::MIN) || value > I256::from(<$i>::MAX) {
                        return Err(TryFromBigIntError);
                    }

                    Ok(value.$actual_low_i() as $i)
                }
            }
        )+
    };

    (@impl_fns $t:ty, $actual_low:ident $low:ident $as:ident) => {
        /// Low word.
        #[inline(always)]
        pub const fn $low(&self) -> $t {
            self.0.$actual_low() as $t
        }

        #[doc = concat!("Conversion to ", stringify!($t) ," with overflow checking.")]
        ///
        /// # Panics
        ///
        #[doc = concat!("If the number is outside the ", stringify!($t), " valid range.")]
        #[inline(always)]
        #[track_caller]
        pub fn $as(&self) -> $t {
            <$t as TryFrom<Self>>::try_from(*self).unwrap()
        }
    };
}

// Use `U256::low_u64` for types which fit in one word.
impl_conversions! {
    u8   [low_u64  -> low_u8,    as_u8],    i8   [low_u64  -> low_i8,    as_i8];
    u16  [low_u64  -> low_u16,   as_u16],   i16  [low_u64  -> low_i16,   as_i16];
    u32  [low_u64  -> low_u32,   as_u32],   i32  [low_u64  -> low_i32,   as_i32];
    u64  [low_u64  -> low_u64,   as_u64],   i64  [low_u64  -> low_i64,   as_i64];
    usize[low_u64  -> low_usize, as_usize], isize[low_u64  -> low_isize, as_isize];
    u128 [low_u128 -> low_u128,  as_u128],  i128 [low_u128 -> low_i128,  as_i128];
}

impl TryFrom<U256> for I256 {
    type Error = TryFromBigIntError;

    #[inline(always)]
    fn try_from(from: U256) -> Result<Self, Self::Error> {
        let value = I256(from);
        match value.sign() {
            Sign::Positive => Ok(value),
            Sign::Negative => Err(TryFromBigIntError),
        }
    }
}

impl TryFrom<I256> for U256 {
    type Error = TryFromBigIntError;

    #[inline(always)]
    fn try_from(value: I256) -> Result<Self, Self::Error> {
        match value.sign() {
            Sign::Positive => Ok(value.0),
            Sign::Negative => Err(TryFromBigIntError),
        }
    }
}

// formatting
impl fmt::Debug for I256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for I256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (sign, abs) = self.into_sign_and_abs();
        fmt::Display::fmt(&sign, f)?;
        write!(f, "{abs}")
    }
}

impl fmt::LowerHex for I256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (sign, abs) = self.into_sign_and_abs();
        fmt::Display::fmt(&sign, f)?;
        write!(f, "{abs:x}")
    }
}

impl fmt::UpperHex for I256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (sign, abs) = self.into_sign_and_abs();
        fmt::Display::fmt(&sign, f)?;

        // NOTE: Work around `U256: !UpperHex`.
        let mut buffer = format!("{abs:x}");
        buffer.make_ascii_uppercase();
        f.write_str(&buffer)
    }
}

// cmp
impl cmp::PartialOrd for I256 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for I256 {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // TODO(nlordell): Once subtraction is implemented:
        // self.saturating_sub(*other).signum64().partial_cmp(&0)

        use cmp::Ordering::*;
        use Sign::*;

        match (self.into_sign_and_abs(), other.into_sign_and_abs()) {
            ((Positive, _), (Negative, _)) => Greater,
            ((Negative, _), (Positive, _)) => Less,
            ((Positive, this), (Positive, other)) => this.cmp(&other),
            ((Negative, this), (Negative, other)) => other.cmp(&this),
        }
    }
}

// arithmetic ops - implemented above
impl<T: Into<I256>> ops::Add<T> for I256 {
    type Output = Self;

    #[track_caller]
    fn add(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_add(rhs.into()))
    }
}

impl<T: Into<I256>> ops::AddAssign<T> for I256 {
    #[track_caller]
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs
    }
}

impl<T: Into<I256>> ops::Sub<T> for I256 {
    type Output = Self;

    #[track_caller]
    fn sub(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_sub(rhs.into()))
    }
}

impl<T: Into<I256>> ops::SubAssign<T> for I256 {
    #[track_caller]
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T: Into<I256>> ops::Mul<T> for I256 {
    type Output = Self;

    #[track_caller]
    fn mul(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_mul(rhs.into()))
    }
}

impl<T: Into<I256>> ops::MulAssign<T> for I256 {
    #[track_caller]
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T: Into<I256>> ops::Div<T> for I256 {
    type Output = Self;

    #[track_caller]
    fn div(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_div(rhs.into()))
    }
}

impl<T: Into<I256>> ops::DivAssign<T> for I256 {
    #[track_caller]
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T: Into<I256>> ops::Rem<T> for I256 {
    type Output = Self;

    #[track_caller]
    fn rem(self, rhs: T) -> Self::Output {
        handle_overflow(self.overflowing_rem(rhs.into()))
    }
}

impl<T: Into<I256>> ops::RemAssign<T> for I256 {
    #[track_caller]
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs;
    }
}

impl<T: Into<I256>> iter::Sum<T> for I256 {
    #[track_caller]
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(I256::zero(), |acc, x| acc + x)
    }
}

impl<T: Into<I256>> iter::Product<T> for I256 {
    #[track_caller]
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(I256::one(), |acc, x| acc * x)
    }
}

// bitwise ops - delegated to U256
impl ops::BitAnd for I256 {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        I256(self.0 & rhs.0)
    }
}

impl ops::BitAndAssign for I256 {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl ops::BitOr for I256 {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        I256(self.0 | rhs.0)
    }
}

impl ops::BitOrAssign for I256 {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl ops::BitXor for I256 {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        I256(self.0 ^ rhs.0)
    }
}

impl ops::BitXorAssign for I256 {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

// Implement Shl and Shr only for types <= usize, since U256 uses .as_usize() which panics
macro_rules! impl_shift {
    ($($t:ty),+) => {
        // We are OK with wrapping behaviour here because it's how Rust behaves with the primitive
        // integer types.

        // $t <= usize: cast to usize
        $(
            impl ops::Shl<$t> for I256 {
                type Output = Self;

                #[inline(always)]
                fn shl(self, rhs: $t) -> Self::Output {
                    self.wrapping_shl(rhs as usize)
                }
            }

            impl ops::ShlAssign<$t> for I256 {
                #[inline(always)]
                fn shl_assign(&mut self, rhs: $t) {
                    *self = *self << rhs;
                }
            }

            impl ops::Shr<$t> for I256 {
                type Output = Self;

                #[inline(always)]
                fn shr(self, rhs: $t) -> Self::Output {
                    self.wrapping_shr(rhs as usize)
                }
            }

            impl ops::ShrAssign<$t> for I256 {
                #[inline(always)]
                fn shr_assign(&mut self, rhs: $t) {
                    *self = *self >> rhs;
                }
            }
        )+
    };
}

#[cfg(target_pointer_width = "16")]
impl_shift!(i8, u8, i16, u16, isize, usize);

#[cfg(target_pointer_width = "32")]
impl_shift!(i8, u8, i16, u16, i32, u32, isize, usize);

#[cfg(target_pointer_width = "64")]
impl_shift!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

// unary ops
impl ops::Neg for I256 {
    type Output = I256;

    #[inline(always)]
    #[track_caller]
    fn neg(self) -> Self::Output {
        handle_overflow(self.overflowing_neg())
    }
}

impl ops::Not for I256 {
    type Output = I256;

    #[inline(always)]
    fn not(self) -> Self::Output {
        I256(!self.0)
    }
}

/// Compute the two's complement of a U256.
#[inline(always)]
fn twos_complement(u: U256) -> U256 {
    (!u).overflowing_add(U256::one()).0
}

/// Panic if overflow on debug mode.
#[inline(always)]
#[track_caller]
fn handle_overflow((result, overflow): (I256, bool)) -> I256 {
    debug_assert!(!overflow, "overflow");
    result
}
