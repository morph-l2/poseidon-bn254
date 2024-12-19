use crate::{Fr, State, T};
use sp1_intrinsics::{
    bn254::syscall_bn254_muladd,
    memory::{memcpy32, memcpy64},
};
use std::mem::MaybeUninit;

const ZERO_TWO: [Fr; 2] = [Fr::zero(), Fr::zero()];

#[inline(always)]
pub(crate) fn sbox_inplace(val: &mut Fr) {
    let mut tmp = MaybeUninit::<Fr>::uninit();
    unsafe {
        let ptr = tmp.as_mut_ptr();
        memcpy32(&Fr::zero(), ptr);
        
        // Calculate x^2
        syscall_bn254_muladd(ptr, val, val);
        let mut x2 = Fr::zero();
        memcpy32(ptr, &mut x2);
        
        // Calculate x^4
        memcpy32(&Fr::zero(), ptr);
        syscall_bn254_muladd(ptr, &x2, &x2);
        let mut x4 = Fr::zero();
        memcpy32(ptr, &mut x4);
        
        // Calculate x^5 = x^4 * x
        memcpy32(&Fr::zero(), ptr);
        syscall_bn254_muladd(ptr, &x4, val);
        memcpy32(ptr, val);
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
        for i in 0..T {
            memcpy32(&new_state[i], &mut state[i]);
        }
    }
}

#[inline(always)]
pub(crate) fn init_state_with_cap_and_msg<'a>(
    state: &'a mut MaybeUninit<State>,
    cap: &Fr,
    msg: &[Fr],
) -> &'a mut State {
    debug_assert!(msg.len() <= 2, "Message length must be <= 2");

    unsafe {
        let ptr = state.as_mut_ptr() as *mut Fr;
        
        // Set capacity
        memcpy32(cap, ptr);
        
        // Set message elements
        match msg.len() {
            0 => {
                memcpy64(ZERO_TWO.as_ptr(), ptr.add(1));
            }
            1 => {
                memcpy32(msg.as_ptr(), ptr.add(1));
                memcpy32(ZERO_TWO.as_ptr(), ptr.add(2));
            }
            2 => {
                memcpy64(msg.as_ptr(), ptr.add(1));
            }
            _ => unreachable!("Message length checked above"),
        }
        
        state.assume_init_mut()
    }
}

#[inline(always)]
pub(crate) unsafe fn set_fr(dst: *mut Fr, val: &Fr) {
    debug_assert!(!dst.is_null(), "Destination pointer must not be null");
    memcpy32(val, dst);
}

#[inline(always)]
pub(crate) fn mul_add_assign(dst: &mut Fr, a: &Fr, b: &Fr) {
    unsafe {
        syscall_bn254_muladd(dst, a, b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbox() {
        let mut val = Fr::from(2u64);
        let expected = Fr::from(32u64); // 2^5 = 32
        sbox_inplace(&mut val);
        assert_eq!(val, expected);
    }

    #[test]
    fn test_state_ops() {
        let mut state = MaybeUninit::<State>::uninit();
        let val = Fr::from(1u64);
        fill_state(&mut state, &val);
        
        let mut state = unsafe { state.assume_init() };
        let new_state = [Fr::from(2u64); T];
        set_state(&mut state, &new_state);
        
        for i in 0..T {
            assert_eq!(state[i], Fr::from(2u64));
        }
    }

    #[test]
    fn test_init_state() {
        let mut state = MaybeUninit::<State>::uninit();
        let cap = Fr::from(1u64);
        let msg = vec![Fr::from(2u64), Fr::from(3u64)];
        
        let state = init_state_with_cap_and_msg(&mut state, &cap, &msg);
        
        assert_eq!(state[0], Fr::from(1u64));
        assert_eq!(state[1], Fr::from(2u64));
        assert_eq!(state[2], Fr::from(3u64));
    }

    #[test]
    fn test_empty_message() {
        let mut state = MaybeUninit::<State>::uninit();
        let cap = Fr::from(1u64);
        let msg: Vec<Fr> = vec![];
        
        let state = init_state_with_cap_and_msg(&mut state, &cap, &msg);
        
        assert_eq!(state[0], Fr::from(1u64));
        assert_eq!(state[1], Fr::zero());
        assert_eq!(state[2], Fr::zero());
    }
}