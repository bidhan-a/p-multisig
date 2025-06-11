use mollusk_svm::result::Check;
use mollusk_svm::Mollusk;
use p_multisig::constants::{MULTISIG_SEED, TRANSACTION_SEED};
use p_multisig::state::MultisigHeader;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;
use alloc::vec;

use p_multisig::state::{TransactionAccount, TransactionHeader, TransactionSigner};
use p_multisig::ID;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const PAYER: Pubkey = pubkey!("JAbjHPzyDkvkCRxTL8usK4g1sQUYTX3f42MvfCqV9LVS");

pub const INITIAL_COUNT: u64 = 42;

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/p_multisig");
    mollusk
}

#[test]

fn test_create_multisig() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    let owner_1 = Pubkey::new_from_array([0x01; 32]);
    let owner_1_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let owner_2 = Pubkey::new_from_array([0x02; 32]);
    let owner_3 = Pubkey::new_from_array([0x03; 32]);

    let seed_bytes = u64::to_le_bytes(1);
    let (multisig, multisig_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(MULTISIG_SEED.as_bytes()), &seed_bytes],
        &PROGRAM,
    );
    let multisig_account = Account::new(0, 0, &system_program);

    // Create the instruction data.
    let multisig_header = MultisigHeader {
        seed: seed_bytes,
        num_owners: u64::to_le_bytes(3),
        threshold: u64::to_le_bytes(3),
        nonce: 0,
        bump: multisig_bump,
    };

    // instruction discriminator = 0
    let mut ser_instruction_data = vec![0];

    // multisig header.
    ser_instruction_data.extend_from_slice(bytemuck::bytes_of(&multisig_header));

    // owners.
    let owners_vec = vec![owner_1, owner_2, owner_3];
    let owners: &[Pubkey] = &owners_vec;
    ser_instruction_data.extend_from_slice(bytemuck::cast_slice::<Pubkey, u8>(owners));

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(owner_1, true),
            AccountMeta::new(multisig, true),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (owner_1, owner_1_account),
            (multisig, multisig_account),
            (system_program, system_account),
        ],
        &[Check::success()],
    );
}

#[test]
fn test_create_transaction() {
    let mollusk = mollusk();

    let (system_program, system_account) = mollusk_svm::program::keyed_account_for_system_program();

    // Owners
    let owner_1 = Pubkey::new_from_array([0x01; 32]);
    let owner_1_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    let owner_2 = Pubkey::new_from_array([0x02; 32]);
    let owner_3 = Pubkey::new_from_array([0x03; 32]);
    let owners_vec = vec![owner_1, owner_2, owner_3];

    // Multisig
    let seed_bytes = u64::to_le_bytes(1);
    let (multisig, multisig_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(MULTISIG_SEED.as_bytes()), &seed_bytes],
        &PROGRAM,
    );
    let mut multisig_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let multisig_header = MultisigHeader {
        seed: seed_bytes,
        num_owners: u64::to_le_bytes(3),
        threshold: u64::to_le_bytes(2),
        nonce: 0,
        bump: multisig_bump,
    };

    let header_bytes = bytemuck::bytes_of(&multisig_header);
    let owners_bytes = bytemuck::cast_slice::<Pubkey, u8>(&owners_vec);
    let size = header_bytes.len() + owners_bytes.len();
    multisig_account.data = vec![0u8; size];
    multisig_account.data[..header_bytes.len()].copy_from_slice(header_bytes);
    multisig_account.data[header_bytes.len()..header_bytes.len() + owners_bytes.len()]
        .copy_from_slice(owners_bytes);

    // Transaction
    let tx_seed_bytes = u64::to_le_bytes(2);
    let (transaction, transaction_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
        &[(TRANSACTION_SEED.as_bytes()), &tx_seed_bytes],
        &PROGRAM,
    );
    let transaction_account = Account::new(0, 0, &system_program);

    let transaction_header = TransactionHeader {
        multisig: multisig.to_bytes(),
        program_id: system_program.to_bytes(),
        num_accounts: u64::to_le_bytes(1),
        num_signers: u64::to_le_bytes(3),
        data_len: u64::to_le_bytes(4),
        executed: 0,
        seed: tx_seed_bytes,
        bump: transaction_bump,
    };

    // Transaction accounts (just one for this test)
    let tx_accounts = [TransactionAccount {
        pubkey: owner_1.to_bytes(),
        is_signer: 1,
        is_writable: 1,
    }];

    // Transaction signers: owner_1 has signed (255), others not signed (0)
    let tx_signers = [
        TransactionSigner {
            pubkey: owner_1.to_bytes(),
            signed: 255,
        },
        TransactionSigner {
            pubkey: owner_2.to_bytes(),
            signed: 0,
        },
        TransactionSigner {
            pubkey: owner_3.to_bytes(),
            signed: 0,
        },
    ];

    // Arbitrary tx data
    let tx_data = [1u8, 2, 3, 4];

    // Serialize instruction data: discriminator, header, accounts, signers, data
    let mut ser_instruction_data = vec![1]; // discriminator for create_transaction
    ser_instruction_data.extend_from_slice(bytemuck::bytes_of(&transaction_header));
    ser_instruction_data
        .extend_from_slice(bytemuck::cast_slice::<TransactionAccount, u8>(&tx_accounts));
    ser_instruction_data
        .extend_from_slice(bytemuck::cast_slice::<TransactionSigner, u8>(&tx_signers));
    ser_instruction_data.extend_from_slice(&tx_data);

    let instruction = Instruction::new_with_bytes(
        PROGRAM,
        &ser_instruction_data,
        vec![
            AccountMeta::new(owner_1, true),
            AccountMeta::new(transaction, true),
            AccountMeta::new(multisig, true),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (owner_1, owner_1_account),
            (transaction, transaction_account),
            (multisig, multisig_account),
            (system_program, system_account),
        ],
        &[Check::success()],
    );
}
