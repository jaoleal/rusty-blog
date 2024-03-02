mod watcher;
use watcher::loader::Branch;
use std::path::PathBuf;
use std::env::current_dir;

fn main() { 
    let path_to_content =  PathBuf::from(current_dir().unwrap()).join(PathBuf::from("content/")); 
    let mut content_tree =  Branch::load_content(path_to_content);
}
