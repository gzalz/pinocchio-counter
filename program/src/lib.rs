// this is for no_alloc #![feature(const_mut_refs)]
use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    instruction::{Seed, Signer},
    program_error::{ProgramError, INVALID_ACCOUNT_DATA},
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

entrypoint!(process_instruction);

const INITIALIZE: u8 = 1;
const INCREMENT: u8 = 2;

/// program_id = pubkey!("H7NQGd5ZDZtHJNmCpgyi6b3kuoJpS8mvQCrVrg9yRt9V")
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix_discr = instruction_data[0];
    if ix_discr == INITIALIZE {
        process_init(program_id, accounts, instruction_data)?;
    } else if ix_discr == INCREMENT {
        process_increment(program_id, accounts, instruction_data)?;
    } else {
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(())
}

///   Initializes a new `Counter`.
///
///   0. `[s]` signer
///   1. `[w]` counter = find_program_address!(b"counter")
///   2. `[ ]`  systemProgram = pubkey!("11111111111111111111111111111111")
pub fn process_init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let signer_info: &AccountInfo = &accounts[0];
    let counter_info: &AccountInfo = &accounts[1];

    let bump = [instruction_data[1]];

    let seed = [Seed::from(b"counter"), Seed::from(&bump)];
    let seeds = Signer::from(&seed);
    let size = 8;

    pinocchio_system::instructions::CreateAccount {
        from: signer_info,
        to: counter_info,
        lamports: Rent::get()?.minimum_balance(size),
        space: size as u64,
        owner: program_id,
    }
    .invoke_signed(&[seeds.clone()])?;

    counter_info.realloc(size, false)?;
    Ok(())
}

///   Increments the `Counter` by 1.
///   0. `[s]` Signer
///   1. `[w]` Counter
pub fn process_increment(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let counter_info: &AccountInfo = &accounts[1];
    let mut value_bytes = counter_info.try_borrow_mut_data()?;
    let counter_value = value_bytes[0..8]
        .try_into()
        .map(u64::from_le_bytes)
        .map_err(|_| INVALID_ACCOUNT_DATA)?;
    let new_counter_value = counter_value.wrapping_add(1);
    value_bytes[0..8].copy_from_slice(&new_counter_value.to_le_bytes());
    Ok(())
}
