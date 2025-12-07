use std::path::PathBuf;

pub fn parse_args(args: &Vec<String>) -> Result<(Box<PathBuf>, Box<PathBuf>, bool), String> {    
    if args.len() < 3{

        return Err("Use as: folder-sync <source> <destination> [-r]".into());
    }

    let source = &args[1];
    let destination = &args[2];

    let source_path = PathBuf::from(source);
    let destination_path = PathBuf::from(destination);

    if source_path.exists() == false {
        return Err(format!("Source folder {} does not exist", source).into());
    }

    if source_path.is_file() {
        return Err(format!("Source {} must be a folder", source).into());
    }

    if args.len() == 4 {
        if args[3] != "-r" {
            return Err("Invalid option. Only '-r' is supported.".into());
        }

        return Ok((Box::from(source_path), Box::from(destination_path), true));

    }

    Ok((Box::from(source_path), Box::from(destination_path), false))
}   