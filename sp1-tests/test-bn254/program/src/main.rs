#![no_main]

sp1_zkvm::entrypoint!(main);
use poseidon_bn254::{hash_with_domain, Fr};

fn main() {
    // test hash_with_domain
    println!(
        "cycle-tracker-start: hash_with_domain(&[Fr::from(1u64), Fr::from(2u64)], Fr::from(3u64))"
    );
    let result2 = hash_with_domain(&[Fr::from(1u64), Fr::from(2u64)], Fr::from(3u64));
    println!("Result: {:?}", result2);
    println!(
        "cycle-tracker-end: hash_with_domain(&[Fr::from(1u64), Fr::from(2u64)], Fr::from(3u64))"
    );
}
