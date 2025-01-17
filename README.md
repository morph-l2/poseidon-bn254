

cd sp1-tests/test-bn254/program

cargo prove build

cd ../script

(rustup override set 1.81.0)

cargo run --release > result.txt