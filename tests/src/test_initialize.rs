use anchor_client::solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::read_keypair_file,
    signer::Signer,
    system_program,
};
use anchor_client::{Client, Cluster};

#[test] 
fn test_initialize() {
    let program_id = favorites_program::ID;
    let anchor_wallet = std::env::var("ANCHOR_WALLET").expect("Set ANCHOR_WALLET env var");
    let payer = read_keypair_file(&anchor_wallet).expect("Failed to read keypair");

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());
    let program = client.program(program_id).expect("Program not found");

    // derive PDA
    let (favorites_pda, _bump) = Pubkey::find_program_address(
        &[b"favorites", payer.pubkey().as_ref()],
        &program_id,
    );

    let number = 42u64;
    let color = "blue".to_string();
    let hobbies = vec!["chess".to_string(), "reading".to_string()];

    let tx = program
        .request()
        .accounts(favorites_program::accounts::SetFavorites {
            user: payer.pubkey(),
            favorites: favorites_pda,
            system_program: system_program::ID,
        })
        .args(favorites_program::instruction::SetFavorites {
            number,
            color,
            hobbies,
        })
        .send()
        .expect("Transaction failed");

    println!("✅ Transaction signature: {}", tx);
}
