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
    use anchor_lang::solana_program::program_option::COption;
    use anchor_spl::{
        token_2022::{spl_token_2022::instruction::AuthorityType, MintTo, SetAuthority},
        token_interface,
    };

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

        Ok(())
    }

    pub fn mint(ctx: Context<MintContext>) -> Result<()> {
        let bump = ctx.bumps.mint_authority;
        let signer_seeds: &[&[&[u8]]] = &[&[b"mint-authority", &[bump]]];

        let accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(ctx.accounts.token_program.key(), accounts, signer_seeds);

        token_interface::mint_to(cpi_ctx, 1)?;

        let cpi_accounts = SetAuthority {
            current_authority: ctx.accounts.mint_authority.to_account_info(),
            account_or_mint: ctx.accounts.mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            cpi_accounts,
            signer_seeds,
        );

        token_interface::set_authority(cpi_ctx, AuthorityType::MintTokens, None)?;

        ctx.accounts.token_account.reload();
        ctx.accounts.mint.reload();

        msg!(
            "Current mint authority on {} is {:?}",
            ctx.accounts.mint.key(),
            ctx.accounts.mint.mint_authority
        );

        msg!("token_account {:?}", ctx.accounts.token_account.state);
        Ok(())
    }
}
