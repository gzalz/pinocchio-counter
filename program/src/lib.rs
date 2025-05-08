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

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix_discr = instruction_data[0];
    if ix_discr == INITIALIZE {
        let payer_info: &AccountInfo = &accounts[0];
        let pda_info: &AccountInfo = &accounts[1];

        let bump = [instruction_data[1]];

        let seed = [Seed::from(b"counter"), Seed::from(&bump)];
        let seeds = Signer::from(&seed);
        let size = 8;

        pinocchio_system::instructions::CreateAccount {
            from: payer_info,
            to: pda_info,
            lamports: Rent::get()?.minimum_balance(size),
            space: size as u64,
            owner: program_id,
        }
        .invoke_signed(&[seeds.clone()])?;

        pda_info.realloc(size, false)?;
    } else if ix_discr == INCREMENT {
        let pda_info: &AccountInfo = &accounts[1];
        let mut data = pda_info.try_borrow_mut_data()?;
        let counter_value = data[0..8]
            .try_into()
            .map(u64::from_le_bytes)
            .map_err(|_| INVALID_ACCOUNT_DATA)?;
        let new_counter_value = counter_value.wrapping_add(1);
        data[0..8].copy_from_slice(&new_counter_value.to_le_bytes());
    } else {
        return Err(ProgramError::InvalidInstructionData);
    }
    Ok(())
}
