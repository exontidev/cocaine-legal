use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, Id, InstructionData, ToAccountMetas,
    },
    anchor_spl::{
        associated_token::{get_associated_token_address_with_program_id, AssociatedToken},
        token_2022,
    },
    litesvm::LiteSVM,
    minter::AmountU64,
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

    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let accounts = minter::accounts::Initialize {
        payer: payer.pubkey(),
        config: Pubkey::find_program_address(&[b"config"], &program_id).0,
        system_program: system_program::id(),
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
            &token_2022::ID,
        ),
        config: Pubkey::find_program_address(&[b"config"], &program_id).0,
        associated_token_program: AssociatedToken::id(),
        system_program: system_program::id(),
        token_program: token_2022::ID,
    };

    let data = minter::instruction::Mint {};

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
    let result = svm.send_transaction(tx).unwrap();
    dbg!(result.logs);
}
