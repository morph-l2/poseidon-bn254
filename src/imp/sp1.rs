use crate::{Fr, State, T};
use sp1_intrinsics::{
    bn254::syscall_bn254_scalar_mac,
    memory::{memcpy32, memcpy64},
};
use std::mem::MaybeUninit;

#[inline(always)]
pub(crate) fn sbox_inplace(val: &mut Fr) {
    let mut temp = MaybeUninit::<Fr>::uninit();
    let zero = Fr::zero();
    let mut temp2 = MaybeUninit::<Fr>::uninit();

    let mut temp3 = MaybeUninit::<Fr>::uninit();

    let mut temp4 = MaybeUninit::<Fr>::uninit();

    let mut temp5 = MaybeUninit::<Fr>::uninit();
    unsafe {
        let ptr = temp.as_mut_ptr();
        memcpy32(&zero, ptr);
        let qtr = temp2.as_mut_ptr();
        memcpy32(val, qtr);

        let ttr = temp3.as_mut_ptr();
        let utr = temp4.as_mut_ptr();
        let vtr = temp5.as_mut_ptr();
        memcpy32(&zero, utr);
        memcpy32(&zero, vtr);
        syscall_bn254_scalar_mac(ptr, val as *const Fr, qtr as *const Fr); // ptr = val * val (val^2)

        memcpy32(ptr, ttr);
        syscall_bn254_scalar_mac(utr, ptr as *const Fr, ttr as *const Fr); // utr = val^4

        syscall_bn254_scalar_mac(vtr, utr as *const Fr, val as *const Fr); // utr = val^4

        memcpy32(vtr, val);
    };
}

#[inline(always)]
pub(crate) fn fill_state(state: &mut MaybeUninit<State>, val: &Fr) {
    let ptr = state.as_mut_ptr() as *mut Fr;
    for i in 0..T {
        unsafe {
            memcpy32(val, ptr.add(i));
        }
    }
}

#[inline(always)]
pub(crate) fn set_state(state: &mut State, new_state: &State) {
    unsafe {
        memcpy32(&new_state[0], &mut state[0]);
        memcpy32(&new_state[1], &mut state[1]);
        memcpy32(&new_state[2], &mut state[2]);
    }
}

#[inline(always)]
pub(crate) fn init_state_with_cap_and_msg<'a>(
    state: &'a mut MaybeUninit<State>,
    cap: &Fr,
    msg: &[Fr],
) -> &'a mut State {
    static ZERO_TWO: [Fr; 2] = [Fr::zero(), Fr::zero()];

    unsafe {
        let ptr = state.as_mut_ptr() as *mut Fr;
        memcpy32(cap, ptr);
        match msg.len() {
            0 => {
                memcpy64(ZERO_TWO.as_ptr(), ptr.add(1));
            }
            1 => {
                memcpy32(msg.as_ptr(), ptr.add(1));
                memcpy32(ZERO_TWO.as_ptr(), ptr.add(2));
            }
            _ => {
                memcpy64(msg.as_ptr(), ptr.add(1));
            }
        }
        state.assume_init_mut()
    }
}

#[inline(always)]
pub(crate) unsafe fn set_fr(dst: *mut Fr, val: &Fr) {
    unsafe {
        memcpy32(val, dst);
    }
}

#[inline(always)]
pub(crate) fn mul_add_assign(dst: &mut Fr, a: &Fr, b: &Fr) {
    unsafe {
        syscall_bn254_scalar_mac(dst, a, b);
    }
}
