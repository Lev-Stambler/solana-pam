#!/bin/bash
# Run from the root

cargo build-bpf
solana airdrop 10 -u d -k ~/.config/solana/id.json

#$(solana-test-validator -u d &) || "Already running"

echo "deploying $PWD/target/deploy/contract_pam.so" 
solana -k ~/.config/solana/id.json program deploy "$PWD/target/deploy/contract_pam_2.so" -u d > program_id.json
