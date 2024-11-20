use num::{bigint::ToBigUint, BigUint};

/// Allows for type conversion to [`num::BigUint`] without worrying about Results.
///
/// The types `u8`, `u16`, `u32`, `u64`, `u128`, `usize` are safely convertible to `BigUint`. Only `f32` and `f64` cannot
/// be converted if they are not possitive, whole numbers. This traits limits its implementation to those safe types
/// mentioned. So no error checking is needed.
///
/// # Example
/// ```
/// use infallible_to_big_int::*;
///
/// // use the conversion directly
/// 153830_u32.to_biguint();
///
/// // or define a function which takes any InfallibleToBigUint
/// fn do_great_things(to_biguint: impl InfallibleToBigUint) {
///     let biguint = to_biguint.to_biguint();
///     // ... do something nice with biguint here
/// }
///
/// // then you can call it like this
/// do_great_things(153830_u32)
/// ```
pub trait InfallibleToBigUint {
    fn to_biguint(&self) -> BigUint;
}

impl InfallibleToBigUint for u8 {
    fn to_biguint(&self) -> BigUint {
        ToBigUint::to_biguint(self)
            .expect("to_biguint failed for u8, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigUint for u16 {
    fn to_biguint(&self) -> BigUint {
        ToBigUint::to_biguint(self)
            .expect("to_biguint failed for u16, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigUint for u32 {
    fn to_biguint(&self) -> BigUint {
        ToBigUint::to_biguint(self)
            .expect("to_biguint failed for u32, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigUint for u64 {
    fn to_biguint(&self) -> BigUint {
        ToBigUint::to_biguint(self)
            .expect("to_biguint failed for u64, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigUint for u128 {
    fn to_biguint(&self) -> BigUint {
        ToBigUint::to_biguint(self)
            .expect("to_biguint failed for u128, this should not happen and is most likely a programming error")
    }
}

impl InfallibleToBigUint for usize {
    fn to_biguint(&self) -> BigUint {
        ToBigUint::to_biguint(self)
            .expect("to_biguint failed for usize, this should not happen and is most likely a programming error")
    }
}

#[cfg(test)]
mod tests {
    use std::u8;

    use num::bigint::ToBigUint;

    use super::InfallibleToBigUint;

    /// Test MIN and MAX values of u8
    #[test]
    fn test_u8() {
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u8::MIN),
            ToBigUint::to_biguint(&u8::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u8::MAX),
            ToBigUint::to_biguint(&u8::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of u16
    #[test]
    fn test_u16() {
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u16::MIN),
            ToBigUint::to_biguint(&u16::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u16::MAX),
            ToBigUint::to_biguint(&u16::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of u32
    #[test]
    fn test_u32() {
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u32::MIN),
            ToBigUint::to_biguint(&u32::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u32::MAX),
            ToBigUint::to_biguint(&u32::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of u64
    #[test]
    fn test_u64() {
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u64::MIN),
            ToBigUint::to_biguint(&u64::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u64::MAX),
            ToBigUint::to_biguint(&u64::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of u128
    #[test]
    fn test_u128() {
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u128::MIN),
            ToBigUint::to_biguint(&u128::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigUint::to_biguint(&u128::MAX),
            ToBigUint::to_biguint(&u128::MAX).unwrap()
        );
    }

    /// Test MIN and MAX values of usize
    #[test]
    fn test_usize() {
        assert_eq!(
            InfallibleToBigUint::to_biguint(&usize::MIN),
            ToBigUint::to_biguint(&usize::MIN).unwrap()
        );
        assert_eq!(
            InfallibleToBigUint::to_biguint(&usize::MAX),
            ToBigUint::to_biguint(&usize::MAX).unwrap()
        );
    }
}
