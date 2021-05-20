#!/bin/bash
# Run from the root

solana airdrop 10 -u d -k ~/.solana/dev-keypair.json
cargo build-bpf

#$(solana-test-validator -u d &) || "Already running"

echo "deploying $PWD/target/deploy/contract_pam.so" 
solana -k ~/.solana/dev-keypair.json program deploy "$PWD/target/deploy/contract_pam.so" -u d > program_id.json
