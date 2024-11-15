use std::time::Duration;

use anyhow::Result;
use clap::{Parser, Subcommand};

use memory_cache::{load_cache, save_cache};

#[derive(Debug, Parser)]
#[clap(author, version, about="Tis a tool for caching", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(author, version, about="does an insert into the cache", long_about = None)]
    Insert {
        #[clap(short, long)]
        key: String,

        #[clap(short, long)]
        value: String,

        #[clap(short, long, default_value = "30")]
        ttl: u64,
    },
    Get {
        #[clap(short, long)]
        key: String,
    },
    Invalidate {
        #[clap(short, long)]
        key: String,
    },
}

//TODO - discuss original plans for the tool.
fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut cache = load_cache().unwrap();

    match cli.command {
        Commands::Insert { key, value, ttl } => {
            cache.insert(&key, value, Duration::from_secs(ttl));
            println!("Inserted key '{}'", key);
        }
        Commands::Get { key } => {
            match cache.get(&key) {
                Some(value) => println!("Value for key '{}': {}", key, value),
                None => println!("No value found for key '{}'", key),
            }
        }
        Commands::Invalidate { key } => {
            cache.invalidate(&key);
            println!("Invalidated key '{}'", key);
        }
    }
    save_cache(&cache)?;

    Ok(())
}
