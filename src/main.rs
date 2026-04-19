use std::path::PathBuf;

use clap::{Parser, Subcommand};
use merkle_tree_collection_reader::meta_merkle_tree::generated_merkle_tree::GeneratedMerkleTreeCollection;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(long)]
    save_path: PathBuf,

    /// Number of times to greet
    #[arg(long)]
    epoch: u64,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SerdeJson,
    SerdeJsonSlice,
    SimdJson,
}

pub fn merkle_tree_collection_file_name(epoch: u64) -> String {
    format!("{}_merkle_tree_collection.json", epoch)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let merkle_tree_path = args
        .save_path
        .join(merkle_tree_collection_file_name(args.epoch));

    match args.command {
        Commands::SerdeJson => {
            let _merkle_trees =
                GeneratedMerkleTreeCollection::new_from_file_serde_json(&merkle_tree_path)
                    .map_err(|e| anyhow::anyhow!("Failed to load merkle tree: {e}"))?;
        }
        Commands::SerdeJsonSlice => {
            let _merkle_trees =
                GeneratedMerkleTreeCollection::new_from_file_serde_json_slice(&merkle_tree_path)
                    .map_err(|e| anyhow::anyhow!("Failed to load merkle tree: {e}"))?;
        }
        Commands::SimdJson => {
            let _merkle_trees =
                GeneratedMerkleTreeCollection::new_from_file_simd_json(&merkle_tree_path)
                    .map_err(|e| anyhow::anyhow!("Failed to load merkle tree: {e}"))?;
        }
    }

    Ok(())
}
