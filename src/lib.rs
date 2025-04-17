#![feature(const_mut_refs)]
use pinocchio::{
  account_info::AccountInfo,
  default_panic_handler,
  msg,
  no_allocator,
  default_allocator,
  lazy_program_entrypoint,
  ProgramResult,
  pubkey::Pubkey,
    entrypoint::InstructionContext,
};

lazy_program_entrypoint!(process_instruction);
no_allocator!();
default_panic_handler!();

pub fn process_instruction(
    mut context: InstructionContext
) -> ProgramResult {
    msg!("Hello from lazy no allocator program!");
    Ok(())
}
