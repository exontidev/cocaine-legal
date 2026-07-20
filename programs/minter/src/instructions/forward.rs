use anchor_lang::prelude::*;
use mpl_core::{
    accounts::{BaseAssetV1, BaseCollectionV1},
    types::UpdateAuthority,
};

use crate::{error::ContractError, Config, ForwardStore, CONFIG_SEED, FORWARD_SEED};

#[derive(Accounts)]
pub struct Forward<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init_if_needed, space = 8 + ForwardStore::INIT_SPACE, payer = payer, seeds = [FORWARD_SEED, asset.key().as_array()], bump)]
    pub forward: Account<'info, ForwardStore>,
    pub asset: Account<'info, BaseAssetV1>,

    #[account(address = config.collection)]
    pub collection: Account<'info, BaseCollectionV1>,

    #[account(seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ForwardArgs {
    pub encrypted_prefix: String,
}

pub fn handle_forward(ctx: Context<Forward>, args: ForwardArgs) -> Result<()> {
    let asset = &ctx.accounts.asset;
    let collection = &ctx.accounts.collection;

    match asset.update_authority {
        UpdateAuthority::Collection(collection_addr) => {
            require_keys_eq!(
                collection_addr,
                collection.key(),
                ContractError::WrongCollection
            );
        }
        _ => return err!(ContractError::NotInAnyCollection),
    }

    let forward = &mut ctx.accounts.forward;
    forward.encrypted_prefix = args.encrypted_prefix;

    Ok(())
}
