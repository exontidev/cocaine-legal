use crate::{Config, CreateMetadataAccountArgsV3, DataV2, MINT_AUTHORITY_SEED};

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, spl_token::instruction::AuthorityType},
    token_2022::{self, Token2022},
    token_interface::{Mint, TokenAccount, TokenMetadataInitialize},
};

#[derive(Accounts)]
pub struct MintContext<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [crate::FEE_VAULT_SEED],
        bump
    )]
    pub fee_vault: SystemAccount<'info>,

    #[account(seeds = [crate::MINT_AUTHORITY_SEED], bump)]
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
    pub token_program: Program<'info, token::Token>,

    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    /// CHECK: validated via seeds constraint against the Token Metadata program
    pub metadata: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,

    #[account(address = crate::mpl_token_metadata_id())]
    /// CHECK: validating mpl address
    pub token_metadata_program: UncheckedAccount<'info>,
}

pub fn deduct_fee(ctx: &Context<MintContext>) -> Result<()> {
    let payer = &ctx.accounts.payer;
    let fee_collection = &ctx.accounts.fee_vault;

    let amount = ctx.accounts.config.fee;

    let transfer_accounts = anchor_lang::system_program::Transfer {
        from: payer.to_account_info(),
        to: fee_collection.to_account_info(),
    };

    let transfer_ctx = CpiContext::new(system_program::id(), transfer_accounts);
    anchor_lang::system_program::transfer(transfer_ctx, amount.0)
}

pub fn mint_nft(ctx: &Context<MintContext>) -> Result<()> {
    let mint = &ctx.accounts.mint;

    let token_account = &ctx.accounts.token_account;
    let mint_authority = &ctx.accounts.mint_authority;
    let mint_authority_bump = ctx.bumps.mint_authority;

    let mint_accounts = token::MintTo {
        mint: mint.to_account_info(),
        to: token_account.to_account_info(),
        authority: mint_authority.to_account_info(),
    };

    let authority_seeds: &[&[&[u8]]] = &[&[MINT_AUTHORITY_SEED, &[mint_authority_bump]]];

    let cpi_mint_context =
        CpiContext::new_with_signer(token_2022::ID, mint_accounts, authority_seeds);

    token::mint_to(cpi_mint_context, 1)?;

    Ok(())
}

pub fn revoke_mint_authority(ctx: &Context<MintContext>) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let mint_authority = &ctx.accounts.mint_authority;

    let token_program = &ctx.accounts.token_program;

    let mint_authority_bump = ctx.bumps.mint_authority;
    let authority_seeds: &[&[&[u8]]] = &[&[MINT_AUTHORITY_SEED, &[mint_authority_bump]]];

    let revoke_accounts = token::SetAuthority {
        current_authority: mint_authority.to_account_info(),
        account_or_mint: mint.to_account_info(),
    };

    let revoke_ctx =
        CpiContext::new_with_signer(*token_program.key, revoke_accounts, authority_seeds);

    token::set_authority(revoke_ctx, AuthorityType::MintTokens, None)
}

pub fn create_metadata(ctx: &Context<MintContext>, prefix: &str) -> Result<()> {
    let mint_authority_bump = ctx.bumps.mint_authority;
    let authority_seeds: &[&[&[u8]]] = &[&[MINT_AUTHORITY_SEED, &[mint_authority_bump]]];

    crate::create_metadata_v3_cpi_signed(
        ctx,
        CreateMetadataAccountArgsV3 {
            data: DataV2 {
                name: String::from("cocaine mails"),
                symbol: String::from("1"),
                uri: String::from("metadata.cocaine.legal/mint/test"),
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            is_mutable: false,
            collection_details: None,
        },
        authority_seeds,
    )?;

    msg!("{}@cocaine.legal was created", &prefix);
    Ok(())
}
