#![allow(clippy::needless_range_loop)]
#![allow(clippy::op_ref)]
#![allow(unexpected_cfgs)]

use itertools::Itertools;
use std::mem::MaybeUninit;
use std::ops::AddAssign;

pub use bn254::{
    ff::{Field, PrimeField},
    Fr,
};

mod constants;

#[cfg(all(
    not(target_os = "zkvm"),
    not(target_vendor = "succinct"),
    feature = "zkvm-hint"
))]
mod zkvm_hints;

#[cfg(all(
    not(target_os = "zkvm"),
    not(target_vendor = "succinct"),
    feature = "zkvm-hint"
))]
pub use zkvm_hints::set_zkvm_hint_hook;

pub(crate) use constants::*;

pub(crate) type State = [Fr; T];
pub(crate) type Mds = [[Fr; T]; T];





/// Hash with domain Fr elements with a specified domain.
pub fn hash_with_domain(inp: &[Fr; 2], domain: Fr) -> Fr {
    let mut state = MaybeUninit::uninit();
    let state = imp::init_state_with_cap_and_msg(&mut state, &domain, inp);
    imp::permute(state);
    state[0]
}

/// Hash a message with an optional capacity.
pub fn hash_msg(msg: &[Fr], cap: Option<u128>) -> Fr {
    let init_cap = Fr::from(cap.unwrap_or(msg.len() as u128));
    let mut msg_idx = 0;
    let mut output = Fr::zero();

    while msg_idx < msg.len() {
        let mut next_inp = [Fr::zero(); 2];
        let remain = msg.len() - msg_idx;
        match remain {
            0 => {}
            1 => next_inp[0] = msg[msg_idx],
            _ => {
                next_inp[0] = msg[msg_idx];
                next_inp[1] = msg[msg_idx + 1];
            }
        }
        output = hash_with_domain(&next_inp, init_cap);
        msg_idx += RATE;
    }
    output
}

/// Hash raw bytes.
pub fn hash_code(code: &[u8]) -> Fr {
    if code.is_empty() {
        return Fr::zero();
    }

    let mut msg = Vec::with_capacity((code.len() + 7) / 8);
    let mut idx = 0;
    while idx < code.len() {
        let remain = code.len() - idx;
        let mut next = 0u64;
        if remain >= 8 {
            next = u64::from_le_bytes(code[idx..idx + 8].try_into().unwrap());
        } else {
            let mut bytes = [0u8; 8];
            bytes[..remain].copy_from_slice(&code[idx..]);
            next = u64::from_le_bytes(bytes);
        }
        msg.push(Fr::from(next));
        idx += 8;
    }

    hash_msg(&msg, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_with_domain() {
        let inp = [Fr::from(1u64), Fr::from(2u64)];
        let domain = Fr::from(3u64);
        let _result = hash_with_domain(&inp, domain);
    }

    #[test]
    fn test_hash_msg() {
        let msg = vec![Fr::from(1u64), Fr::from(2u64), Fr::from(3u64)];
        let _result = hash_msg(&msg, None);
    }

    #[test]
    fn test_hash_code() {
        let code = vec![1u8, 2u8, 3u8, 4u8];
        let _result = hash_code(&code);
    }

    #[test]
    fn test_empty_hash() {
        let result = hash_code(&[]);
        assert_eq!(result, Fr::zero());
    }
}

/// Helper function to convert bytes to Fr elements
#[inline]
fn bytes_to_fr(bytes: &[u8]) -> Vec<Fr> {
    let mut fr_elements = Vec::with_capacity((bytes.len() + 7) / 8);
    let mut idx = 0;

    while idx < bytes.len() {
        let remain = bytes.len() - idx;
        let mut next = 0u64;

        if remain >= 8 {
            next = u64::from_le_bytes(bytes[idx..idx + 8].try_into().unwrap());
        } else {
            let mut tmp = [0u8; 8];
            tmp[..remain].copy_from_slice(&bytes[idx..]);
            next = u64::from_le_bytes(tmp);
        }

        fr_elements.push(Fr::from(next));
        idx += 8;
    }

    fr_elements
}

#[cfg(test)]
mod conversion_tests {
    use super::*;

    #[test]
    fn test_bytes_to_fr() {
        let bytes = vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
        let fr_elements = bytes_to_fr(&bytes);
        assert_eq!(fr_elements.len(), 1);
    }

    #[test]
    fn test_partial_bytes() {
        let bytes = vec![1u8, 2u8, 3u8];
        let fr_elements = bytes_to_fr(&bytes);
        assert_eq!(fr_elements.len(), 1);
    }
}