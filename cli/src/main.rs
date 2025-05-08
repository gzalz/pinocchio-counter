use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    system_program,
};
use solana_sdk::instruction::AccountMeta;
 use std::str::FromStr;

fn main() {
    let rpc_url = "http://localhost:8899";
    let client = RpcClient::new(rpc_url.to_string());
    let payer = read_keypair_file("/Users/eric/.config/solana/id.json").unwrap();

    let program_id = Pubkey::from_str("H7NQGd5ZDZtHJNmCpgyi6b3kuoJpS8mvQCrVrg9yRt9V").unwrap();
    let (pda_pubkey, bump) = Pubkey::find_program_address(
        &[b"counter".as_ref()],
        &program_id,
    );

    let system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    let sysvar_rent = Pubkey::from_str("SysvarRent111111111111111111111111111111111").unwrap();

    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(pda_pubkey, false),
        AccountMeta::new(system_program, false),
        AccountMeta::new(sysvar_rent, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts,
        data: vec![2, bump],
    };

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, recent_blockhash);

    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!("Transaction sent: {sig}");
}
