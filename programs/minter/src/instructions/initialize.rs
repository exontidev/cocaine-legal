use crate::{Config, CONFIG_SEED, UPDATE_AUTHORITY_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub collection: Signer<'info>,

    /// CHECK: PDA authority.
    #[account(seeds = [UPDATE_AUTHORITY_SEED], bump)]
    pub update_authority: UncheckedAccount<'info>,

    #[account(init, space = 8 + Config::INIT_SPACE, payer = payer, seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    #[account(address = mpl_core::ID)]
    /// CHECK: program id constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeArgs {
    pub fee: crate::Sol,
    pub fee_collector: Pubkey,
    pub collection: Pubkey,
}

pub fn handle_initialize(ctx: Context<Initialize>, args: InitializeArgs) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.from_args(args);

    create_collection(&ctx)?;

    Ok(())
}

fn create_collection(ctx: &Context<Initialize>) -> Result<()> {
    mpl_core::instructions::CreateCollectionV2CpiBuilder::new(&ctx.accounts.mpl_core_program)
        .collection(&ctx.accounts.collection)
        .payer(&ctx.accounts.payer)
        .update_authority(Some(&ctx.accounts.update_authority))
        .system_program(&ctx.accounts.system_program)
        .name(String::from("COCAINE MAILS"))
        .uri(String::from("metadata.cocaine.legal/collection"))
        .invoke()?;

    Ok(())
}
