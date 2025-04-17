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
    // RPC and payer setup
    let rpc_url = "http://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());
    let payer = read_keypair_file("/Users/eric/.config/solana/id.json").unwrap(); // or read from file

    // The program to invoke
    let program_id = Pubkey::from_str("8tNV32xuAQnkaHLvkRkCU1QfyDYoDT3F48saRQPcGTTy").unwrap();
    let dest_pubkey = Pubkey::from_str("Sp5wv9Tyb9P3NiWLAohbxHVUDq2APRd9tdT2sLxbLZ3").unwrap();
    let system_program = Pubkey::from_str("11111111111111111111111111111111").unwrap();

    // Optional: Accounts to include (can be empty or just the program itself)
    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true), // writable + signer
        AccountMeta::new(dest_pubkey, false),
        AccountMeta::new(system_program, false),
    ];

    // Instruction with no data
    let instruction = Instruction {
        program_id,
        accounts,
        data: vec![42, 0, 0, 0, 0, 0, 0, 0], // empty data
    };

    // Build and send the transaction
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let message = Message::new(&[instruction], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, recent_blockhash);

    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!("Transaction sent: {sig}");
}
