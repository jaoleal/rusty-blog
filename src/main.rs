mod watcher;
use watcher::content_parser::Branch;
use watcher::content_renderer::Renderer;
use std::path::PathBuf;
use std::env::current_dir;

fn main() { 
    let path_to_content =  PathBuf::from(current_dir().unwrap()).join(PathBuf::from("test_content/")); 
    let mut content_tree =  Branch::load_content(path_to_content).unwrap();
    let mut render = Renderer::new(&content_tree, PathBuf::from(current_dir().unwrap()).join(PathBuf::from("output/")));
    match render.render(None) {
        Ok(_) => {
            println!("Rendered content successfully");
        }
        Err(err) => {
            println!("Error rendering content: {:?}", err);
        }
    }
}
