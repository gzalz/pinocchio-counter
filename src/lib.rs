use pinocchio::{
  account_info::AccountInfo,
  entrypoint,
  msg,
  ProgramResult,
  pubkey::Pubkey
};
use pinocchio_system::instructions::Transfer;
entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
    let payer_info = accounts.get(0).unwrap();
    let dest_info = accounts.get(1).unwrap();
    let lamport_bytes: [u8; 8] = instruction_data[..8].try_into().unwrap();
    Transfer {
        from: payer_info,
        to: dest_info,
        lamports: u64::from_le_bytes(lamport_bytes),
    }.invoke()?;
    Ok(())
}
