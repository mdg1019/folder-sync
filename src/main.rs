mod file_system;
mod utils;
use std::env::args;

use crate::file_system::{copy_files, create_destination_dir_if_not_exists};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = args().collect::<Vec<String>>();

    let args = vec!["folder-sync".to_string(), "/home/mark/test-src".to_string(), "/home/mark/test-dest".to_string()];

    let (src_path, dest_path) = utils::parse_args(&args)?;

    println!("Building file tree for source path...");
    let src_tree = file_system::build_tree(*src_path.clone())?;

    let dir_exists = create_destination_dir_if_not_exists(&dest_path)?;

    if !dir_exists {
        return Ok(());
    }


    let (_, _, _, _, children) = file_system::get_node_info(&src_tree);

    copy_files(children.unwrap(), &dest_path)?;

    println!("Building file tree for destination path...");
    let dest_tree = file_system::build_tree(*dest_path.clone())?;

    Ok(())
}
