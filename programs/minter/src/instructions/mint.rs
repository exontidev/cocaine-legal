use std::format;

use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer},
};

use crate::{Config, CONFIG_SEED, FEE_VAULT_SEED, UPDATE_AUTHORITY_SEED};

#[derive(Accounts)]
pub struct MintContext<'info> {
    #[account(mut)]
    pub asset: Signer<'info>,

    #[account(mut, address = config.collection)]
    /// CHECK: config collection verify
    pub collection: UncheckedAccount<'info>,

    /// CHECK: PDA authority.
    #[account(seeds = [UPDATE_AUTHORITY_SEED], bump)]
    pub update_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,

    #[account(mut, seeds = [FEE_VAULT_SEED], bump)]
    /// CHECK: PDA account
    pub fee_vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    #[account(address = mpl_core::ID)]
    /// CHECK: mpl address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

/// TODO: prefix encrypted
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintArgs {
    pub prefix: String,
}

pub fn handle_mint(ctx: Context<MintContext>, args: MintArgs) -> Result<()> {
    deduct_fee(&ctx)?;
    mint_nft(&ctx, args)?;

    Ok(())
}

fn deduct_fee(ctx: &Context<MintContext>) -> Result<()> {
    let transfer_accounts = Transfer {
        from: ctx.accounts.payer.to_account_info(),
        to: ctx.accounts.fee_vault.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_accounts,
    );

    system_program::transfer(cpi_context, ctx.accounts.config.fee.0)
}

fn mint_nft(ctx: &Context<MintContext>, args: MintArgs) -> Result<()> {
    let bump = ctx.bumps.update_authority;
    let seeds: &[&[u8]] = &[UPDATE_AUTHORITY_SEED, &[bump]];
    let signer_seeds = &[seeds];

    mpl_core::instructions::CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .asset(&ctx.accounts.asset)
        .collection(Some(&ctx.accounts.collection))
        .payer(&ctx.accounts.payer)
        .authority(Some(&ctx.accounts.update_authority)) // collection's update authority
        .owner(Some(&ctx.accounts.payer))
        .update_authority(None) // defaults to collection's authority
        .system_program(&ctx.accounts.system_program)
        .name(args.prefix)
        .uri(format!(
            "metadata.cocaine.legal/nft/{}",
            ctx.accounts.asset.key
        ))
        .invoke_signed(signer_seeds)?;

    Ok(())
}
