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
    Run { file: String },
    Format {
        #[arg(long)] to_human: bool,
        #[arg(long)] to_json: bool,
        file: String,
    },
    Grammar {
        #[arg(long)] export: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Run { file } => {
            println!("Running file: {}", file);
        },
        Commands::Format { to_human, to_json, file } => {
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
