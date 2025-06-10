use mollusk_svm::result::Check;
use mollusk_svm::Mollusk;
use p_multisig::constants::MULTISIG_SEED;
use p_multisig::state::MultisigHeader;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;
use alloc::vec;

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
