use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::{
    constants::{MULTISIG_SEED, TRANSACTION_SEED},
    state::{Multisig, Transaction},
};

pub fn process_create_transaction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [user, transaction, multisig, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !user.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (transaction_header, accounts, signers, tx_data) = Transaction::parse(&data)?;
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

    // For each owner, check that there is a signer entry with matching pubkey.
    for owner in owners {
        match signers.iter().find(|s| s.pubkey == *owner) {
            Some(signer_entry) => {
                // If this owner is the transaction creator, check that they have signed.
                if owner == user.key() && signer_entry.signed != 255 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                // Other owners must not have signed yet (0)
                else if owner != user.key() && signer_entry.signed != 0 {
                    return Err(ProgramError::InvalidInstructionData);
                }
            }
            None => {
                // Owner not found in signers list.
                return Err(ProgramError::InvalidInstructionData);
            }
        }
    }

    // Create transaction account.
    let num_accounts = u64::from_le_bytes(transaction_header.num_accounts);
    let num_signers = u64::from_le_bytes(transaction_header.num_signers);
    let data_len = u64::from_le_bytes(transaction_header.data_len);
    let size = Transaction::size(num_accounts, num_signers, data_len);
    pinocchio_system::instructions::CreateAccount {
        from: user,
        to: transaction,
        space: size as u64,
        lamports: Rent::get()?.minimum_balance(size),
        owner: &crate::ID,
    }
    .invoke()?;

    // Write data to transaction account.
    Transaction::write(transaction, transaction_header, accounts, signers, tx_data)?;

    Ok(())
}
