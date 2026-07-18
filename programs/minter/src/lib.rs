pub mod constants;
pub mod error;
pub mod helper;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use helper::*;
pub use instructions::*;
pub use state::*;

declare_id!("FfHBH4w4ubu9GWDHcF4pzEA3VKDobQMYZq2JSCKZdQQz");

#[program]
pub mod minter {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        fee: AmountU64<9>,
        collection: Pubkey,
        supply: u8,
    ) -> Result<()> {
        ctx.accounts.config.supply = supply;
        ctx.accounts.config.collection = collection;
        ctx.accounts.config.fee = fee;
        ctx.accounts.config.fee_authority = *ctx.accounts.fee_authority.key;

        Ok(())
    }

    pub fn mint(ctx: Context<MintContext>) -> Result<()> {
        mint::deduct_fee(&ctx)?;
        mint::mint_nft(&ctx)?;
        mint::revoke_mint_authority(&ctx)?;

        Ok(())
    }
}
