// this is for no_alloc #![feature(const_mut_refs)]
use pinocchio::{
    account_info::AccountInfo,
    default_allocator, default_panic_handler, entrypoint,
    entrypoint::lazy::MaybeAccount,
    entrypoint::InstructionContext,
    lazy_program_entrypoint, msg,
    program_error::{ProgramError, INVALID_ACCOUNT_DATA},
    pubkey::Pubkey,
    ProgramResult,
};

lazy_program_entrypoint!(process_instruction);
default_allocator!();
default_panic_handler!();

#[repr(C)]
#[derive(Debug)]
pub struct ZeroCopy {
    pub field1: u64,
    pub field2: u32,
}

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    if context.available() == 0 {
        return Err(ProgramError::InvalidAccountData);
    }
    let zc_account_info: AccountInfo = context.next_account().unwrap().assume_account();
    let zc_data = zc_account_info.try_borrow_data().unwrap();
    let ptr = zc_data.as_ptr() as *const ZeroCopy;
    let zc_struct = unsafe { &*ptr };
    msg!(zc_struct.field1.to_string().as_str());
    msg!(zc_struct.field2.to_string().as_str());
    Ok(())
}
