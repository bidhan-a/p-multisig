use pinocchio::program_error::ProgramError;

pub mod create_multisig;
pub mod create_transaction;

pub use create_multisig::*;
pub use create_transaction::*;

#[repr(u8)]
pub enum MultisigInstruction {
    CreateMultisig,
    CreateTransaction,
}

impl TryFrom<&u8> for MultisigInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MultisigInstruction::CreateMultisig),
            1 => Ok(MultisigInstruction::CreateTransaction),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
