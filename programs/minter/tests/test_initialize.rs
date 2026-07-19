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
    minter::{AmountU64, FEE_VAULT_SEED},
    solana_keypair::Keypair,
    solana_signer::Signer,
    solana_transaction::Transaction,
};

#[test]
fn test_initialize() {
    let program_id = minter::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(env!("CARGO_TARGET_TMPDIR"), "/../deploy/minter.so"));

    svm.add_program(program_id, bytes);
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let accounts = minter::accounts::HelloWorld {};
    let data = minter::instruction::HelloWorld {};

    let ix = Instruction {
        program_id,
        accounts: accounts.to_account_metas(None),
        data: data.data(),
    };

    let blockhash = svm.latest_blockhash();

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);

    let result = svm.send_transaction(tx).unwrap();
    dbg!(result.logs);
}
