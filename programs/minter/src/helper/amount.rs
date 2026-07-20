use anchor_lang::prelude::*;

#[derive(AnchorSerialize, InitSpace, AnchorDeserialize, Clone, Copy)]
pub struct AmountU64<const DECIMALS: u8>(pub u64);

pub type Sol = AmountU64<9>;
