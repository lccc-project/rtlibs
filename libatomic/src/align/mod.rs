pub unsafe trait NaturalAlignment: Sized {
    type Align: AlignTy;
}

#[repr(C)]
pub struct Align1;

#[repr(C, align(2))]
pub struct Align2;

#[repr(C, align(4))]
pub struct Align4;

#[repr(C, align(8))]
pub struct Align8;

#[repr(C, align(16))]
pub struct Align16;

unsafe impl AlignTy for Align1 {}
unsafe impl AlignTy for Align2 {}
unsafe impl AlignTy for Align4 {}
unsafe impl AlignTy for Align8 {}
unsafe impl AlignTy for Align16 {}

use core::mem::MaybeUninit;

use internal::AlignTy;

pub const fn new_alignment<T: AlignTy>() -> T {
    unsafe { MaybeUninit::<T>::uninit().assume_init() }
}

mod internal {

    pub unsafe trait AlignTy {}

    pub const fn clamp_round_size<T>() -> usize {
        let x = core::mem::size_of::<T>();

        let y = x.next_power_of_two();

        if y > 16 {
            16
        } else {
            y
        }
    }

    pub unsafe trait DoNaturalAlignment: Sized {
        type Align: AlignTy;
    }

    unsafe impl DoNaturalAlignment for [u8; 1] {
        type Align = super::Align1;
    }

    unsafe impl DoNaturalAlignment for [u8; 2] {
        type Align = super::Align2;
    }

    unsafe impl DoNaturalAlignment for [u8; 4] {
        type Align = super::Align4;
    }

    unsafe impl DoNaturalAlignment for [u8; 8] {
        type Align = super::Align8;
    }

    unsafe impl DoNaturalAlignment for [u8; 16] {
        type Align = super::Align16;
    }
}

unsafe impl<T> NaturalAlignment for T
where
    [u8; internal::clamp_round_size::<T>()]: internal::DoNaturalAlignment,
{
    type Align = <[u8; internal::clamp_round_size::<T>()] as internal::DoNaturalAlignment>::Align;
}
