
# Art Marketplace Smart Contract

## Overview

This project is a smart contract for an art marketplace implemented using the Internet Computer (IC) framework. It allows for creating artist profiles, minting artworks, minting NFTs, and handling transactions. The contract also includes query functions to retrieve information about artists, artworks, NFTs, and transactions.

## Structures

### Artist

```rust
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Artist {
    id: u64,
    name: String,
    wallet_address: String,
    email: String,
    created_at: u64,
}
```

### Artwork

```rust
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Artwork {
    id: u64,
    artist_id: u64,
    title: String,
    description: String,
    image_url: String,
    created_at: u64,
}
```

### NFT

```rust
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct NFT {
    id: u64,
    artwork_id: u64,
    owner_ids: Vec<u64>, // Allow multiple owners for fractional ownership
    price: u64,
    created_at: u64,
}
```

### Transaction

```rust
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Transaction {
    id: u64,
    nft_id: u64,
    buyer_id: u64,
    seller_id: u64,
    price: u64,
    created_at: u64,
}
```

## Payload Definitions

### ArtistPayload

```rust
#[derive(candid::CandidType, Deserialize, Serialize)]
struct ArtistPayload {
    name: String,
    wallet_address: String,
    email: String,
}
```

**Sample Request**

```json
{
    "name": "John Doe",
    "wallet_address": "0x1234567890abcdef",
    "email": "johndoe@example.com"
}
```

### ArtworkPayload

```rust
#[derive(candid::CandidType, Deserialize, Serialize)]
struct ArtworkPayload {
    artist_id: u64,
    title: String,
    description: String,
    image_url: String,
}
```

**Sample Request**

```json
{
    "artist_id": 1,
    "title": "Sunset Over Mountains",
    "description": "A beautiful sunset over the mountains.",
    "image_url": "https://example.com/sunset.jpg"
}
```

### NFTPayload

```rust
#[derive(candid::CandidType, Deserialize, Serialize)]
struct NFTPayload {
    artwork_id: u64,
    owner_ids: Vec<u64>,
    price: u64,
}
```

**Sample Request**

```json
{
    "artwork_id": 1,
    "owner_ids": [1, 2],
    "price": 1000
}
```

### TransactionPayload

```rust
#[derive(candid::CandidType, Deserialize, Serialize)]
struct TransactionPayload {
    nft_id: u64,
    buyer_id: u64,
    seller_id: u64,
    price: u64,
}
```

**Sample Request**

```json
{
    "nft_id": 1,
    "buyer_id": 2,
    "seller_id": 1,
    "price": 1000
}
```

## Functions

### Create Artist Profile

```rust
#[ic_cdk::update]
fn create_artist_profile(payload: ArtistPayload) -> Result<Artist, Error> { ... }
```

### Create Artwork

```rust
#[ic_cdk::update]
fn mint_artwork(payload: ArtworkPayload) -> Result<Artwork, Error> { ... }
```

### Mint NFT

```rust
#[ic_cdk::update]
fn mint_nft(payload: NFTPayload) -> Result<NFT, Error> { ... }
```

### Buy NFT

```rust
#[ic_cdk::update]
fn buy_nft(payload: TransactionPayload) -> Result<Transaction, Error> { ... }
```

## Query Functions

### Get Artist

```rust
#[ic_cdk::query]
fn get_artist(artist_id: u64) -> Result<Artist, String> { ... }
```

### Get Artwork

```rust
#[ic_cdk::query]
fn get_artwork(artwork_id: u64) -> Result<Artwork, String> { ... }
```

### Get NFT

```rust
#[ic_cdk::query]
fn get_nft(nft_id: u64) -> Result<NFT, String> { ... }
```

### Get Transaction

```rust
#[ic_cdk::query]
fn get_transaction(transaction_id: u64) -> Result<Transaction, String> { ... }
```

### Get All Artists

```rust
#[ic_cdk::query]
fn get_all_artists() -> Result<Vec<Artist>, Error> { ... }
```

### Get All Artworks

```rust
#[ic_cdk::query]
fn get_all_artworks() -> Result<Vec<Artwork>, Error> { ... }
```

### Get All NFTs

```rust
#[ic_cdk::query]
fn get_all_nfts() -> Result<Vec<NFT>, Error> { ... }
```

### Get All Transactions

```rust
#[ic_cdk::query]
fn get_all_transactions() -> Result<Vec<Transaction>, Error> { ... }
```

## Error Handling

The contract defines the following errors:

```rust
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
    AlreadyExists { msg: String },
    Unauthorized { msg: String },
}
```


## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```