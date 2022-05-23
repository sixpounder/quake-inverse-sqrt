//! # quake-inverse-sqrt
//!
//! This crate implements a trait for all numeric base types to give them the ability to compute
//! their own inverse square root using the infamous Quake III algorithm.
//!
//! The result is approximated in favour of speed of execution.
const THREE_HALFS: f32 = 1.5;
const WTF: u32 = 0x5f3759df;

#[derive(Debug)]
pub enum QSqrtError {
    Overflow
}

/// A trait to implement fast inverse square root for
/// a variety of types
pub trait QSqrt {
    type Output;
    fn fast_inverse_sqrt(&self) -> Result<Self::Output, QSqrtError>;
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
        unsafe {
            i = std::mem::transmute::<Self, u32>(y);
        }

        // What the f*ck
        i = WTF - (i >> 1);

        unsafe {
            y = std::mem::transmute::<u32, Self>(i);
        }

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

impl_types!(u32, u16, u8, i32, i16, i8, usize, isize);

#[cfg(test)]
mod tests {
    use crate::QSqrt;

    #[test]
    fn float_input() {
        let x: f32 = 4.;
        let res = x.fast_inverse_sqrt_unchecked();
        assert!(res > 0.49 && res < 0.51);
    }

    #[test]
    fn float_64_input() {
        let x: f64 = 4.;
        let res = x.fast_inverse_sqrt_unchecked();
        assert!(res > 0.49 && res < 0.51);
    }

    #[test]
    fn uint32_input() {
        let x: u32 = 4;
        let res = x.fast_inverse_sqrt_unchecked();
        assert!(res > 0.49 && res < 0.51);
    }

    #[test]
    fn uint16_input() {
        let x: u16 = 4;
        let res = x.fast_inverse_sqrt_unchecked();
        assert!(res > 0.49 && res < 0.51);
    }

    #[test]
    fn uint8_input() {
        let x: u8 = 4;
        let res = x.fast_inverse_sqrt_unchecked();
        assert!(res > 0.49 && res < 0.51);
    }

    #[test]
    fn int32_input() {
        let x: i32 = 4;
        let res = x.fast_inverse_sqrt_unchecked();
        assert!(res > 0.49 && res < 0.51);
    }

    #[test]
    fn int16_input() {
        let x: i16 = 4;
        let res = x.fast_inverse_sqrt_unchecked();
        assert!(res > 0.49 && res < 0.51);
    }

    #[test]
    fn int8_input() {
        let x: i8 = 4;
        let res = x.fast_inverse_sqrt_unchecked();
        assert!(res > 0.49 && res < 0.51);
    }
}

