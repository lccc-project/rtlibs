use core::cmp::Ordering;

pub unsafe trait ArchFloat: Sized {
    /// Computes `x + y` within 0.5 ULP
    fn fadd(x: Self, y: Self) -> Self;
    /// Computes `x - y` within 0.5 ULP
    fn fsub(x: Self, y: Self) -> Self;
    /// Computes `x * y` within 0.5 ULP
    fn fmul(x: Self, y: Self) -> Self;
    /// Computes `x / y` within 0.5 ULP
    fn fdiv(x: Self, y: Self) -> Self;

    /// Computes `-x`
    ///
    /// This is a guaranteed exact operation
    fn fneg(x: Self) -> Self;

    /// Computes `1/x` within 0.5 ULP
    fn frecip(x: Self) -> Self;

    /// Computes `x==y`
    fn feq(x: Self, y: Self) -> bool;
}

pub unsafe trait ArchFloatCmp: ArchFloat {
    /// Compares two floating-point values, with incomparable values being treated as greater than
    fn fcmpgt(x: Self, y: Self) -> Ordering;

    /// Compares two floating-point values, with incomparable
    fn fcmplt(x: Self, y: Self) -> Ordering;
}

pub unsafe trait ArchFloatFma: ArchFloat {
    /// Computes `x * y + z` within 0.5 ULP
    fn ffma(x: Self, y: Self, z: Self) -> Self;
}

pub unsafe trait ArchFloatConvert<F: ArchFloat>: ArchFloat{
    fn round_to_dest(self) -> F;
}

pub unsafe trait ArchFloatConvertInt: ArchFloat{
    fn to_i32_sat(self) -> i32;
    fn to_i64_sat(self) -> i64;
    fn to_i128_sat(self) -> i128;
}


#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::{f128_abi, f16_abi, fx2x64_abi};


