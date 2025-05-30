//! A simple blockchain sandbox application that allows users to:
//! - Create and mine blocks
//! - Add transactions to the blockchain
//! - Adjust mining difficulty and rewards
//! - View the current state of the blockchain
#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs,
    unreachable_pub,
    unused_crate_dependencies
)]
#![warn(
    rust_2018_idioms,
    rust_2021_compatibility,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::doc_markdown,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::similar_names,
    clippy::struct_excessive_bools
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/t1ltxz-gxd/blockchain-sandbox/main/assets/images/logo.png"
)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/t1ltxz-gxd/blockchain-sandbox/main/assets/images/favicon.png"
)]

use colored::Colorize;
pub mod blockchain;

/// Main entry point for the blockchain sandbox application.
///
/// Initializes a blockchain with user-specified miner address and difficulty,
/// then presents an interactive menu for blockchain operations.
fn main() {
    let mut miner_address = String::new();
    println!("Enter miner address: ");
    std::io::stdin()
        .read_line(&mut miner_address)
        .expect("Failed to read line");
    let miner_address = miner_address.trim().to_string();

    let mut difficulty = String::new();
    println!("Enter difficulty (default 2): ");
    std::io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");
    let difficulty: u32 = difficulty.trim().parse().unwrap_or(2);

    println!("Generating genesis block...");
    let mut chain = blockchain::Chain::new(miner_address, difficulty, None);

    if let Some(genesis) = chain.get_latest_block_json() {
        println!("Genesis Block:\n{}", genesis.green());
    }

    loop {
        println!();
        println!("{}", "Choose an option:".blue().bold());
        println!("{}", "1. New Transaction".magenta());
        println!("{}", "2. Mine a new block".green());
        println!("{}", "3. Change difficulty".yellow());
        println!("{}", "4. Change reward".cyan());
        println!("{}", "5. Show blockchain".white());
        println!("{}", "0. Exit".red().underline());

        print!("Enter your choice: ");
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice = choice.trim();
        handle_menu_choice(&mut chain, choice);
        if choice == "0" {
            break;
        }
    }
}

/// Processes user menu choices and performs the corresponding blockchain operations.
///
/// # Arguments
///
/// * `chain` - A mutable reference to the blockchain instance
/// * `choice` - The user's menu selection as a string
///
/// # Menu Options
///
/// * "1": Add a new transaction to the pending transactions pool
/// * "2": Mine a new block with pending transactions
/// * "3": Change the mining difficulty
/// * "4": Change the mining reward
/// * "5": Display the entire blockchain
/// * "0": Exit the application
fn handle_menu_choice(chain: &mut blockchain::Chain, choice: &str) {
    match choice {
        "1" => {
            let mut sender = String::new();
            let mut receiver = String::new();
            let mut amount = String::new();

            println!("Sender: ");
            std::io::stdin().read_line(&mut sender).unwrap();
            println!("Receiver: ");
            std::io::stdin().read_line(&mut receiver).unwrap();
            println!("Amount: ");
            std::io::stdin().read_line(&mut amount).unwrap();

            let sender = sender.trim();
            let receiver = receiver.trim();
            let amount: f32 = amount.trim().parse().unwrap_or(0.0);

            if chain.add_transaction(sender.to_string(), receiver.to_string(), amount) {
                println!("{}", "Transaction added successfully:".green().bold());
                println!("From: {sender}");
                println!("To: {receiver}");
                println!("Amount: {amount}");
            } else {
                println!("{}", "Failed to add transaction.".red());
            }
        }

        "2" => {
            println!("{}", "Mining new block...".yellow().bold());

            chain.generate_new_block();

            let latest_block_after = chain.get_chain().last().unwrap();
            let block_hash = blockchain::Chain::hash(&latest_block_after.get_header());
            println!("{}", "New block mined:".green().bold());
            println!("Hash:         {block_hash}");
            println!(
                "Prev Hash:    {}",
                latest_block_after.get_header().get_previous_hash()
            );
            println!(
                "Nonce:        {}",
                latest_block_after.get_header().get_nonce()
            );
            println!(
                "Transactions: {}",
                latest_block_after.get_transactions().len()
            );
            println!("Reward:       {}", chain.get_reward());
        }

        "3" => {
            let mut new_difficulty = String::new();
            println!("Enter new difficulty: ");
            std::io::stdin().read_line(&mut new_difficulty).unwrap();
            let new_difficulty: u32 = new_difficulty
                .trim()
                .parse()
                .unwrap_or_else(|_| chain.get_difficulty());

            let old_difficulty = chain.get_difficulty();
            chain.update_difficulty(new_difficulty);
            println!("{}", "Difficulty updated:".cyan().bold());
            println!("Old: {old_difficulty}");
            println!("New: {new_difficulty}");
        }

        "4" => {
            let mut new_reward = String::new();
            println!("Enter new reward: ");
            std::io::stdin().read_line(&mut new_reward).unwrap();
            let new_reward: f32 = new_reward
                .trim()
                .parse()
                .unwrap_or_else(|_| chain.get_reward());

            let old_reward = chain.get_reward();
            chain.update_reward(new_reward);
            println!("{}", "Reward updated:".cyan().bold());
            println!("  Old: {old_reward}");
            println!("  New: {new_reward}");
        }

        "5" => {
            println!("{}", "Current blockchain:".bold());
            for (i, block) in chain.get_blocks_json().iter().enumerate() {
                println!("--- Block #{i} ---\n{block}\n");
            }
        }

        "0" => {
            println!("{}", "Exiting program.".red().bold());
        }

        _ => println!("{}", "Invalid choice, try again.".red()),
    }
}
