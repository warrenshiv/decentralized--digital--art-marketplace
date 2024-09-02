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

// SwapStatus Enum
#[derive(candid::CandidType, Deserialize, Serialize, Clone)]
enum SwapStatus {
    Pending,
    Accepted,
    Rejected,
}

// User Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
    phone_number: String,
    email: String,
    created_at: u64,
}

// Book Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Book {
    id: u64,
    user_id: u64,
    title: String,
    author: String,
    description: String,
    created_at: u64,
}

// SwapRequest Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct SwapRequest {
    id: u64,
    book_id: u64,
    requested_by_id: u64,
    status: SwapStatus,
    created_at: u64,
}

// Feedback Struct
#[derive(candid::CandidType, Serialize, Deserialize, Clone)]
struct Feedback {
    id: u64,
    user_id: u64,
    swap_request_id: u64,
    rating: u8,
    comment: String,
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

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static BOOKS_STORAGE: RefCell<StableBTreeMap<u64, Book, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );

    static SWAP_REQUESTS_STORAGE: RefCell<StableBTreeMap<u64, SwapRequest, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static FEEDBACK_STORAGE: RefCell<StableBTreeMap<u64, Feedback, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Book {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Book {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for SwapRequest {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for SwapRequest {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Feedback {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Feedback {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Payloads Definitions

// User Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    name: String,
    phone_number: String,
    email: String,
}

// Book Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct BookPayload {
    user_id: u64,
    title: String,
    author: String,
    description: String,
}

// SwapRequest Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct SwapRequestPayload {
    book_id: u64,
    requested_by_id: u64,
}

// Feedback Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct FeedbackPayload {
    user_id: u64,
    swap_request_id: u64,
    rating: u8,
    comment: String,
}

// Functions

#[ic_cdk::update]
fn create_user_profile(payload: UserPayload) -> Result<User, Error> {
    // Validate the payload to ensure that the required fields are present
    if payload.name.is_empty() || payload.phone_number.is_empty() || payload.email.is_empty() {
        return Err(Error::EmptyFields {
            msg: "All fields are required".to_string(),
        });
    }

    // Validate the payload to ensure that the email format is correct
    // Validate the email address
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(&payload.email) {
        return Err(Error::InvalidEmail {
            msg: "Ensure the email address is of the correct format".to_string(),
        });
    }

    // Ensure email address uniqueness
    let email_exists = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, user)| user.email == payload.email)
    });

    if email_exists {
        return Err(Error::AlreadyExists {
            msg: "Email already exists".to_string(),
        });
    }

    // Validate the payload to ensure that the phone number format is correct and is 10 digits
    let phone_number_regex = Regex::new(r"^[0-9]{10}$").unwrap();
    if !phone_number_regex.is_match(&payload.phone_number) {
        return Err(Error::InvalidPhoneNumber {
            msg: "Ensure the phone number is of the correct format".to_string(),
        });
    }

    // Generate a new unique ID for the user
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    // Create the user profile
    let user_profile = User {
        id,
        name: payload.name,
        phone_number: payload.phone_number,
        email: payload.email,
        created_at: time(),
    };

    // Store the new user profile in the USERS_STORAGE
    USERS_STORAGE.with(|storage| storage.borrow_mut().insert(id, user_profile.clone()));

    Ok(user_profile)
}

// Function to get a user profile
#[ic_cdk::query]
fn get_user_profile(user_id: u64) -> Result<User, String> {
    // Ensure that the user exists
    let user = USERS_STORAGE.with(|storage| storage.borrow().get(&user_id));
    match user {
        Some(user) => Ok(user.clone()),
        None => Err("User does not exist".to_string()),
    }
}

// Function to update a user profile
#[ic_cdk::update]
fn update_user_profile(user_id: u64, payload: UserPayload) -> Result<User, String> {
    // Ensure that the user exists
    let user = USERS_STORAGE.with(|storage| storage.borrow().get(&user_id));
    match user {
        Some(user) => {
            // Validate the payload to ensure that the required fields are present
            if payload.name.is_empty() || payload.phone_number.is_empty() || payload.email.is_empty() {
                return Err("All fields are required".to_string());
            }

            // Validate the user id to ensure it exists
            if user_id != user.id {
                return Err("User does not exist".to_string());
            }
            
            // Validate the payload to ensure that the email format is correct
            let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
            if !email_regex.is_match(&payload.email) {
                return Err("Ensure the email address is of the correct format".to_string());
            }

            // Ensure email address uniqueness
            let email_exists = USERS_STORAGE.with(|storage| {
                storage
                    .borrow()
                    .iter()
                    .any(|(_, user)| user.email == payload.email)
            });

            if email_exists {
                return Err("Email already exists".to_string());
            }

            // Validate the payload to ensure that the phone number format is correct and is 10 digits
            let phone_number_regex = Regex::new(r"^[0-9]{10}$").unwrap();
            if !phone_number_regex.is_match(&payload.phone_number) {
                return Err("Ensure the phone number is of the correct format".to_string());
            }

            // Update the user profile
            let updated_user = User {
                id: user.id,
                name: payload.name,
                phone_number: payload.phone_number,
                email: payload.email,
                created_at: user.created_at,
            };

            // Store the updated user profile in the USERS_STORAGE
            USERS_STORAGE.with(|storage| storage.borrow_mut().insert(user_id, updated_user.clone()));

            Ok(updated_user)
        }
        None => Err("User does not exist".to_string()),
    }
}

// Function to retrieve all users
#[ic_cdk::query]
fn get_all_users() -> Result<Vec<User>, Error> {
    USERS_STORAGE.with(|storage| {
         let stable_btree_map  = &*storage.borrow();
        
        let records: Vec<User> = stable_btree_map
            .iter()
            .map(|(_, user)| user.clone())
            .collect();
        
        if records.is_empty() {
            return Err(Error::NotFound {
                msg: "No users found".to_string(),
            });
        }
        
        else {
            Ok(records)
        }
    })
}

// Function to create a book
#[ic_cdk::update]
fn create_book(payload: BookPayload) -> Result<Book, String> {
    // Validate the payload to ensure that the required fields are present
    if payload.user_id == 0 || payload.title.is_empty() || payload.author.is_empty() {
        return Err("All fields are required".to_string());
    }

    // Ensure that the user exists
    let user_exists = USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.user_id));
    if !user_exists {
        return Err("User does not exist".to_string());
    }

    // Generate a new unique ID for the book
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    // Create the book
    let book = Book {
        id,
        user_id: payload.user_id,
        title: payload.title,
        author: payload.author,
        description: payload.description,
        created_at: time(),
    };

    // Store the new book in the BOOKS_STORAGE
    BOOKS_STORAGE.with(|storage| storage.borrow_mut().insert(id, book.clone()));

    Ok(book)
}

// Function to get a book by id
#[ic_cdk::query]
fn get_book_id(book_id: u64) -> Result<Book, String> {
    // Ensure that the book exists
    let book = BOOKS_STORAGE.with(|storage| storage.borrow().get(&book_id));
    match book {
        Some(book) => Ok(book.clone()),
        None => Err("Book does not exist".to_string()),
    }
}

// Function to Fetch book by user id
#[ic_cdk::query]
fn get_books_by_user_id(user_id: u64) -> Result<Vec<Book>, String> {
    let books = BOOKS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<Book> = stable_btree_map
            .iter()
            .filter(|(_, book)| book.user_id == user_id)
            .map(|(_, book)| book.clone())
            .collect();
        if records.is_empty() {
            return Err("No books found".to_string());
        }
        Ok(records)
    })?;
    Ok(books)
}

// Function to fetch books by user name
#[ic_cdk::query]
fn get_books_by_user_name(name: String) -> Result<Vec<Book>, String> {
    let books = BOOKS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<Book> = stable_btree_map
            .iter()
            .filter(|(_, book)| {
                let user = USERS_STORAGE.with(|storage| storage.borrow().get(&book.user_id));
                match user {
                    Some(user) => user.name == name,
                    None => false,
                }
            })
            .map(|(_, book)| book.clone())
            .collect();
        if records.is_empty() {
            return Err("No books found for the specified user name".to_string());
        }
        Ok(records)
    })?;
    Ok(books)
}

// Function to fetch books by book title
#[ic_cdk::query]
fn get_books_by_title(title: String) -> Result<Vec<Book>, String> {
    let books = BOOKS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<Book> = stable_btree_map
            .iter()
            .filter(|(_, book)| book.title == title)
            .map(|(_, book)| book.clone())
            .collect();
        if records.is_empty() {
            return Err("No books found".to_string());
        }
        Ok(records)
    })?;
    Ok(books)
}

// Function to retrieve all books
#[ic_cdk::query]
fn get_all_books() -> Result<Vec<Book>, Error> {
    BOOKS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<Book> = stable_btree_map
            .iter()
            .map(|(_, book)| book.clone())
            .collect();
        if records.is_empty() {
            return Err(Error::NotFound {
                msg: "No books found".to_string(),
            });
        }
        Ok(records)
    })
}

// Function to create a swap request
#[ic_cdk::update]
fn create_swap_request(payload: SwapRequestPayload) -> Result<SwapRequest, String> {
    // Validate the payload to ensure that the required fields are present
    if payload.book_id == 0 || payload.requested_by_id == 0 {
        return Err("All fields are required".to_string());
    }

    // Ensure that the book exists
    let book_exists = BOOKS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.book_id));
    if !book_exists {
        return Err("Book does not exist".to_string());
    }

    // Ensure that the user exists
    let user_exists =
        USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.requested_by_id));
    if !user_exists {
        return Err("User does not exist".to_string());
    }

    // Generate a new unique ID for the swap request
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    // Create the swap request and initialize the status to pending
    let swap_request = SwapRequest {
        id,
        book_id: payload.book_id,
        requested_by_id: payload.requested_by_id,
        status: SwapStatus::Pending,
        created_at: time(),
    };

    // Store the new swap request in the SWAP_REQUESTS_STORAGE
    SWAP_REQUESTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, swap_request.clone()));

    Ok(swap_request)
}

// Fetch to get swap requests
#[ic_cdk::query]
fn get_swap_requests(swap_request_id: u64) -> Result<SwapRequest, String> {
    // Ensure that the swap request exists
    let swap_request = SWAP_REQUESTS_STORAGE.with(|storage| storage.borrow().get(&swap_request_id));
    match swap_request {
        Some(swap_request) => Ok(swap_request.clone()),
        None => Err("Swap request does not exist".to_string()),
    }
}

// Function to fetch swap requests by user id
#[ic_cdk::query]
fn get_swap_requests_by_user_id(user_id: u64) -> Result<Vec<SwapRequest>, String> {
    let swap_requests = SWAP_REQUESTS_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<SwapRequest> = stable_btree_map
            .iter()
            .filter(|(_, swap_request)| swap_request.requested_by_id == user_id)
            .map(|(_, swap_request)| swap_request.clone())
            .collect();
        if records.is_empty() {
            return Err("No swap requests found".to_string());
        }
        Ok(records)
    })?;
    Ok(swap_requests)
}

#[ic_cdk::update]
fn create_feedback(payload: FeedbackPayload) -> Result<Feedback, String> {
    // Validate the payload to ensure that the required fields are present
    if payload.user_id == 0 || payload.swap_request_id == 0 || payload.rating == 0 {
        return Err("All fields are required".to_string());
    }

    // Ensure that the user exists
    let user_exists = USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.user_id));
    if !user_exists {
        return Err("User does not exist".to_string());
    }

    // Ensure that the swap request exists
    let swap_request_exists = SWAP_REQUESTS_STORAGE
        .with(|storage| storage.borrow().contains_key(&payload.swap_request_id));
    if !swap_request_exists {
        return Err("Swap request does not exist".to_string());
    }

    // Generate a new unique ID for the feedback
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    // Create the feedback
    let feedback = Feedback {
        id,
        user_id: payload.user_id,
        swap_request_id: payload.swap_request_id,
        rating: payload.rating,
        comment: payload.comment,
        created_at: time(),
    };

    // Store the new feedback in the FEEDBACK_STORAGE
    FEEDBACK_STORAGE.with(|storage| storage.borrow_mut().insert(id, feedback.clone()));
    
    Ok(feedback)
}

// Function to fetch feedbacks for a specific user
#[ic_cdk::query]
fn get_feedbacks_by_user_id(user_id: u64) -> Result<Vec<Feedback>, String> {
    let feedbacks = FEEDBACK_STORAGE.with(|storage| {
        let stable_btree_map = &*storage.borrow();
        let records: Vec<Feedback> = stable_btree_map
            .iter()
            .filter(|(_, feedback)| feedback.user_id == user_id)
            .map(|(_, feedback)| feedback.clone())
            .collect();
        if records.is_empty() {
            return Err("No feedbacks found".to_string());
        }
        Ok(records)
    })?;
    Ok(feedbacks)
}

// Error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
    InvalidEmail { msg: String },
    AlreadyExists { msg: String },
    InvalidPhoneNumber { msg: String },
    EmptyFields { msg: String },
}

// need this to generate candid
ic_cdk::export_candid!();
