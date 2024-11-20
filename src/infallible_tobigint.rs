use num::{bigint::ToBigInt, BigInt};

/// Allows for type conversion to [`num::BigInt`] without worrying about Results.
///
/// The types `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128` and `isize` are safely
/// convertible to `BigInt`. Only `f32` and `f64` cannot be converted if they are not whole numbers. This traits limits
/// its implementation to those safe types mentioned. So no error checking is needed.
///
/// # Example
/// ```
/// use crate::infallible_to_big_int::*;
///
/// // use the conversion directly
/// 153830.to_bigint();
///
/// // or define a function which takes any InfallibleToBigInt
/// fn do_great_things(to_bigint: impl InfallibleToBigInt) {
///     let bigint = to_bigint.to_bigint();
///     // ... do something nice with bigint here
/// }
///
/// // then you can call it like this
/// do_great_things(153830)
/// ```
pub trait InfallibleToBigInt {
    fn to_bigint(&self) -> BigInt;
}

impl InfallibleToBigInt for i8 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for i8, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for i16 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for i16, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for i32 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for i32, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for i64 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for i64, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for i128 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for i128, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for isize {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for isize, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for u8 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for u8, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for u16 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for u16, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for u32 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for u32, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for u64 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for u64, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for u128 {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for u128, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigInt for usize {
    fn to_bigint(&self) -> BigInt {
        ToBigInt::to_bigint(self)
            .expect("to_bigint failed for usize, this should not happen and is most likely a programming error")
    }
}

#[cfg(test)]
mod tests {
    use num::bigint::ToBigInt;

    use super::InfallibleToBigInt;

    /// Test MIN and MAX values of u8
    #[test]
    fn test_u8() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u8::MIN),
            ToBigInt::to_bigint(&u8::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u8::MAX),
            ToBigInt::to_bigint(&u8::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of u16
    #[test]
    fn test_u16() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u16::MIN),
            ToBigInt::to_bigint(&u16::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u16::MAX),
            ToBigInt::to_bigint(&u16::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of u32
    #[test]
    fn test_u32() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u32::MIN),
            ToBigInt::to_bigint(&u32::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u32::MAX),
            ToBigInt::to_bigint(&u32::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of u64
    #[test]
    fn test_u64() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u64::MIN),
            ToBigInt::to_bigint(&u64::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u64::MAX),
            ToBigInt::to_bigint(&u64::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of u128
    #[test]
    fn test_u128() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u128::MIN),
            ToBigInt::to_bigint(&u128::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&u128::MAX),
            ToBigInt::to_bigint(&u128::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of usize
    #[test]
    fn test_usize() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&usize::MIN),
            ToBigInt::to_bigint(&usize::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&usize::MAX),
            ToBigInt::to_bigint(&usize::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of i8
    #[test]
    fn test_i8() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i8::MIN),
            ToBigInt::to_bigint(&i8::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i8::MAX),
            ToBigInt::to_bigint(&i8::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of i16
    #[test]
    fn test_i16() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i16::MIN),
            ToBigInt::to_bigint(&i16::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i16::MAX),
            ToBigInt::to_bigint(&i16::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of i32
    #[test]
    fn test_i32() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i32::MIN),
            ToBigInt::to_bigint(&i32::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i32::MAX),
            ToBigInt::to_bigint(&i32::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of i64
    #[test]
    fn test_i64() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i64::MIN),
            ToBigInt::to_bigint(&i64::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i64::MAX),
            ToBigInt::to_bigint(&i64::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of i128
    #[test]
    fn test_i128() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i128::MIN),
            ToBigInt::to_bigint(&i128::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&i128::MAX),
            ToBigInt::to_bigint(&i128::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of isize
    #[test]
    fn test_isize() {
        assert_eq!(
            InfallibleToBigInt::to_bigint(&isize::MIN),
            ToBigInt::to_bigint(&isize::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigInt::to_bigint(&isize::MAX),
            ToBigInt::to_bigint(&isize::MAX).unwrap()
        );
    }
}
