use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::{constants::MULTISIG_SEED, state::Multisig};

pub fn process_create_multisig(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [user, multisig, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !user.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (multisig_header, owners) = Multisig::parse(&data)?;

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

    // Validate owners.
    let threshold = u64::from_le_bytes(multisig_header.threshold) as usize;
    if !(threshold > 0 && owners.len() > 0 && threshold <= owners.len()) {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Check if user is in the list of owners.
    if !owners.iter().any(|k| k.eq(user.key())) {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Create multisig account.
    let num_owners = u64::from_le_bytes(multisig_header.num_owners);
    let size = Multisig::size(num_owners);
    pinocchio_system::instructions::CreateAccount {
        from: user,
        to: multisig,
        space: size as u64,
        lamports: Rent::get()?.minimum_balance(size),
        owner: &crate::ID,
    }
    .invoke()?;

    // Write data to multisig account.
    Multisig::write(multisig, multisig_header, owners)?;

    Ok(())
}
