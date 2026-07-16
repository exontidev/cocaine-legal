use anchor_lang::{AnchorDeserialize, AnchorSerialize, InitSpace};

#[derive(AnchorSerialize, InitSpace, AnchorDeserialize, Clone, Copy)]
pub struct AmountU64<const DECIMALS: u8>(pub u64);
