use crate::{Fr, State, T};
use sp1_intrinsics::{
    bn254::syscall_bn254_scalar_mac,
    memory::{memcpy32, memcpy64},
};
use std::mem::MaybeUninit;

#[inline(always)]
pub(crate) fn sbox_inplace(val: &mut Fr) {
    // 计算 x^5 = ((x^2)^2) * x
    let mut tmp = MaybeUninit::<Fr>::uninit();
    
    unsafe {
        let ptr = tmp.as_mut_ptr();
        // 初始化为0
        *ptr = Fr::zero();
        // 计算 x^2 (0 + x*x)
        syscall_bn254_scalar_mac(ptr, val, val);
        // 计算 x^4 (0 + x^2*x^2)
        let x2 = *ptr;
        *ptr = Fr::zero();
        syscall_bn254_scalar_mac(ptr, &x2, &x2);
        // 计算 x^5 (0 + x^4*x)
        let x4 = *ptr;
        *ptr = Fr::zero();
        syscall_bn254_scalar_mac(ptr, &x4, val);
        // 写回结果
        *val = *ptr;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbox() {
        let mut val = Fr::from(2u64);
        sbox_inplace(&mut val);
    }

    #[test]
    fn test_state_ops() {
        let mut state = MaybeUninit::<State>::uninit();
        let val = Fr::from(1u64);
        fill_state(&mut state, &val);
        
        let mut state = unsafe { state.assume_init() };
        let new_state = [Fr::from(2u64); T];
        set_state(&mut state, &new_state);
    }
}