#![allow(unexpected_cfgs)]
use pinocchio::{
    address::declare_id, entrypoint, error::ProgramError, AccountView, Address, ProgramResult,
};

use crate::instructions::EscrowInstrctions;

mod instructions;
mod state;
mod tests;

entrypoint!(process_instruction);

declare_id!("4ibrEMW5F6hKnkW4jVedswYv6H6VtwPN6ar6dvXDN1nT");

pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &ID);

    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match EscrowInstrctions::try_from(discriminator)? {
        // EscrowInstrctions::MakeV2 => instructions::process_make_instruction_v2(accounts, data)?,
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
