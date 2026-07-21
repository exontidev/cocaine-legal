use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ForwardStore {
    #[max_len(16)]
    pub encrypted_prefix: String,
    pub owner: Pubkey,
}
