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
solana program deploy \
    --keypair ../test-program-deloyer-keypair.json \
    --url https://api.mainnet-beta.solana.com \
    --program-id ../test-program-keypair.json \
    --with-compute-unit-price 50000 \
    --max-sign-attempts 100 \
    --use-rpc \
    target/deploy/test_program.so

# Get program hash
solana-verify get-program-hash \
    --url https://api.mainnet-beta.solana.com \
    95UyNHWjir7Jyjr4hSr7rCRZWxq9dfy6NGfbtmuFPvha

Expect a hash like this one: 1410d9a565458093452a6258196f5a23b423766e2e89e0deda122bf0e609b758

# Deploy verification - test-program-deloyer-keypair.json must be availabe in project root to work
solana-verify verify-from-repo \
    --url https://api.mainnet-beta.solana.com \
    --program-id 95UyNHWjir7Jyjr4hSr7rCRZWxq9dfy6NGfbtmuFPvha https://github.com/aliniftemi/test_program \
    --commit-hash 1410954411bca779268f413acb46a9bab44b1a32 \
    --library-name test_program \
    --mount-path .

# Verify
solana-verify verify-from-repo --remote -um --program-id 95UyNHWjir7Jyjr4hSr7rCRZWxq9dfy6NGfbtmuFPvha https://github.com/aliniftemi/test_program
