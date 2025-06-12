use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey, ProgramResult};

use crate::{
    constants::{MULTISIG_SEED, TRANSACTION_SEED},
    state::{Multisig, Transaction},
};

pub fn process_approve_transaction(accounts: &[AccountInfo]) -> ProgramResult {
    let [user, transaction, multisig, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !user.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (transaction_header, _, _, _) = Transaction::read(transaction)?;
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

    // Check if user is in the list of multisig owners.
    if !owners.iter().any(|k| k.eq(user.key())) {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Update the signed status for the user.
    let signers = Transaction::signers(transaction)?;
    for signer_entry in signers.iter_mut() {
        if signer_entry.pubkey == *user.key() {
            if signer_entry.signed == 255 {
                // User has already approved the transaction.
                return Err(ProgramError::InvalidInstructionData);
            } else {
                signer_entry.signed = 255;
                break;
            }
        }
    }

    Ok(())
}
