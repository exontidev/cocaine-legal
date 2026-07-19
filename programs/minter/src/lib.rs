pub mod constants;
pub mod emits;
pub mod error;
pub mod helper;
pub mod instructions;
pub mod metaplex;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use emits::*;
pub use helper::*;
pub use instructions::*;
pub use metaplex::*;
pub use state::*;

declare_id!("3VkR6Q98H8tSCrC28GT8ePJcWYHkhvmM1rFo8BhcZc9E");

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

    pub fn mint(ctx: Context<MintContext>, prefix: String) -> Result<()> {
        mint::deduct_fee(&ctx)?;
        mint::mint_nft(&ctx)?;
        mint::create_metadata(&ctx, &prefix)?;
        mint::revoke_mint_authority(&ctx)?;

        emit!(MintEvent {
            mint: ctx.accounts.mint.key(),
            token_account: ctx.accounts.token_account.key(),
            owner: *ctx.accounts.payer.key,
            prefix
        });

        Ok(())
    }
}
