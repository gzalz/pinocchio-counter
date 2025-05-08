use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let mut ixn_discr = 0;
    if command == "init" {
        ixn_discr = 1;
    } else if command == "increment" {
        ixn_discr = 2;
    } 

    let rpc_url = "http://localhost:8899";
    let client = RpcClient::new(rpc_url.to_string());
    let payer = read_keypair_file("/Users/eric/.config/solana/id.json").unwrap();

    let program_id = Pubkey::from_str("H7NQGd5ZDZtHJNmCpgyi6b3kuoJpS8mvQCrVrg9yRt9V").unwrap();
    let (pda_pubkey, bump) = Pubkey::find_program_address(&[b"counter".as_ref()], &program_id);
    let system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();

    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(pda_pubkey, false),
        AccountMeta::new(system_program, false),
    ];

    let instruction = Instruction {
        program_id,
        accounts,
        data: vec![ixn_discr, bump],
    };

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, recent_blockhash);

    let signature = client.send_and_confirm_transaction(&tx).unwrap();
    println!("Transaction sent: {signature}");
}
