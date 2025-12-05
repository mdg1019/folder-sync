mod utils;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().collect::<Vec<String>>();

    let (source_path, destination_path) = utils::parse_args(args)?;
    println!("Source path: {:?}", source_path);
    println!("Destination path: {:?}", destination_path);

    Ok(())
}
