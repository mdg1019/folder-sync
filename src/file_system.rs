use std::path::PathBuf;
use std::fs;
use std::io::{stdin, Result};

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
 
                if dest_file_path.metadata()?.permissions().readonly() {
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