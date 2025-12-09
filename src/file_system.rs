use std::path::PathBuf;
use std::fs;
use std::io::{stdin, Result};
use std::collections::HashSet;

#[derive(Debug)]
pub enum Node {
    File {
        name: String,
        path: PathBuf,
        metadata: fs::Metadata,
    },
    Directory {
        name: String,
        path: PathBuf,
        metadata: fs::Metadata,
        children: Vec<Node>,
    },
}

pub fn build_tree(path: PathBuf) -> std::io::Result<Node> {
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().to_string());

    let metadata = fs::metadata(&path)?;

    if metadata.is_file() {
        Ok(Node::File { name, path, metadata })
    } else {
        let mut children = Vec::new();

        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            children.push(build_tree(entry.path())?);
        }

        Ok(Node::Directory { name, path, metadata,children })
    }
}

pub fn create_destination_dir_if_not_exists(destination_path: &PathBuf) -> Result<bool> {
    if !destination_path.exists() {
        println!(
            "Destination path: '{}' does not exist. Do you want to create it? (y/n)?", destination_path.display());

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "y" {
            println!("Exiting...");
            return Ok(false);
        } 

        println!("Creating destination directory...");
        fs::create_dir_all(&destination_path)?;           
    }

    Ok(true)
}

pub fn get_node_info(node: &Node) -> (&str, &String, &PathBuf, &fs::Metadata, Option<&Vec<Node>>) {
    match node {
        Node::File { name, path, metadata } => ("File", name, path, metadata, None),
        Node::Directory { name, path, metadata, children } => ("Directory", name, path, metadata, Some(children)),
    }
}

pub fn copy_files(children: &Vec<Node>, dest_path: &PathBuf) -> Result<()> {
    for src_node in children {
        match src_node {
            Node::File { name, path, .. } => {
                let dest_file_path = dest_path.join(name);

                if dest_file_path.metadata().is_ok() {
                    let src_metadata = fs::metadata(path)?;
                    let dest_metadata = fs::metadata(&dest_file_path)?;

                    if src_metadata.modified()? <= dest_metadata.modified()? {
                        println!("Skipping file (up-to-date): {}", dest_file_path.display());
                        continue;
                    }
                }
 
                if dest_file_path.exists() && dest_file_path.metadata()?.permissions().readonly() {
                    fs::remove_file(&dest_file_path)?;
                }
 
                fs::copy(path, &dest_file_path)?;

                println!("Copied file: {} to {}", path.display(), dest_file_path.display());
            }
            Node::Directory { name, children, .. } => {
                let new_dest_path = dest_path.join(name);

                if !fs::metadata(&new_dest_path).is_ok() {
                    fs::create_dir_all(&new_dest_path)?;
                    println!("Created directory: {}", new_dest_path.display());
                }
                
                copy_files(children, &new_dest_path)?;
            }
        }
    }

    Ok(())
}

pub fn remove_extra_files(src_children: &Vec<Node>, dest_path: &PathBuf) -> Result<()> {
    let mut src_names: HashSet<String> = HashSet::new();

    for src_node in src_children {
        let (_, name, _, _, _) = get_node_info(src_node);
        src_names.insert(name.clone());
    }

    if let Ok(entries) = fs::read_dir(dest_path) {
        for entry in entries {
            let entry = entry?;
            let dest_name = entry.file_name().to_string_lossy().to_string();
            let dest_path = entry.path();

            if !src_names.contains(&dest_name) {
                if dest_path.is_dir() {
                    fs::remove_dir_all(&dest_path)?;
                    println!("Removed directory: {}", dest_path.display());
                } else {
                    fs::remove_file(&dest_path)?;
                    println!("Removed file: {}", dest_path.display());
                }
            } else {
                if dest_path.is_dir() {
                    for src_node in src_children {
                        if let Node::Directory { name, children, .. } = src_node {
                            if name == &dest_name {
                                remove_extra_files(children, &dest_path)?;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}



// pub fn print_tree(node: &Node, indent: usize) {
//     match node {
//         Node::File { name, .. } => {
//             println!("{}📄 {}", " ".repeat(indent), name);
//         }
//         Node::Directory { name, children, .. } => {
//             println!("{}📁 {}", " ".repeat(indent), name);
//             for child in children {
//                 print_tree(child, indent + 2);
//             }
//         }
//     }
// }