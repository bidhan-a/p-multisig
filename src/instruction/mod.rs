use pinocchio::program_error::ProgramError;

pub mod create_multisig;

pub use create_multisig::*;

#[repr(u8)]
pub enum MultisigInstruction {
    CreateMultisig,
}

impl TryFrom<&u8> for MultisigInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(MultisigInstruction::CreateMultisig),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
