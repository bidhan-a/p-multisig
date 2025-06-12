use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey, ProgramResult};

use crate::{
    constants::{MULTISIG_SEED, TRANSACTION_SEED},
    state::{Multisig, Transaction},
};

pub fn process_execute_transaction(accounts: &[AccountInfo]) -> ProgramResult {
    let [transaction, multisig, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let (transaction_header, accounts, signers, tx_data) = Transaction::read(transaction)?;
    let (multisig_header, owners) = Multisig::read(multisig)?;

    // Validate multisig account.
    let multisig_pda = pubkey::create_program_address(
        &[
            MULTISIG_SEED.as_bytes(),
            multisig_header.seed.as_ref(),
            &[multisig_header.bump as u8],
        ],
        &crate::ID,
    )?;
    if multisig.key() != &multisig_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate transaction account.
    let transaction_pda = pubkey::create_program_address(
        &[
            TRANSACTION_SEED.as_bytes(),
            transaction_header.seed.as_ref(),
            &[transaction_header.bump as u8],
        ],
        &crate::ID,
    )?;
    if transaction.key() != &transaction_pda {
        return Err(ProgramError::InvalidAccountData);
    }

    // Check if transaction has already been executed.
    if transaction_header.executed == 255 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Check if we have enough signners.
    let threshold = u64::from_le_bytes(multisig_header.threshold);
    let approved_count = signers.iter().filter(|s| s.signed == 255).count() as u64;
    if approved_count < threshold {
        return Err(ProgramError::InvalidInstructionData);
    }

    // TODO: Execute the transaction.

    Ok(())
}
