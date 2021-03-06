<a href="https://proteus.finance/" title="Proteus Finance"><img align="center" src="../../assets/logo_with_text.svg" height="150" alt="Logo" /></a>
<br />


# Proteus Token


This is cw 20 token Proteus Project. In which you can mint, transfer, and burn tokens.
This project also have different more functionalties in which seed, Liquidity, Staking, Launch Pad, Advisor, IDO, Insurance and team we will explain these functionalties and also how they work

<br />


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
cargo wasm
```

- thats command will build the project and make target folder in your project.

```sh
cargo run-script optimize
```

- this command will genrate artifact file in your folder and this folder have cw20.wasm file

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
                     
Initate Json

```sh
{
  "name": "Proteus Token",
  "symbol": "PROTEUS",
  "supply_limit": "1000000",
  "decimals": 6,
  "marketing": {
    "marketing": "terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc"
  },
  "minter": {
    "minter": "terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc",
    "cap": "1000000"
  },
  "initial_balances": [
    {
      "amount": "0",
      "address": "terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc"
    }
  ]
}
```


## Seed

- Seed function **_have 9_** percent of token supply
- Seed function will take one parameter  invetsors object array 
- Seed will directly start after deployment.
- only owner can use this function.
- Seed function will not work after 18 months of token deployment

Seed Json

```sh          
{
  "seed": {
    "investors": [
      {
        "amount": "10,
        "investor": "terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc",
      }
    ]
  }
}
```


## Public

- IDO function **_have 4_** percent of token supply.
- IDO function will take one parameter  recipient  address thats will be in String.
- IDO function will work from 6 to 8 momth .
            
Public Json

```sh             
{
    "public":{
        "amount":"10",
        "recipient":"terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc"
    }
}
```

## Staking Fund

- Staking function **_have 25_** percent of token supply.
- Staking function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String.

Staking Json

```sh             
{
    "staking":{
        "amount":"10",
        "recipient":"terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc"
    }
}
```

## Team

- Team function **_have 17_** percent of token supply.
- Team function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String.
- Every month only **10 percent amount** of 20 percent of token can be through teaming.
- Team function will not work after 23 months of token deployment.
- Team function only will work after the six months of deployment of token.

Team Json

```sh             
{
    "team":{
        "amount":"10",
        "recipient":"terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc"
    }
}
```

## Advisor

- Advisor function **_have 2_** percent of token supply.
- Advisor function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String.
- Every month only **10 percent amount** of 2 percent of token can be through advising.
- Advisor function will not work after 13 months of token deployment.
- Advisor function only will work after the six months of deployment of token.
    
Advisor Json

```sh             
{
    "advisor":{
        "amount":"10",
        "recipient":"terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc"
    }
}
```                     

## Community

- Launchpad function **_have 17_** percent of token supply.
- Launchpad function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String.
- Every month only **10 percent amount** of 15 percent of token can be through advising.
- Launchpad function will not work after 16 months of token deployment.
- Launchpad function only will work after the six months of deployment of token.
                    
Community Json

```sh             
{
    "community":{
        "amount":"10",
        "recipient":"terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc"
    }
}
```                    

## Liquidity

- Liquidity function **_have 6_** percent of token supply.
- Liquidity function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String

Liquidity Json
                    
```sh             
{
    "liquidity":{
        "amount":"10",
        "recipient":"terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc"
    }
}
```  

## Insurance

- Insurance function **_have 2_** percent of token supply.
- Insurance function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String

Insurance Json

```sh             
{
    "insurance":{
        "amount":"10",
        "recipient":"terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc"
    }
}
```                                          
