use anchor_lang::prelude::*;

use crate::AmountU64;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub fee: AmountU64<9>,
    pub collection: Pubkey,
    pub supply: u8,
}
