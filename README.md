# pinocchio-counter

This is a demonstration of a Solana program and CLI for simple counter state on-chain.

## Setup

```
mkdir credentials
solana-keygen new -o credentials/counter.json
```

## Compiling

```
cd program 
cargo build-bpf
cd ..
```

```
cd cli
cargo build
cd ..
```

## Deploying Locally

```
cd /tmp
solana-test-validator --reset
```

```
cd -
solana program deploy target/sbf-solana-solana/release/pinocchio_counter_sbf.so --program-id credentials/counter.json
```

## Testing Against Local Validator

```
cd cli
cargo run init
# Check the PDA address in the transaction in Solana explorer against your local
# validator, configured in Custom RPC URL
cargo run increment
```

Check the account data with
```
solana address $PDA_PUBKEY
```
