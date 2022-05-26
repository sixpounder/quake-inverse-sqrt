//! # quake-inverse-sqrt
//!
//! This crate implements a trait for all numeric base types to give them the ability to compute
//! their own inverse square root using the infamous Quake III algorithm.
//!
//! The result is approximated in favour of speed of execution.
//!
//! # Example
//!
//! ```
//! let num: f32 = 4.0.fast_inverse_sqrt();
//! assert!(num > 0.49 && num < 0.51);
//! ```
const THREE_HALFS: f32 = 1.5;
const WTF: u32 = 0x5f3759df;

#[derive(Debug)]
pub enum QSqrtError {
    Overflow,
}

/// A trait to implement fast inverse square root for
/// a variety of types
pub trait QSqrt {
    type Output;

    /// Computes the fast inverse square root of `self`
    fn fast_inverse_sqrt(&self) -> Result<Self::Output, QSqrtError>;

    /// Like `fast_inverse_sqrt` but panics on errors
    fn fast_inverse_sqrt_unchecked(&self) -> Self::Output {
        self.fast_inverse_sqrt().unwrap()
    }
}

impl QSqrt for f32 {
    type Output = f32;

    fn fast_inverse_sqrt(&self) -> Result<Self::Output, QSqrtError> {
        let mut y = *self;
        let mut i: u32;
        let x2: f32 = self * 0.5;

        // Evil bit hack
        i = y.to_bits();

        // What the f*ck
        i = WTF - (i >> 1);

        y = f32::from_bits(i);

        // Newton iteration
        y = y * (THREE_HALFS - (x2 * y * y));

        Ok(y)
    }
}

impl QSqrt for f64 {
    type Output = f32;

    fn fast_inverse_sqrt(&self) -> Result<Self::Output, QSqrtError> {
        if *self >= f32::MIN.into() && *self <= f32::MAX.into() {
            (*self as f32).fast_inverse_sqrt()
        } else {
            Err(QSqrtError::Overflow)
        }
    }
}

macro_rules! impl_types {
    ( $($ty: ty),* ) => {
        $(
            impl QSqrt for $ty {
                type Output = f32;

                fn fast_inverse_sqrt(&self) -> Result<Self::Output, QSqrtError> {
                    let value = *self as f32;
                    value.fast_inverse_sqrt()
                }
            }
        )*
    };
}

impl_types!(u64, u32, u16, u8, i64, i32, i16, i8, usize, isize);

#[cfg(test)]
mod tests {
    use crate::QSqrt;

    macro_rules! make_test {
        ($name: tt, $ty: ty, $value: expr, $expected_lower_bound: expr, $expected_upper_bound: expr) => {
            #[test]
            fn $name() {
                let x: $ty = $value;
                let res = x.fast_inverse_sqrt_unchecked();
                assert!(res > $expected_lower_bound && res < $expected_upper_bound);
            }
        };
    }

    make_test!(f32_input, f32, 4., 0.49, 0.51);
    make_test!(f64_input, f64, 4., 0.49, 0.51);
    make_test!(u64_input, u64, 4, 0.49, 0.51);
    make_test!(u32_input, u32, 4, 0.49, 0.51);
    make_test!(u16_input, u16, 4, 0.49, 0.51);
    make_test!(u8_input, i8, 4, 0.49, 0.51);
    make_test!(i64_input, i64, 4, 0.49, 0.51);
    make_test!(i32_input, i32, 4, 0.49, 0.51);
    make_test!(i16_input, i16, 4, 0.49, 0.51);
    make_test!(i8_input, i8, 4, 0.49, 0.51);
}


