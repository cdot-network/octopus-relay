Octopus Relay
==========

Quick Start
===========

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

OCT_RELAY_CONTRACT_NAME (default is `dev-1616819063071-1140774` or `neardev/dev-account`)

OCT_TOKEN_CONTRACT_NAME (default is `dev-1615435740118-2637667`)


