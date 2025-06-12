use crate::instruction::{self, MultisigInstruction};
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// Define the program entrypoint.
program_entrypoint!(process_instruction);
// Do not allocate memory.
no_allocator!();
// Use the nostd panic handler.
nostd_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match MultisigInstruction::try_from(ix_disc)? {
        MultisigInstruction::CreateMultisig => {
            instruction::process_create_multisig(accounts, instruction_data)
        }
        MultisigInstruction::CreateTransaction => {
            instruction::process_create_transaction(accounts, instruction_data)
        }
        MultisigInstruction::ApproveTransaction => {
            instruction::process_approve_transaction(accounts)
        }
        MultisigInstruction::ExecuteTransaction => {
            instruction::process_execute_transaction(accounts)
        }
    }
}
