# pinocchio-counter

This is a demonstration of a Solana program and CLI for simple counter state on-chain.

## Setup

A hardcoded program id keypair exists at `credentials/counter.json`. This keypair is public
and should not be used for anything other than local testing of this program.

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
