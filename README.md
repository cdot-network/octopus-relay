# Octopus Relay

# Quick Start

To run this project locally:

1. Prerequisites: Make sure you have Node.js ≥ 12 installed (https://nodejs.org), then use it to install [yarn]: `npm install --global yarn` (or just `npm i -g yarn`)
2. Install dependencies: `yarn install` (or just `yarn`)
3. Run the local development server: `yarn dev` (see `package.json` for a
   full list of `scripts` you can run with `yarn`)

Now you'll have a local development environment backed by the NEAR TestNet! Running `yarn dev` will tell you the URL you can visit in your browser to see the app.

## Development

`yarn dev`

## Build Web

`yarn build:web`

## Build Contract

`yarn build:contract:debug`

## Evironment Variables

OCT_NETWORK (default is `testnet`)

OCT_RELAY_CONTRACT_NAME (default is `dev-1617174393394-9371007` or `neardev/dev-account`)

OCT_TOKEN_CONTRACT_NAME (default is `dev-1616962983544-1322706`)

## Usage

### Init

Everytime after deployed, you need run:

**new octopus-ralay**

```
near call octopus-ralay new '{"owner": "madtest.testnet", "appchain_minium_validators": 2, "minium_staking_amount": 100}' --accountId your_id
```

**storage deposit**

```
near call oct_token storage_deposit  '{"account_id": "octopus-ralay"}' --accountId your_id --amount 0.1
```

### Using

**register_appchain**

Note that there can be no spaces after the comma.

```
near call oct_token ft_transfer_call '{"receiver_id": "octopus-ralay", "amount": "1000", "msg": "register_appchain,madchain,http://xasx.com,scsadvdfbfvervdsfvdfs"}' --accountId your_id --amount 0.000000000000000000000001
```

**using wrong function name, will be refunded**

```
near call oct_token ft_transfer_call '{"receiver_id": "octopus-ralay", "amount": "1000", "msg": "test_wrong_function,testchain,http://test.com,testtesttest"}' --accountId your_id --amount 0.000000000000000000000001
```
