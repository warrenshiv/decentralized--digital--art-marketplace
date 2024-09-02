#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use regex::Regex;
use std::borrow::Cow;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Artist Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Artist {
    id: u64,
    name: String,
    wallet_address: String,
    email: String,
    created_at: u64,
}

// Artwork Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Artwork {
    id: u64,
    artist_id: u64,
    title: String,
    description: String,
    image_url: String,
    created_at: u64,
}

// NFT Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct NFT {
    id: u64,
    artwork_id: u64,
    owner_ids: Vec<u64>, // Allow multiple owners for fractional ownership
    price: u64,
    status: String, // Pending, Completed, Cancelled
    created_at: u64,
}

// Transaction Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Transaction {
    id: u64,
    nft_id: u64,
    buyer_id: u64,
    seller_id: u64,
    price: u64,
    created_at: u64,
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static ARTISTS_STORAGE: RefCell<StableBTreeMap<u64, Artist, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static ARTWORKS_STORAGE: RefCell<StableBTreeMap<u64, Artwork, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );

    static NFTS_STORAGE: RefCell<StableBTreeMap<u64, NFT, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static TRANSACTIONS_STORAGE: RefCell<StableBTreeMap<u64, Transaction, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );
}

impl Storable for Artist {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Artist {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Artwork {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Artwork {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for NFT {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for NFT {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Transaction {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Transaction {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Payloads Definitions

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ArtistPayload {
    name: String,
    wallet_address: String,
    email: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ArtworkPayload {
    artist_id: u64,
    title: String,
    description: String,
    image_url: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct NFTPayload {
    artwork_id: u64,
    owner_ids: Vec<u64>,
    price: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct TransactionPayload {
    nft_id: u64,
    buyer_id: u64,
    seller_id: u64,
    price: u64,
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
    AlreadyExists { msg: String },
    Unauthorized { msg: String },
}

// Functions

// Create Artist Profile
#[ic_cdk::update]
fn create_artist_profile(payload: ArtistPayload) -> Result<Artist, Error> {
    if payload.name.is_empty() || payload.wallet_address.is_empty() || payload.email.is_empty() {
        return Err(Error::InvalidInput {
            msg: "All fields are required".to_string(),
        });
    }

    // Validate the email address
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(&payload.email) {
        return Err(Error::InvalidInput {
            msg: "Ensure the email address is of the correct format".to_string(),
        });
    }

    // Ensure email address uniqueness
    let email_exists = ARTISTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, artist)| artist.email == payload.email)
    });

    if email_exists {
        return Err(Error::AlreadyExists {
            msg: "Email already exists".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let artist = Artist {
        id,
        name: payload.name,
        wallet_address: payload.wallet_address,
        email: payload.email,
        created_at: time(),
    };

    ARTISTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, artist.clone()));

    Ok(artist)
}

// Create Artwork
#[ic_cdk::update]
fn mint_artwork(payload: ArtworkPayload) -> Result<Artwork, Error> {
    if payload.title.is_empty() || payload.image_url.is_empty() {
        return Err(Error::InvalidInput {
            msg: "All fields are required".to_string(),
        });
    }

    let artist_exists =
        ARTISTS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.artist_id));
    if !artist_exists {
        return Err(Error::NotFound {
            msg: "Artist does not exist".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let artwork = Artwork {
        id,
        artist_id: payload.artist_id,
        title: payload.title,
        description: payload.description,
        image_url: payload.image_url,
        created_at: time(),
    };

    ARTWORKS_STORAGE.with(|storage| storage.borrow_mut().insert(id, artwork.clone()));

    Ok(artwork)
}

// Mint NFT
#[ic_cdk::update]
fn mint_nft(payload: NFTPayload) -> Result<NFT, Error> {
    if payload.owner_ids.is_empty() || payload.price == 0 {
        return Err(Error::InvalidInput {
            msg: "All fields are required".to_string(),
        });
    }

    let artwork_exists =
        ARTWORKS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.artwork_id));
    if !artwork_exists {
        return Err(Error::NotFound {
            msg: "Artwork does not exist".to_string(),
        });
    }

    // Ensure all owner ids exist
    let owner_exists = ARTISTS_STORAGE.with(|storage| {
        payload
            .owner_ids
            .iter()
            .all(|owner_id| storage.borrow().contains_key(owner_id))
    });

    if !owner_exists {
        return Err(Error::NotFound {
            msg: "One or more owner ids do not exist".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let nft = NFT {
        id,
        artwork_id: payload.artwork_id,
        owner_ids: payload.owner_ids,
        price: payload.price,
        status: "Pending".to_string(),
        created_at: time(),
    };

    NFTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, nft.clone()));

    Ok(nft)
}

// Buy NFT
#[ic_cdk::update]
fn buy_nft(payload: TransactionPayload) -> Result<Transaction, Error> {
    let nft_exists = NFTS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.nft_id));
    if !nft_exists {
        return Err(Error::NotFound {
            msg: "NFT does not exist".to_string(),
        });
    }

    let buyer_exists =
        ARTISTS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.buyer_id));
    if !buyer_exists {
        return Err(Error::NotFound {
            msg: "Buyer does not exist".to_string(),
        });
    }

    let seller_exists =
        ARTISTS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.seller_id));
    if !seller_exists {
        return Err(Error::NotFound {
            msg: "Seller does not exist".to_string(),
        });
    }

    // Ensure the buyer is not the seller
    if payload.buyer_id == payload.seller_id {
        return Err(Error::InvalidInput {
            msg: "Buyer and seller cannot be the same".to_string(),
        });
    }

    // Ensure the price is same as the NFT price
    let nft_price = NFTS_STORAGE.with(|storage| {
        let nft = storage.borrow().get(&payload.nft_id).unwrap();
        nft.price
    });

    if nft_price != payload.price {
        return Err(Error::InvalidInput {
            msg: "Price does not match the NFT price".to_string(),
        });
    }

    // Ensure the status of the NFT is Pending
    let nft_status = NFTS_STORAGE.with(|storage| {
        let nft = storage.borrow().get(&payload.nft_id).unwrap();
        nft.status.clone()
    });

    if nft_status != "Pending" {
        return Err(Error::InvalidInput {
            msg: "NFT is not available for sale".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let transaction = Transaction {
        id,
        nft_id: payload.nft_id,
        buyer_id: payload.buyer_id,
        seller_id: payload.seller_id,
        price: payload.price,
        created_at: time(),
    };

    // Update NFT owner
    NFTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(nft) = storage.get(&payload.nft_id) {
            let mut nft = nft.clone();
            nft.owner_ids.push(payload.buyer_id);
            storage.insert(payload.nft_id, nft);
        }
    });

    // Update NFT status
    NFTS_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(nft) = storage.get(&payload.nft_id) {
            let mut nft = nft.clone();
            nft.status = "Completed".to_string();
            storage.insert(payload.nft_id, nft);
        }
    });

    TRANSACTIONS_STORAGE.with(|storage| storage.borrow_mut().insert(id, transaction.clone()));

    Ok(transaction)
}

// Query Functions

#[ic_cdk::query]
fn get_artist(artist_id: u64) -> Result<Artist, String> {
    let artist = ARTISTS_STORAGE.with(|storage| storage.borrow().get(&artist_id));
    match artist {
        Some(artist) => Ok(artist.clone()),
        None => Err("Artist does not exist".to_string()),
    }
}

#[ic_cdk::query]
fn get_artwork(artwork_id: u64) -> Result<Artwork, String> {
    let artwork = ARTWORKS_STORAGE.with(|storage| storage.borrow().get(&artwork_id));
    match artwork {
        Some(artwork) => Ok(artwork.clone()),
        None => Err("Artwork does not exist".to_string()),
    }
}

#[ic_cdk::query]
fn get_nft(nft_id: u64) -> Result<NFT, String> {
    let nft = NFTS_STORAGE.with(|storage| storage.borrow().get(&nft_id));
    match nft {
        Some(nft) => Ok(nft.clone()),
        None => Err("NFT does not exist".to_string()),
    }
}

#[ic_cdk::query]
fn get_transaction(transaction_id: u64) -> Result<Transaction, String> {
    let transaction = TRANSACTIONS_STORAGE.with(|storage| storage.borrow().get(&transaction_id));
    match transaction {
        Some(transaction) => Ok(transaction.clone()),
        None => Err("Transaction does not exist".to_string()),
    }
}

#[ic_cdk::query]
fn get_all_artists() -> Result<Vec<Artist>, Error> {
    ARTISTS_STORAGE.with(|storage| {
        let records: Vec<Artist> = storage
            .borrow()
            .iter()
            .map(|(_, artist)| artist.clone())
            .collect();
        if records.is_empty() {
            return Err(Error::NotFound {
                msg: "No artists found".to_string(),
            });
        }
        Ok(records)
    })
}

#[ic_cdk::query]
fn get_all_artworks() -> Result<Vec<Artwork>, Error> {
    ARTWORKS_STORAGE.with(|storage| {
        let records: Vec<Artwork> = storage
            .borrow()
            .iter()
            .map(|(_, artwork)| artwork.clone())
            .collect();
        if records.is_empty() {
            return Err(Error::NotFound {
                msg: "No artworks found".to_string(),
            });
        }
        Ok(records)
    })
}

#[ic_cdk::query]
fn get_all_nfts() -> Result<Vec<NFT>, Error> {
    NFTS_STORAGE.with(|storage| {
        let records: Vec<NFT> = storage
            .borrow()
            .iter()
            .map(|(_, nft)| nft.clone())
            .collect();
        if records.is_empty() {
            return Err(Error::NotFound {
                msg: "No NFTs found".to_string(),
            });
        }
        Ok(records)
    })
}

#[ic_cdk::query]
fn get_all_transactions() -> Result<Vec<Transaction>, Error> {
    TRANSACTIONS_STORAGE.with(|storage| {
        let records: Vec<Transaction> = storage
            .borrow()
            .iter()
            .map(|(_, transaction)| transaction.clone())
            .collect();
        if records.is_empty() {
            return Err(Error::NotFound {
                msg: "No transactions found".to_string(),
            });
        }
        Ok(records)
    })
}

// need this to generate candid
ic_cdk::export_candid!();
