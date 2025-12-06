mod file_system;
mod utils;
use std::env::args;

use crate::file_system::{create_destination_dir_if_not_exists};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().collect::<Vec<String>>();

    let (source_path, destination_path) = utils::parse_args(args)?;

    println!("Building file tree for source path...");
    let src_tree = file_system::build_tree(*source_path.clone())?;

    let dir_exists = create_destination_dir_if_not_exists(&destination_path)?;

    if !dir_exists {
        return Ok(());
    }

    let (file_type, name, path, metadata, children) = file_system::get_node_info(&src_tree);
    println!("File Type: {}, Name: {}, Path: {}, Metadata: {:?}, Children: {:?}", file_type, name, path.display(), metadata, children);

    Ok(())
}
