pub mod arch;

#[repr(transparent)]
pub struct f16(arch::f16_abi);

#[repr(transparent)]
pub struct f128(arch::f128_abi);

#[repr(transparent)]
pub struct fx2x64(arch::fx2x64_abi);

macro_rules! impl_abi_types{
    ($($fp_ty:ident: $int_ty:ident;)*) => {
        $(impl $fp_ty{
            pub const fn to_bits(self) -> $int_ty{
                let x: [$int_ty; core::mem::size_of::<Self>()/core::mem::size_of::<$int_ty>()] = unsafe{core::mem::transmute(self)};
                match x{
                    [v, ..] => v
                }
            }
            pub const fn from_bits(bits: $int_ty) -> Self{
                let mut x: [$int_ty; core::mem::size_of::<Self>()/core::mem::size_of::<$int_ty>()] = [0;core::mem::size_of::<Self>()/core::mem::size_of::<$int_ty>()];
                x[0] = bits;

                unsafe{core::mem::transmute(x)}
            }

            pub const fn to_be(self) -> Self{
                Self::from_bits(self.to_bits().to_be())
            }
            pub const fn to_le(self) -> Self{
                Self::from_bits(self.to_bits().to_le())
            }
        })*
    }
}

impl_abi_types! {
    f16: u16;
    f128: u128;
    fx2x64: u128;
}
