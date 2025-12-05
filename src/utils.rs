use std::path::PathBuf;

pub fn parse_args(args: Vec<String>) -> Result<(Box<PathBuf>, Box<PathBuf>), String> {    
    if args.len() < 3{

        return Err("Use as: folder-sync <source> <destination>".into());
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

    Ok((Box::from(source_path), Box::from(destination_path)))
}   