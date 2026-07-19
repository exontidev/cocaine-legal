use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, Id, InstructionData, ToAccountMetas,
    },
    anchor_spl::{
        associated_token::{get_associated_token_address_with_program_id, AssociatedToken},
        token, token_2022,
    },
    litesvm::LiteSVM,
    minter::{mpl_token_metadata_id, AmountU64, FEE_VAULT_SEED},
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::{versioned::VersionedTransaction, Transaction},
};

#[test]
fn test_initialize() {
    let program_id = minter::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(env!("CARGO_TARGET_TMPDIR"), "/../deploy/minter.so"));
    let mpl_bytes = include_bytes!("./dependencies/mpl_token_metadata.so");

    svm.add_program(program_id, bytes).unwrap();
    svm.add_program(mpl_token_metadata_id(), mpl_bytes).unwrap();

    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let accounts = minter::accounts::Initialize {
        payer: payer.pubkey(),
        config: Pubkey::find_program_address(&[b"config"], &program_id).0,
        system_program: system_program::id(),
        fee_vault: Pubkey::find_program_address(&[FEE_VAULT_SEED], &program_id).0,
        fee_authority: payer.pubkey(),
    };

    let data = minter::instruction::Initialize {
        fee: AmountU64(100_000),
        collection: Pubkey::new_unique(),
        supply: 150,
    };

    let ix = Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    };

    let blockhash = svm.latest_blockhash();

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);

    let result = svm.send_transaction(tx).unwrap();
    // dbg!(result.logs);

    let mint = Keypair::new();

    let accounts = minter::accounts::MintContext {
        payer: payer.pubkey(),
        mint_authority: Pubkey::find_program_address(&[b"mint-authority"], &program_id).0,
        mint: mint.pubkey(),
        token_account: get_associated_token_address_with_program_id(
            &payer.pubkey(),
            &mint.pubkey(),
            &token::ID,
        ),
        config: Pubkey::find_program_address(&[b"config"], &program_id).0,
        associated_token_program: AssociatedToken::id(),
        system_program: system_program::id(),
        token_program: token::ID,
        fee_vault: Pubkey::find_program_address(&[FEE_VAULT_SEED], &program_id).0,
        metadata: Pubkey::find_program_address(
            &[
                b"metadata",
                mpl_token_metadata_id().as_array(),
                mint.pubkey().as_array(),
            ],
            &mpl_token_metadata_id(),
        )
        .0,
        rent: anchor_lang::solana_program::rent::id(),
        token_metadata_program: mpl_token_metadata_id(),
    };

    let data = minter::instruction::Mint {
        prefix: String::from("dealer"),
    };

    let ix = Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    };

    let blockhash = svm.latest_blockhash();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        blockhash,
    );

    match svm.send_transaction(tx) {
        Ok(result) => {
            for log in &result.logs {
                println!("{log}");
            }
        }
        Err(failed) => {
            eprintln!("Transaction failed: {:?}", failed.err);
            eprintln!("--- logs ---");
            for log in &failed.meta.logs {
                eprintln!("{log}");
            }
            eprintln!(
                "compute units consumed: {}",
                failed.meta.compute_units_consumed
            );
        }
    }
}
