use crate::fp::arch::*;
use crate::fp::*;

use core::arch::asm;

use core::arch::x86_64;

pub type f16_abi = x86_64::__m128;
pub type f128_abi = x86_64::__m128;
pub type fx2x64_abi = x86_64::__m128;

macro_rules! def_arch_fp{
    {
        $($(#[$ty_cfg:meta])* $ty:ident : $constraint:tt $(.$field:tt)? {
            mov:  $fmov_instr:literal;
            fadd: $fadd_instr:literal;
            fsub: $fsub_instr:literal;
            fmul: $fmul_instr:literal;
            fdiv: $fdiv_instr:literal;
            fneg: $fxor_instr:literal @ $fneg_const:literal;
            frecip: $frecip_instr:literal;
            fcmp: $fcmp_instr:literal;
            $(#[$fma_cfg:meta])* ffma: $ffma_instr:literal;
        })*
    } => {
        $(
            $(#[$ty_cfg])* unsafe impl ArchFloat for $ty{
                fn fadd(x: Self, y: Self) -> Self{
                    let z: Self;

                    unsafe{asm!(concat!($fadd_instr)," {}, {}", inout($constraint) x$(.$field)?=>z$(.$field)?, in($constraint) y$(.$field)?, options(nomem,nostack));}
                    z
                }
                fn fsub(x: Self, y: Self) -> Self{
                    let z: Self;

                    unsafe{asm!(concat!($fsub_instr)," {}, {}", inout($constraint) x$(.$field)?=>z$(.$field)?, in($constraint) y$(.$field)?, options(nomem,nostack));}
                    z
                }
                fn fmul(x: Self, y: Self) -> Self{
                    let z: Self;

                    unsafe{asm!(concat!($fmul_instr)," {}, {}", inout($constraint) x$(.$field)?=>z$(.$field)?, in($constraint) y$(.$field)?, options(nomem,nostack));}
                    z
                }
                fn fdiv(x: Self, y: Self) -> Self{
                    let z: Self;

                    unsafe{asm!(concat!($fdiv_instr)," {}, {}", inout($constraint) x$(.$field)?=>z$(.$field)?, in($constraint) y$(.$field)?, options(nomem,nostack));}
                    z
                }
                fn fneg(x: Self) -> Self{
                    let z: Self;

                    let val_bits: u64 = $fneg_const;

                    unsafe{
                        asm!(
                            concat!($fmov_instr, "xmm1, [{ptr}]"),
                            concat!($fxor_instr," {x}, xmm1"),
                            x = inout($constraint) x$(.$field)?=>z$(.$field)?, ptr = in(reg) core::ptr::addr_of!(val_bits), out("xmm1") _, options(pure, readonly,nostack)
                        );
                    }
                    z
                }
                fn frecip(x: Self) -> Self{
                    let z: Self;

                    unsafe{asm!(concat!($frecip_instr)," {}", inout($constraint) x$(.$field)?=>z$(.$field)?, options(nomem,nostack));}
                    z
                }

                fn feq(x: Self, y: Self) -> bool{
                    let z: u32;

                    unsafe{asm!(
                        concat!(
                            $fcmp_instr,
                            " {x}, {y}, 0"
                        ),
                        "movd {res:e}, {x}",
                        x = inout($constraint) x$(.$field)?=>_, y = in($constraint) y$(.$field)?, res = out(reg) z, options(nomem,nostack,pure)
                    )}

                    z==0
                }
            }
            $(#[$ty_cfg])* unsafe impl ArchFloatCmp for $ty{
                fn fcmplt(x: Self, y: Self) -> core::cmp::Ordering{
                    use core::cmp::Ordering;
                    let z: i32;
                    unsafe{asm!(
                        concat!($fmov_instr, " xmm0, {x}"),
                        concat!($fcmp_instr, "xmm0, {y}, 1"),
                        concat!($fcmp_instr, "{x}, {y}, 0"),
                        "movd eax, xmm0",
                        "xor {res:e}, {res:e}",
                        "test eax, eax",
                        "cmovne {res:e}, 0xFFFFFFFF",
                        "movd ecx, {x}",
                        "or eax, ecx",
                        "test eax, eax",
                        "cmovne {res:e}, 1",
                        x = inout($constraint) x$(.$field)?=>_, y = in($constraint) y$(.$field)?, res = out(reg) z, out("xmm0") _, out("eax") _, out("ecx") _,options(nomem,nostack)
                    )}

                    match z{
                        1 => Ordering::Greater,
                        0 => Ordering::Equal,
                        -1 => Ordering::Less,
                        _ => unsafe{core::hint::unreachable_unchecked()}
                    }
                }
                fn fcmpgt(x: Self, y: Self) -> core::cmp::Ordering{
                    use core::cmp::Ordering;
                    let z: i32;
                    unsafe{
                        asm!(
                            concat!($fmov_instr, " xmm0, {x}"),
                            concat!($fcmp_instr, "xmm0, {y}, 6"),
                            "movd eax, {x}",
                            concat!($fcmp_instr, "xmm0, {y}, 3"),
                            concat!($fcmp_instr, "{x}, {y}, 0"),
                            "xor {res:e}, {res:e}",
                            "test eax, eax",
                            "cmovne {res:e}, 1",
                            "movd ecx, {x}",
                            "and eax, ecx",
                            "not eax",
                            "movd ecx, xmm0",
                            "or eax, ecx",
                            "test eax, eax",
                            "cmovne {res:e}, 0xFFFFFFFF",
                            x = inout($constraint) x$(.$field)?=>_, y = in($constraint) y$(.$field)?, res = out(reg) z, out("xmm0") _, out("eax") _, out("ecx") _, options(nomem,nostack)
                        )

                    }
                    match z{
                        1 => Ordering::Greater,
                        0 => Ordering::Equal,
                        -1 => Ordering::Less,
                        _ => unsafe{core::hint::unreachable_unchecked()}
                    }
                }
            }
            $(#[$ty_cfg])* const _: () = {
                $(#[$fma_cfg])* unsafe impl ArchFloatFma for $ty{
                    fn ffma(x: Self, y: Self, z: Self) -> Self{
                        let res: Self;
                        unsafe{
                            asm!(concat!($ffma_instr," {x}, {y}, {z}"), x=inout($constraint) x$(.$field)?=>res$(.$field)?, y = in($constraint) y$(.$field)?, z = in($constraint) z$(.$field)?)
                        }

                        res
                    }
                }
            };
        )*
    }
}

def_arch_fp! {
    f32: xmm_reg {
        mov:  "movss";
        fadd: "addss";
        fsub: "subss";
        fmul: "mulss";
        fdiv: "divss";
        fneg: "xorps" @ 0x80000000;
        frecip: "recipss";
        fcmp: "cmpss";
        #[cfg(target_feature = "fma")] ffma: "vfmadd213ss";
    }
    f64: xmm_reg {
        mov:  "movsd";
        fadd: "addsd";
        fsub: "subsd";
        fmul: "mulsd";
        fdiv: "divsd";
        fneg: "xorpd" @ 0x8000000000000000;
        frecip: "recipsd";
        fcmp: "cmpsd";
        #[cfg(target_feature = "fma")] ffma: "vfmadd213sd";
    }
    #[cfg(target_feature = "avx512fp16")] f16: xmm_reg .0{
        mov: "vmovsh";
        fadd: "vaddsh";
        fsub: "vsubsh";
        fmul: "vmulsh";
        fdiv: "vdivsh";
        fneg: "vxorps" @ 0x8000;
        frecip: "vrcpsh";
        fcmp: "vcmpsh";
        ffma: "vfmadd213sh";
    }
}
