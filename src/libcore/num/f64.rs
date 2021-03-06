// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Operations and constants for 64-bits floats (`f64` type)

use default::Default;
use intrinsics;
use mem;
use num::{FPNormal, FPCategory, FPZero, FPSubnormal, FPInfinite, FPNaN};
use num::{Zero, One, Bounded, Signed, Num, Primitive, Float};
use option::Option;

#[cfg(not(test))] use cmp::{Eq, Ord};
#[cfg(not(test))] use ops::{Add, Sub, Mul, Div, Rem, Neg};

// FIXME(#5527): These constants should be deprecated once associated
// constants are implemented in favour of referencing the respective
// members of `Bounded` and `Float`.

pub static RADIX: uint = 2u;

pub static MANTISSA_DIGITS: uint = 53u;
pub static DIGITS: uint = 15u;

pub static EPSILON: f64 = 2.2204460492503131e-16_f64;

/// Smallest finite f64 value
pub static MIN_VALUE: f64 = -1.7976931348623157e+308_f64;
/// Smallest positive, normalized f64 value
pub static MIN_POS_VALUE: f64 = 2.2250738585072014e-308_f64;
/// Largest finite f64 value
pub static MAX_VALUE: f64 = 1.7976931348623157e+308_f64;

pub static MIN_EXP: int = -1021;
pub static MAX_EXP: int = 1024;

pub static MIN_10_EXP: int = -307;
pub static MAX_10_EXP: int = 308;

pub static NAN: f64 = 0.0_f64/0.0_f64;

pub static INFINITY: f64 = 1.0_f64/0.0_f64;

pub static NEG_INFINITY: f64 = -1.0_f64/0.0_f64;

/// Various useful constants.
pub mod consts {
    // FIXME: replace with mathematical constants from cmath.

    // FIXME(#5527): These constants should be deprecated once associated
    // constants are implemented in favour of referencing the respective members
    // of `Float`.

    /// Archimedes' constant
    pub static PI: f64 = 3.14159265358979323846264338327950288_f64;

    /// pi * 2.0
    pub static PI_2: f64 = 6.28318530717958647692528676655900576_f64;

    /// pi/2.0
    pub static FRAC_PI_2: f64 = 1.57079632679489661923132169163975144_f64;

    /// pi/3.0
    pub static FRAC_PI_3: f64 = 1.04719755119659774615421446109316763_f64;

    /// pi/4.0
    pub static FRAC_PI_4: f64 = 0.785398163397448309615660845819875721_f64;

    /// pi/6.0
    pub static FRAC_PI_6: f64 = 0.52359877559829887307710723054658381_f64;

    /// pi/8.0
    pub static FRAC_PI_8: f64 = 0.39269908169872415480783042290993786_f64;

    /// 1.0/pi
    pub static FRAC_1_PI: f64 = 0.318309886183790671537767526745028724_f64;

    /// 2.0/pi
    pub static FRAC_2_PI: f64 = 0.636619772367581343075535053490057448_f64;

    /// 2.0/sqrt(pi)
    pub static FRAC_2_SQRTPI: f64 = 1.12837916709551257389615890312154517_f64;

    /// sqrt(2.0)
    pub static SQRT2: f64 = 1.41421356237309504880168872420969808_f64;

    /// 1.0/sqrt(2.0)
    pub static FRAC_1_SQRT2: f64 = 0.707106781186547524400844362104849039_f64;

    /// Euler's number
    pub static E: f64 = 2.71828182845904523536028747135266250_f64;

    /// log2(e)
    pub static LOG2_E: f64 = 1.44269504088896340735992468100189214_f64;

    /// log10(e)
    pub static LOG10_E: f64 = 0.434294481903251827651128918916605082_f64;

    /// ln(2.0)
    pub static LN_2: f64 = 0.693147180559945309417232121458176568_f64;

    /// ln(10.0)
    pub static LN_10: f64 = 2.30258509299404568401799145468436421_f64;
}

#[cfg(not(test))]
impl Ord for f64 {
    #[inline]
    fn lt(&self, other: &f64) -> bool { (*self) < (*other) }
    #[inline]
    fn le(&self, other: &f64) -> bool { (*self) <= (*other) }
    #[inline]
    fn ge(&self, other: &f64) -> bool { (*self) >= (*other) }
    #[inline]
    fn gt(&self, other: &f64) -> bool { (*self) > (*other) }
}
#[cfg(not(test))]
impl Eq for f64 {
    #[inline]
    fn eq(&self, other: &f64) -> bool { (*self) == (*other) }
}

impl Default for f64 {
    #[inline]
    fn default() -> f64 { 0.0 }
}

impl Primitive for f64 {}

impl Num for f64 {}

impl Zero for f64 {
    #[inline]
    fn zero() -> f64 { 0.0 }

    /// Returns true if the number is equal to either `0.0` or `-0.0`
    #[inline]
    fn is_zero(&self) -> bool { *self == 0.0 || *self == -0.0 }
}

impl One for f64 {
    #[inline]
    fn one() -> f64 { 1.0 }
}

#[cfg(not(test))]
impl Add<f64,f64> for f64 {
    #[inline]
    fn add(&self, other: &f64) -> f64 { *self + *other }
}
#[cfg(not(test))]
impl Sub<f64,f64> for f64 {
    #[inline]
    fn sub(&self, other: &f64) -> f64 { *self - *other }
}
#[cfg(not(test))]
impl Mul<f64,f64> for f64 {
    #[inline]
    fn mul(&self, other: &f64) -> f64 { *self * *other }
}
#[cfg(not(test))]
impl Div<f64,f64> for f64 {
    #[inline]
    fn div(&self, other: &f64) -> f64 { *self / *other }
}
#[cfg(not(test))]
impl Rem<f64,f64> for f64 {
    #[inline]
    fn rem(&self, other: &f64) -> f64 {
        extern { fn fmod(a: f64, b: f64) -> f64; }
        unsafe { fmod(*self, *other) }
    }
}
#[cfg(not(test))]
impl Neg<f64> for f64 {
    #[inline]
    fn neg(&self) -> f64 { -*self }
}

impl Signed for f64 {
    /// Computes the absolute value. Returns `NAN` if the number is `NAN`.
    #[inline]
    fn abs(&self) -> f64 {
        unsafe { intrinsics::fabsf64(*self) }
    }

    /// The positive difference of two numbers. Returns `0.0` if the number is less than or
    /// equal to `other`, otherwise the difference between`self` and `other` is returned.
    #[inline]
    fn abs_sub(&self, other: &f64) -> f64 {
        extern { fn fdim(a: f64, b: f64) -> f64; }
        unsafe { fdim(*self, *other) }
    }

    /// # Returns
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is NaN
    #[inline]
    fn signum(&self) -> f64 {
        if self != self { NAN } else {
            unsafe { intrinsics::copysignf64(1.0, *self) }
        }
    }

    /// Returns `true` if the number is positive, including `+0.0` and `INFINITY`
    #[inline]
    fn is_positive(&self) -> bool { *self > 0.0 || (1.0 / *self) == INFINITY }

    /// Returns `true` if the number is negative, including `-0.0` and `NEG_INFINITY`
    #[inline]
    fn is_negative(&self) -> bool { *self < 0.0 || (1.0 / *self) == NEG_INFINITY }
}

impl Bounded for f64 {
    // NOTE: this is the smallest non-infinite f32 value, *not* MIN_VALUE
    #[inline]
    fn min_value() -> f64 { -MAX_VALUE }

    #[inline]
    fn max_value() -> f64 { MAX_VALUE }
}

impl Float for f64 {
    #[inline]
    fn nan() -> f64 { NAN }

    #[inline]
    fn infinity() -> f64 { INFINITY }

    #[inline]
    fn neg_infinity() -> f64 { NEG_INFINITY }

    #[inline]
    fn neg_zero() -> f64 { -0.0 }

    /// Returns `true` if the number is NaN
    #[inline]
    fn is_nan(self) -> bool { self != self }

    /// Returns `true` if the number is infinite
    #[inline]
    fn is_infinite(self) -> bool {
        self == Float::infinity() || self == Float::neg_infinity()
    }

    /// Returns `true` if the number is neither infinite or NaN
    #[inline]
    fn is_finite(self) -> bool {
        !(self.is_nan() || self.is_infinite())
    }

    /// Returns `true` if the number is neither zero, infinite, subnormal or NaN
    #[inline]
    fn is_normal(self) -> bool {
        self.classify() == FPNormal
    }

    /// Returns the floating point category of the number. If only one property
    /// is going to be tested, it is generally faster to use the specific
    /// predicate instead.
    fn classify(self) -> FPCategory {
        static EXP_MASK: u64 = 0x7ff0000000000000;
        static MAN_MASK: u64 = 0x000fffffffffffff;

        let bits: u64 = unsafe { mem::transmute(self) };
        match (bits & MAN_MASK, bits & EXP_MASK) {
            (0, 0)        => FPZero,
            (_, 0)        => FPSubnormal,
            (0, EXP_MASK) => FPInfinite,
            (_, EXP_MASK) => FPNaN,
            _             => FPNormal,
        }
    }

    #[inline]
    fn mantissa_digits(_: Option<f64>) -> uint { MANTISSA_DIGITS }

    #[inline]
    fn digits(_: Option<f64>) -> uint { DIGITS }

    #[inline]
    fn epsilon() -> f64 { EPSILON }

    #[inline]
    fn min_exp(_: Option<f64>) -> int { MIN_EXP }

    #[inline]
    fn max_exp(_: Option<f64>) -> int { MAX_EXP }

    #[inline]
    fn min_10_exp(_: Option<f64>) -> int { MIN_10_EXP }

    #[inline]
    fn max_10_exp(_: Option<f64>) -> int { MAX_10_EXP }

    #[inline]
    fn min_pos_value(_: Option<f64>) -> f64 { MIN_POS_VALUE }

    /// Returns the mantissa, exponent and sign as integers.
    fn integer_decode(self) -> (u64, i16, i8) {
        let bits: u64 = unsafe { mem::transmute(self) };
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };
        // Exponent bias + mantissa shift
        exponent -= 1023 + 52;
        (mantissa, exponent, sign)
    }

    /// Round half-way cases toward `NEG_INFINITY`
    #[inline]
    fn floor(self) -> f64 {
        unsafe { intrinsics::floorf64(self) }
    }

    /// Round half-way cases toward `INFINITY`
    #[inline]
    fn ceil(self) -> f64 {
        unsafe { intrinsics::ceilf64(self) }
    }

    /// Round half-way cases away from `0.0`
    #[inline]
    fn round(self) -> f64 {
        unsafe { intrinsics::roundf64(self) }
    }

    /// The integer part of the number (rounds towards `0.0`)
    #[inline]
    fn trunc(self) -> f64 {
        unsafe { intrinsics::truncf64(self) }
    }

    /// The fractional part of the number, satisfying:
    ///
    /// ```rust
    /// let x = 1.65f64;
    /// assert!(x == x.trunc() + x.fract())
    /// ```
    #[inline]
    fn fract(self) -> f64 { self - self.trunc() }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error. This produces a more accurate result with better performance than
    /// a separate multiplication operation followed by an add.
    #[inline]
    fn mul_add(self, a: f64, b: f64) -> f64 {
        unsafe { intrinsics::fmaf64(self, a, b) }
    }

    /// The reciprocal (multiplicative inverse) of the number
    #[inline]
    fn recip(self) -> f64 { 1.0 / self }

    #[inline]
    fn powf(self, n: f64) -> f64 {
        unsafe { intrinsics::powf64(self, n) }
    }

    #[inline]
    fn powi(self, n: i32) -> f64 {
        unsafe { intrinsics::powif64(self, n) }
    }

    /// sqrt(2.0)
    #[inline]
    fn sqrt2() -> f64 { consts::SQRT2 }

    /// 1.0 / sqrt(2.0)
    #[inline]
    fn frac_1_sqrt2() -> f64 { consts::FRAC_1_SQRT2 }

    #[inline]
    fn sqrt(self) -> f64 {
        unsafe { intrinsics::sqrtf64(self) }
    }

    #[inline]
    fn rsqrt(self) -> f64 { self.sqrt().recip() }

    /// Archimedes' constant
    #[inline]
    fn pi() -> f64 { consts::PI }

    /// 2.0 * pi
    #[inline]
    fn two_pi() -> f64 { consts::PI_2 }

    /// pi / 2.0
    #[inline]
    fn frac_pi_2() -> f64 { consts::FRAC_PI_2 }

    /// pi / 3.0
    #[inline]
    fn frac_pi_3() -> f64 { consts::FRAC_PI_3 }

    /// pi / 4.0
    #[inline]
    fn frac_pi_4() -> f64 { consts::FRAC_PI_4 }

    /// pi / 6.0
    #[inline]
    fn frac_pi_6() -> f64 { consts::FRAC_PI_6 }

    /// pi / 8.0
    #[inline]
    fn frac_pi_8() -> f64 { consts::FRAC_PI_8 }

    /// 1.0 / pi
    #[inline]
    fn frac_1_pi() -> f64 { consts::FRAC_1_PI }

    /// 2.0 / pi
    #[inline]
    fn frac_2_pi() -> f64 { consts::FRAC_2_PI }

    /// 2.0 / sqrt(pi)
    #[inline]
    fn frac_2_sqrtpi() -> f64 { consts::FRAC_2_SQRTPI }

    /// Euler's number
    #[inline]
    fn e() -> f64 { consts::E }

    /// log2(e)
    #[inline]
    fn log2_e() -> f64 { consts::LOG2_E }

    /// log10(e)
    #[inline]
    fn log10_e() -> f64 { consts::LOG10_E }

    /// ln(2.0)
    #[inline]
    fn ln_2() -> f64 { consts::LN_2 }

    /// ln(10.0)
    #[inline]
    fn ln_10() -> f64 { consts::LN_10 }

    /// Returns the exponential of the number
    #[inline]
    fn exp(self) -> f64 {
        unsafe { intrinsics::expf64(self) }
    }

    /// Returns 2 raised to the power of the number
    #[inline]
    fn exp2(self) -> f64 {
        unsafe { intrinsics::exp2f64(self) }
    }

    /// Returns the natural logarithm of the number
    #[inline]
    fn ln(self) -> f64 {
        unsafe { intrinsics::logf64(self) }
    }

    /// Returns the logarithm of the number with respect to an arbitrary base
    #[inline]
    fn log(self, base: f64) -> f64 { self.ln() / base.ln() }

    /// Returns the base 2 logarithm of the number
    #[inline]
    fn log2(self) -> f64 {
        unsafe { intrinsics::log2f64(self) }
    }

    /// Returns the base 10 logarithm of the number
    #[inline]
    fn log10(self) -> f64 {
        unsafe { intrinsics::log10f64(self) }
    }


    /// Converts to degrees, assuming the number is in radians
    #[inline]
    fn to_degrees(self) -> f64 { self * (180.0f64 / Float::pi()) }

    /// Converts to radians, assuming the number is in degrees
    #[inline]
    fn to_radians(self) -> f64 {
        let value: f64 = Float::pi();
        self * (value / 180.0)
    }
}

