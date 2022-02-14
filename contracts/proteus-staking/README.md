<a href="https://proteus.finance/" title="Proteus Finance"><img align="center" src="../assets/logo_with_text.svg" height="150" alt="Logo" /></a>
<br />

# Proteus Staking Contract.
in proteus staking smart contract do staking of proteus token. in which stker can stake his protesus token but he need to pay bridging and staking fee according to the tire and staker also can lock his staking token for specific time so then he can gain more reward.

## Enviorment Setup

```sh
git clone --depth 1 https://github.com/terra-money/localterra
```

so first clone the repo for setting terra

```sh
cd localterra
```

- go to the terra folder

```sh
docker-compose up
```

- run the docker in your system by above command
- and follow the blow commands step by step

```sh
rustup default stable
rustup target add wasm32-unknown-unknown
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

## Build Project

- now we will see the steps to build the projecct so you need to follow bellow steps to compile the code

```sh
cargo build
```

- thats command will build the project and make target folder in your project.

```sh
cargo run-script optimize
```
- if this command not genrate for you proteusstaking.wasm then rusn this command give below 

```sh
docker run --rm -v "$(pwd)":/code   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/rust-optimizer:0.12.3`sh
```


- this command will genrate artifact file in your folder and this folder have proteusstaking.wasm file

## Deploy Project on Terra

- now the time to deploy the smart contract on terra .
- so open the terra station
- https://station.terra.money/contract
- upload your wasm on terra station
- open the transaction on terra explorer
- copy the code etc "3240"
- then come back to terrastation and click to instantiate
- enter code and init msg this init msg will be in json.
- and confirm the trnasaction
- so now you deployed succesfully smart contract on terra blockchain

---