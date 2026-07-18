use anchor_lang::prelude::*;

use crate::{constants::*, state::config::Config};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [FEE_VAULT_SEED],
        bump
    )]
    pub fee_vault: SystemAccount<'info>,
    pub fee_authority: SystemAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

// pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
//     ctx.accounts.counter.count = 0;
//     ctx.accounts.counter.authority = ctx.accounts.payer.key();

//     let cpi_accounts = anchor_lang::system_program::Transfer {
//         from: ctx.accounts.payer.to_account_info(),
//         to: ctx.accounts.counter.to_account_info(),
//     };
//     let cpi_ctx = CpiContext::new(anchor_lang::system_program::ID, cpi_accounts);
//     anchor_lang::system_program::transfer(cpi_ctx, HELLO_WORLD_LAMPORTS)?;

//     msg!("Hello, world! Counter initialized");
//     Ok(())
// }
