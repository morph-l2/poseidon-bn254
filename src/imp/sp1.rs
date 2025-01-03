use crate::{Fr, State, T};
use core::arch::asm;
use std::mem::MaybeUninit;

const MEMCPY_32: u32 = 0x00_01_01_90;
const MEMCPY_64: u32 = 0x00_01_01_91;
const BN254_MULADD: u32 = 0x00_01_01_1F;

#[inline(always)]
pub(crate) fn mul_add_assign(a: &mut Fr, b: &Fr, c: &Fr) {
    unsafe {
        asm!(
            "ecall",
            in("t0") BN254_MULADD,
            in("a0") a,
            in("a1") &[b, c],
            in("a2") &Fr::zero(),
        );
    }
}

#[inline(always)]
pub(crate) fn sbox_inplace(val: &mut Fr) {
    let mut tmp = Fr::zero();
    let zero = Fr::zero();
    unsafe {
        // Copy val to tmp
        asm!(
            "ecall",
            in("t0") MEMCPY_32,
            in("a0") val,
            in("a1") &mut tmp,
        );
        
        // Calculate x^5 using repeated multiplication
        for _ in 0..4 {
            asm!(
                "ecall",
                in("t0") BN254_MULADD,
                in("a0") &mut tmp,
                in("a1") &[val, &zero],
                in("a2") &zero,
            );
        }

        // Copy result back to val
        asm!(
            "ecall",
            in("t0") MEMCPY_32,
            in("a0") &tmp,
            in("a1") val,
        );
    }
}


#[inline(always)]
pub(crate) fn fill_state(state: &mut MaybeUninit<State>, val: &Fr) {
    for i in 0..T {
        unsafe {
            asm!(
                "ecall",
                in("t0") MEMCPY_32,
                in("a0") val,
                in("a1") (state.as_mut_ptr() as *mut Fr).add(i),
            );
        }
    }
}

#[inline(always)]
pub(crate) fn set_state(state: &mut State, new_state: &State) {
    unsafe {
        asm!(
            "ecall",
            in("t0") MEMCPY_64,
            in("a0") &new_state[0],
            in("a1") &mut state[0],
        );
        asm!(
            "ecall",
            in("t0") MEMCPY_32,
            in("a0") &new_state[2],
            in("a1") &mut state[2],
        );
    }
}

#[inline(always)]
pub(crate) fn init_state_with_cap_and_msg<'a>(
    state: &'a mut MaybeUninit<State>,
    cap: &Fr,
    msg: &[Fr],
) -> &'a mut State {
    const ZERO_TWO: [Fr; 2] = [Fr::zero(), Fr::zero()];

    match msg.len() {
        0 => unsafe {
            asm!(
                "ecall",
                in("t0") MEMCPY_32,
                in("a0") cap,
                in("a1") (state.as_mut_ptr() as *mut Fr),
            );
            asm!(
                "ecall",
                in("t0") MEMCPY_64,
                in("a0") &ZERO_TWO,
                in("a1") (state.as_mut_ptr() as *mut Fr).add(1),
            );
        },
        1 => unsafe {
            asm!(
                "ecall",
                in("t0") MEMCPY_32,
                in("a0") cap,
                in("a1") (state.as_mut_ptr() as *mut Fr),
            );
            asm!(
                "ecall",
                in("t0") MEMCPY_32,
                in("a0") msg.as_ptr(),
                in("a1") (state.as_mut_ptr() as *mut Fr).add(1),
            );
            asm!(
                "ecall",
                in("t0") MEMCPY_32,
                in("a0") &ZERO_TWO[1],
                in("a1") (state.as_mut_ptr() as *mut Fr).add(2),
            );
        },
        _ => unsafe {
            asm!(
                "ecall",
                in("t0") MEMCPY_32,
                in("a0") cap,
                in("a1") (state.as_mut_ptr() as *mut Fr),
            );
            asm!(
                "ecall",
                in("t0") MEMCPY_64,
                in("a0") msg.as_ptr(),
                in("a1") (state.as_mut_ptr() as *mut Fr).add(1),
            );
        },
    }

    unsafe { state.assume_init_mut() }
}

#[inline(always)]
pub(crate) unsafe fn set_fr(dst: *mut Fr, val: &Fr) {
    unsafe {
        asm!(
            "ecall",
            in("t0") MEMCPY_32,
            in("a0") val,
            in("a1") dst,
        );
    }
}