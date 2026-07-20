use crate::{InitializeArgs, Sol};
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub fee: Sol,
    pub fee_collector: Pubkey,

    pub collection: Pubkey,
}

impl Config {
    pub fn from_args(&mut self, args: InitializeArgs) {
        self.fee = args.fee;
        self.fee_collector = args.fee_collector;
        self.collection = args.collection;
    }
}
