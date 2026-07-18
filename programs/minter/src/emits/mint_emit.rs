use anchor_lang::prelude::*;

#[event]
pub struct MintEvent {
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub owner: Pubkey,
    pub prefix: String,
}
