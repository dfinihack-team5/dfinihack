# Roadmap Prediction Market

Welcome to the Roadmap Prediction Market Dapp built for the 2021 DFINIHacks Hackathon!

## About

### The Current Problem

Right now, members of the Internet Computer ecosystem have limited tools for signaling their sentiment on roadmap proposals.

Users can view the [Roadmap microsite](https://dfinity.org/roadmap), [comment on the forum](https://forum.dfinity.org/t/direct-integration-with-bitcoin/6147/11), and [vote on proposals](https://dashboard.internetcomputer.org/proposal/18337), but have no way to signal their relative support for different proposals.

### The Proposed Solution
The Roadmap Prediction Market dapp introduces a token-voting based system.

Each roadmap item becomes its own "market" where users can acquire tokens to signal their support for the given roadmap item.

When the market reaches its maturity, the market participants will earn (or lose) rewards based on their positions in that market.

### Features

The Roadmap Prediction Market is (to our knowledge) the first Prediction Market deployed to the IC.

The RPM uses a [logarithmic market scoring rule (LMSR)](https://www.cultivatelabs.com/prediction-markets-guide/how-does-logarithmic-market-scoring-rule-lmsr-work) to price shares in each market. The implementation of this LMSR mechanism can be found within the `cost` function of `src/api/market.rs`.

Markets can be created by the canister deployer.

Unfortunately there is no front end as of yet, but please feel free to implement one!

---

## Developing

### Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:8000?canisterId={asset_canister_id}`.

To deploy to the production Internet Computer network simply specify the IC network to deploy to

```bash
dfx deploy --network ic
```

### Interacting

You can interact with it using `dfx canister call`:

```bash
# Add a profile
dfx canister call roadmap_prediction_market update '(record {name = "Luxi"; description = "mountain dog"; keywords = vec {"scars"; "toast"}})'

# Retrieve your profile
dfx canister call roadmap_prediction_market getSelf
```

This is based on the [Profile tutorial](https://sdk.dfinity.org/docs/rust-guide/rust-profile.html),
using `roadmap_prediction_market` as a name instead of `rust_profile` so most of the commands there
apply (with the changed canister name).

### Adding New Markets
Markets are simple records with a name and description. To initialze a new market make the following canister call:

```bash
dfx canister --network ic call roadmap_prediction_market newMarket '("Badlands", "Badlands is a concept that involves applying Internet Computer technology to create a new network supported by amateur node providers, using low cost devices, that creates the maximum possible level of decentralization and censorship resistance for smart contracts.")'
```

### Getting Markets
You can retreive markets with the `getMarket` call:

```bash
 dfx canister --network ic call roadmap_prediction_market getMarkets
```

Which will return all existing markets:
```
(
  vec {
    record {
      no_price = 0.5 : float64;
      yes_price = 0.5 : float64;
      market = record {
        name = "Badlands";
        yes_shares = 0 : float64;
        description = "Badlands is a concept that involves applying Internet Computer technology to create a new network supported by amateur node providers, using low cost devices, that creates the maximum possible level of decentralization and censorship resistance for smart contracts.";
        no_shares = 0 : float64;
      };
    };
    record {
      no_price = 0.5 : float64;
      yes_price = 0.5 : float64;
      market = record {
        name = "Custom domains for ic0.app";
        yes_shares = 0 : float64;
        description = "Current URLs for the Internet Computer are derived solely from Canister IDs, which are difficult for humans to remember. Discuss implementations and plans for custom domains.";
        no_shares = 0 : float64;
      };
    };
    record {
      no_price = 0.5 : float64;
      yes_price = 0.5 : float64;
      market = record {
        name = "Endorphin";
        yes_shares = 0 : float64;
        description = "Endorphin is a free and open crypto OS for smartphones and other end-user devices. The vision of Endorphin will allow the vast majority of dapps to be built using a combination of HTML, JavaScript, CSS, media, and WebAssembly — just like websites.";
        no_shares = 0 : float64;
      };
    };
  },
)
```

## Front End

Additionally, if you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 8000.

## Issues and workarounds

If you run into this error while running `dfx deploy`:

```
error[E0463]: can't find crate for `core`
  |
  = note: the `wasm32-unknown-unknown` target may not be installed
```

You need to install support for the `wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown
```

### Note on frontend environment variables

If you are hosting frontend code somewhere without using DFX, you may need to make one of the following adjustments to ensure your project does not fetch the root key in production:

- set`NODE_ENV` to `production` if you are using Webpack
- use your own preferred method to replace `process.env.NODE_ENV` in the autogenerated declarations
- Write your own `createActor` constructor
