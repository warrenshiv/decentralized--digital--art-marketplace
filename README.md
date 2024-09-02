# Book Swap Platform

This project is a decentralized platform built on the Internet Computer, aiming to facilitate the exchange of books among users. It leverages the power of the blockchain to ensure transparency and reliability in the swapping process.

## Key Features

1. **User Management**
   - **Create User Profile:** Allows the creation of new user profiles with validation for input fields.
   - **Get User Profile:** Retrieves the profile of a user by their unique ID.
   - **Update User Profile:** Updates the existing user profile with new information.
   - **Get All Users:** Retrieves all registered user profiles.

2. **Book Management**
   - **Create Book:** Allows a user to register a new book for swapping.
   - **Get Book:** Retrieves the details of a specific book by its ID.
   - **Get Books by User ID:** Lists all books registered by a specific user.
   - **Get Books by Title:** Retrieves books filtered by their title.

3. **Swap Management**
   - **Create Swap Request:** Initiates a swap request for a book.
   - **Get Swap Requests by User ID:** Retrieves all swap requests initiated by a specific user.
   - **Get Swap Request Details:** Provides details of a specific swap request.

4. **Feedback Management**
   - **Create Feedback:** Allows users to leave feedback on a completed swap.
   - **Get Feedbacks by User ID:** Retrieves all feedback associated with a specific user.

5. **Error Handling**
   - **Not Found:** Returns an error if a requested resource (user, book, swap request) is not found.
   - **Invalid Input:** Handles errors related to invalid email, phone number, or missing required fields.


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