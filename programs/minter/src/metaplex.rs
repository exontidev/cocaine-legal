use anchor_lang::prelude::*;

pub mod create_metadata;
pub mod verify_collection;

pub use create_metadata::*;

pub fn mpl_token_metadata_id() -> Pubkey {
    pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
}
