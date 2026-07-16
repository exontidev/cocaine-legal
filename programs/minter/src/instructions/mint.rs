use crate::Config;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct MintContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [b"mint-authority"], bump)]
    pub mint_authority: SystemAccount<'info>,

    #[account(
        init,
        signer,
        payer = payer,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub config: Account<'info, Config>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}
