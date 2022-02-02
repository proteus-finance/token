<div align="left">
  <a align="left" href="https://proteus.finance/">
    <img align="left" src="assets/logo.svg" alt="Logo">
  </a>
  <br/>
  <h2 align="left">Proteus Token</h2>
    <p>This is cw 20 token Proteus Project. In which you can mint, transfer, and burn tokens.</p>
    <p>This project also have different more functionalties in which seed,Liquidity,Staking,Launch Pad,Advisor,IDO,Insurance and team we will explain these functionalties and also how they work</p>
</div>



## Seed

* Seed function **_have 8_** percent of token supply
* Seed function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String
* Seed function only will work after the three months of deployment of token
* You can only do **_5 percent_** of seeding every month
* Seed function will not work after 23 months of token deployment

## IDO

* IDO function **_have 4_** percent of token supply.
* IDO function will take two parameter one is amount thats will be in **_Uint128_** and address thats will be in String.
* IDO function will work from 6 to 8 momth .

## Staking Fund

* Staking function **_have 25_** percent of token supply.
* Staking function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String.

## Team

* Team function **_have 20_** percent of token supply.
* Team function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String.
* Every month only **10 percent amount** of 20 percent of token can be through teaming.
* Team function will not work after 23 months of token deployment.
* Team function only will work after the six months of deployment of token.

## Advisor

* Advisor function **_have 2_** percent of token supply.
* Advisor function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String.
* Every month only **10 percent amount** of 2 percent of token can be through advising.
* Advisor function will not work after 13 months of token deployment.
* Advisor function only will work after the six months of deployment of token.

## Launchpad

* Launchpad function **_have 15_** percent of token supply.
* Launchpad function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String.
* Every month only **10 percent amount** of 15 percent of token can be through advising.
* Launchpad function will not work after 16 months of token deployment.
* Launchpad function only will work after the six months of deployment of token.

## Liquidity

* Liquidity function **_have 6_** percent of token supply.
* Liquidity function will take two parameter one is amount thats will be in **_Uint128_** and recipient address thats will be in String

## Enviorment Setup

*  # git clone --depth 1 https://github.com/terra-money/localterra
*  so first clone the repo for setting terra
*  # cd localterra
*  go to the terra folder  
*  # docker-compose up 
*  run the docker in your system by above command

*  and follow the blow commands step by step
*  # rustup default stable
*  # rustup target add wasm32-unknown-unknown
*  # cargo install cargo-generate --features vendored-openssl
*  # cargo install cargo-run-script

## Build Project

* now we will see the steps to build the projecct so you need to follow bellow steps to compile the code
* # cargo wasm.
* thats command will build the project and make target folder in your project.
* # cargo run-script optimize
* this command will genrate artifact file in your folder and this folder have cw20.wasm file

## Deploy Project on Terra

>  now the time to deploy the smart contract on terra .
> so open the terra station
>  https://station.terra.money/contract  
> upload your wam on terra station 
> open the transaction on terra explorer
> copy the code etc "3240"
> then come back to terrastation and click to instantiate 
> enter code and init msg this init msg will be in json.
> and confirm the trnasaction
> so now you deployed succesfully smart contract on terra  blockchain



