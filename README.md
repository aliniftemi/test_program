# test_program
Simple Solana program

# Generate program keypair 
solana-keygen new --outfile test-program-keypair.json

# Get program address
solana address --keypair test-program-keypair.json

# Install Solana client
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Cargo
sudo apt-get update
sudo apt install cargo

# Install some dependencies (if not alredy present)
sudo apt-get update
sudo apt-get install pkg-config libssl-dev

# Traditional program build to ensure is buildable, remove target build after success building
cargo build-sbf
rm -rf target/
rm Cargo.lock

# Install solana verify
cargo install solana-verify

# Build program with solana verify tool
rustup override set 1.68.0
cargo generate-lockfile

solana-verify build

# Deploy
solana program deploy --url https://api.mainnet-beta.solana.com target/deploy/test_program.so --program-id 95UyNHWjir7Jyjr4hSr7rCRZWxq9dfy6NGfbtmuFPvha --with-compute-unit-price 50000 --max-sign-attempts 100 --use-rpc