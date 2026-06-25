mod aur;
mod session;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aurguard")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        package: String,
    },
    Inspect {
        package: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { package } => {
            session::create_session(&package);
            aur::scan_package(&package);
        }
        Commands::Inspect { package } => {
            if !session::can_inspect(&package) {
                println!(
                    "Package '{}' is not available to inspect.",
                    package
                );
                println!(
                    "Run scan first or inspect a listed dependency."
                );
                return;
            }
            aur::inspect_package(&package);
        }
    }
}
