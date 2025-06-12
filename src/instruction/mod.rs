use pinocchio::program_error::ProgramError;

pub mod approve_transaction;
pub mod create_multisig;
pub mod create_transaction;
pub mod execute_transaction;

pub use approve_transaction::*;
pub use create_multisig::*;
pub use create_transaction::*;
pub use execute_transaction::*;

#[repr(u8)]
pub enum MultisigInstruction {
    CreateMultisig,
    CreateTransaction,
    ApproveTransaction,
    ExecuteTransaction,
}

impl TryFrom<&u8> for MultisigInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MultisigInstruction::CreateMultisig),
            1 => Ok(MultisigInstruction::CreateTransaction),
            2 => Ok(MultisigInstruction::ApproveTransaction),
            3 => Ok(MultisigInstruction::ExecuteTransaction),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
