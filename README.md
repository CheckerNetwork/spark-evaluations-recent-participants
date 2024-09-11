# spark-evaluations: Recent Participants

Contract deployed at `0xcb3e3291a298a44224bc3bafd04957e9feed5767`.

## API

### constructor (address owner)

### .get() public view (uint[])

Get all recent participants.

### .set(uint date, uint[] participants) public

Onwer only. Set participants for given date (0-30).

## Development

This repo requires Rust and Cargo, which can be installed from
[here](https://doc.rust-lang.org/book/ch01-01-installation.html)

##### Install Foundry

We recommend you install it from source:

```bash
git clone https://github.com/foundry-rs/foundry
cd foundry
git checkout 9a4bb7f5
# install cast + forge
cargo install --path ./cli --profile local --bins --locked --force
# install anvil
cargo install --path ./anvil --profile local --locked --force
```

##### Clone Repo and Install

```bash
git clone https://github.com/filecoin-station/spark-impact-evaluator.git
cd spark-impact-evaluator
git submodule update --init --recursive
forge test
```

## Deployment

The deployment relies on contract bindings generated in the `/contract-bindings`
directory. If you make changes to the contracts, run:

```bash
rm -rf contract-bindings
forge bind  --crate-name contract-bindings -b ./contract-bindings
```

This will create new bindings with the modified contracts.

Deployment can then proceed either with a locally stored mnemonic or a connected
ethereum ledger wallet. To use with a mnemonic, create a `secrets/mnemonic` file
in the root directory.

To deploy, run:

```bash
(cd contract-utils && cargo run)
```

## Tests

#### Integration Tests

Integration tests run on the filecoin calibration net and require a wallet with
test FIL to pay for gas fees on the calibration net. Test FIL is free and can be
obtained using the [faucet](https://faucet.calibration.fildev.network/).

Before running integration tests, these env vars are required:

```bash
export TEST_RPC_URL=https://api.calibration.node.glif.io/rpc/v1
export TEST_MNEMONIC={insert wallet mnemonic here}
export TEST_CONTRACT_ADDRESS={this can be an empty string}
```

To run tests, run:

```bash
cd contract-utils
cargo test  -- --nocapture --test-threads 1
```
