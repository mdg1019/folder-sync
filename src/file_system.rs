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