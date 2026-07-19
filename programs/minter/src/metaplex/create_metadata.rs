use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::{AccountMeta, Instruction};
use anchor_lang::solana_program::program::{invoke, invoke_signed};

use crate::MintContext;

pub fn create_metadata_v3_cpi_signed(
    ctx: &Context<MintContext>,
    args: CreateMetadataAccountArgsV3,
    seeds: &[&[&[u8]]],
) -> Result<()> {
    let mut data = vec![33u8];
    args.serialize(&mut data)?;

    let accounts = vec![
        AccountMeta::new(*ctx.accounts.metadata.key, false),
        AccountMeta::new_readonly(*&ctx.accounts.mint.key(), false),
        AccountMeta::new_readonly(*ctx.accounts.mint_authority.key, true),
        AccountMeta::new(*ctx.accounts.payer.key, true),
        AccountMeta::new_readonly(*&ctx.accounts.mint_authority.key(), false),
        AccountMeta::new_readonly(*&ctx.accounts.system_program.key(), false),
        AccountMeta::new_readonly(*&ctx.accounts.rent.key(), false),
    ];

    let ix = Instruction {
        program_id: *ctx.accounts.token_metadata_program.key,
        accounts,
        data,
    };

    invoke_signed(
        &ix,
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        seeds,
    )?;

    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateMetadataAccountArgsV3 {
    pub data: DataV2,
    pub is_mutable: bool,
    pub collection_details: Option<CollectionDetails>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DataV2 {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum CollectionDetails {
    V1 { size: u64 },
    V2 { padding: [u8; 8] },
}
