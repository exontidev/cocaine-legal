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
    minter::{
        AmountU64, ForwardArgs, InitializeArgs, MintArgs, CONFIG_SEED, FEE_VAULT_SEED,
        FORWARD_SEED, UPDATE_AUTHORITY_SEED,
    },
    solana_keypair::Keypair,
    solana_signer::Signer,
    solana_transaction::Transaction,
    std::dbg,
};

#[test]
fn test_initialize() {
    let program_id = minter::id();
    let mut svm = LiteSVM::new();

    let program_bytes =
        include_bytes!(concat!(env!("CARGO_TARGET_TMPDIR"), "/../deploy/minter.so"));

    let mpl_bytes = include_bytes!("./dependencies/mpl_core.so");

    svm.add_program(mpl_core::ID, mpl_bytes);
    svm.add_program(program_id, program_bytes);

    let payer = Keypair::new();
    let collection = Keypair::new();
    let asset = Keypair::new();

    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    initialize(&mut svm, &payer, &collection, program_id);
    mint(&mut svm, &payer, &asset, &collection, program_id);
    forward(&mut svm, &payer, &asset, &collection, program_id);
}

fn initialize(svm: &mut LiteSVM, payer: &Keypair, collection: &Keypair, program_id: Pubkey) {
    let accounts = minter::accounts::Initialize {
        payer: payer.pubkey(),
        config: Pubkey::find_program_address(&[CONFIG_SEED], &program_id).0,
        system_program: system_program::ID,
        collection: collection.pubkey(),
        update_authority: Pubkey::find_program_address(&[UPDATE_AUTHORITY_SEED], &program_id).0,
        mpl_core_program: mpl_core::ID,
    };

    let data = minter::instruction::Initialize {
        args: InitializeArgs {
            fee: AmountU64(10_000_000),
            fee_collector: payer.pubkey(),
            collection: collection.pubkey(),
        },
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
        &[&payer, &collection],
        blockhash,
    );

    let result = svm.send_transaction(tx).unwrap();
}

fn mint(
    svm: &mut LiteSVM,
    payer: &Keypair,
    asset: &Keypair,
    collection: &Keypair,
    program_id: Pubkey,
) {
    let mint_accounts = minter::accounts::MintContext {
        asset: asset.pubkey(),
        collection: collection.pubkey(),
        update_authority: Pubkey::find_program_address(&[UPDATE_AUTHORITY_SEED], &program_id).0,
        payer: payer.pubkey(),
        config: Pubkey::find_program_address(&[CONFIG_SEED], &program_id).0,
        system_program: system_program::id(),
        mpl_core_program: mpl_core::ID,
        fee_vault: Pubkey::find_program_address(&[FEE_VAULT_SEED], &program_id).0,
    };

    let mint_message = minter::instruction::Mint {
        args: MintArgs {
            prefix: String::from("dealer"),
        },
    };

    let ix = Instruction {
        program_id,
        accounts: mint_accounts.to_account_metas(None),
        data: mint_message.data(),
    };

    let blockhash = svm.latest_blockhash();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer, &asset],
        blockhash,
    );

    let result = svm.send_transaction(tx).unwrap();
    dbg!(result.logs);
}

fn forward(
    svm: &mut LiteSVM,
    payer: &Keypair,
    asset: &Keypair,
    collection: &Keypair,
    program_id: Pubkey,
) {
    let mint_accounts = minter::accounts::Forward {
        asset: asset.pubkey(),
        collection: collection.pubkey(),

        payer: payer.pubkey(),
        config: Pubkey::find_program_address(&[CONFIG_SEED], &program_id).0,
        system_program: system_program::id(),

        forward: Pubkey::find_program_address(
            &[FORWARD_SEED, asset.pubkey().as_array()],
            &program_id,
        )
        .0,
    };

    let mint_message = minter::instruction::Forward {
        args: ForwardArgs {
            encrypted_prefix: String::from("dealer"),
        },
    };

    let ix = Instruction {
        program_id,
        accounts: mint_accounts.to_account_metas(None),
        data: mint_message.data(),
    };

    let blockhash = svm.latest_blockhash();

    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let result = svm.send_transaction(tx).unwrap();

    dbg!(svm.get_account(
        &Pubkey::find_program_address(&[FORWARD_SEED, asset.pubkey().as_array()], &program_id,).0
    ));
}
