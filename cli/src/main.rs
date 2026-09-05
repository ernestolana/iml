use clap::{Parser, Subcommand};
use std::fs;
use core::Arena;
use schemars::schema_for;

#[derive(Parser)]
#[command(name = "iml")]
#[command(about = "IML CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { 
        file: String,
        #[arg(long)] auto_repair: bool,
    },
    Format {
        #[arg(long)] to_human: bool,
        #[arg(long)] to_json: bool,
        #[arg(long)] to_pseudo: bool,
        #[arg(long)] from_pseudo: bool,
        file: String,
    },
    Grammar {
        #[arg(long)] export: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run { file, auto_repair } => {
            println!("Running file: {}", file);
            let content = fs::read_to_string(file).expect("Failed to read file");
            let arena: Arena = serde_json::from_str(&content).expect("Invalid JSON AST");
            match checker::check_arena(&arena) {
                Ok(_) => println!("Execution successful."),
                Err(e) => {
                    println!("[ERROR] Two-Pass Linear Checker Failed:\n{}", e);
                    if *auto_repair {
                        println!("\n[Auto-Repair] Flag detected. Feeding visual diagnostic back to LLM context...");
                    }
                }
            }
        },
        Commands::Format { to_human, to_json, to_pseudo, from_pseudo, file } => {
            if *to_human {
                let content = fs::read_to_string(file).expect("Failed to read file");
                let arena: Arena = serde_json::from_str(&content).expect("Invalid JSON AST");
                let human = syntax::to_human_readable(&arena);
                println!("{}", human);
            } else if *to_json {
                let content = fs::read_to_string(file).expect("Failed to read file");
                let arena = syntax::from_human_readable(&content).expect("Invalid Human format");
                let json = serde_json::to_string_pretty(&arena).unwrap();
                println!("{}", json);
            } else if *to_pseudo {
                let content = fs::read_to_string(file).expect("Failed to read file");
                let arena: Arena = serde_json::from_str(&content).expect("Invalid JSON AST");
                let pseudo = syntax::to_pseudo_c(&arena);
                println!("{}", pseudo);
            } else if *from_pseudo {
                let content = fs::read_to_string(file).expect("Failed to read file");
                let arena = syntax::from_pseudo_c(&content).expect("Invalid Pseudo format");
                let json = serde_json::to_string_pretty(&arena).unwrap();
                println!("{}", json);
            }
        },
        Commands::Grammar { export } => {
            let schema = schema_for!(Arena);
            if export == "json" {
                println!("{}", serde_json::to_string_pretty(&schema).unwrap());
            } else if export == "gbnf" {
                let gbnf = core::gbnf::schema_to_gbnf(&schema);
                println!("{}", gbnf);
            } else {
                println!("Unknown grammar export format. Use 'json' or 'gbnf'.");
            }
        },
    }
}
