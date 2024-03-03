pub mod content_parser {
    use std::any::type_name;
    use std::path::PathBuf;
    
    use anyhow::Result;

    #[derive(Debug)]
    pub struct Branch {
        pub path: PathBuf, 
        pub taipe: ContentCategories,
        pub children: Option<Vec<Branch>>,
    }
    #[derive(Debug)]
    pub enum ContentCategories {
        Section(String),
        Post(String),
    }

    impl Branch{
        pub fn load_content(path: PathBuf)-> Result<Vec<Branch>> {
            let mut ret =  Vec::<Branch>::new();
            let path_content = path.read_dir();
            match path_content {
                Ok(entries) => {
                    entries.for_each(|entry| {
                        let path = entry.unwrap().path();
                        if path.is_dir() {
                            ret.push(Branch {
                                path: path.clone(),
                                taipe: ContentCategories::Section(path.clone().file_name().unwrap().to_str().unwrap().to_string()),
                                children: Some(Self::load_content(path.clone().join(path)).unwrap())
                            });
                        } else {
                            ret.push(Branch {
                                path: path.clone(),
                                taipe: ContentCategories::Post(path.file_name().unwrap().to_str().unwrap().to_string()),
                                children: None,
                            });
                        }
                    });
                    Ok(ret)
                }
                Err(err) => {
                    Err(anyhow::Error::new(err))
                }
            }
        }
    }
}
pub mod blog{
    use yew::Html;

    pub enum html{
        Post(String)
    }
}
pub mod content_renderer {
    use crate::watcher::content_parser::Branch;
    use std::{any::{type_name, Any}, fs, io::{Read, Write}, path::PathBuf};
    use anyhow::{Ok, Result};
    use markdown::to_html;
    pub struct Renderer<'a> {
        pub content_tree: &'a Vec<Branch>,
        pub output_path: PathBuf,
    }
    impl Renderer<'_>{
        pub fn new(content_tree: &Vec<Branch>, output_path: PathBuf) -> Renderer<'_> {    
            Renderer {
                content_tree,
                output_path,
            }
        }
        pub fn inner_md_to_hmtl(&self, html_string: String) -> String {
            /* 
                This function ensures that all the links to markdown files are replaced with links to a html file            
             */
            let ret = html_string.replace(".md", ".html");
            ret
        }
        pub fn render(&self, section: Option<String>) -> Result<()> {
            for branch in self.content_tree {
                match &branch.children {
                    Some(children) => {
                        let mut path = self.output_path.clone();
                        path.push(&branch.path);
                        std::fs::create_dir_all(&path)?;
                        let renderer = Renderer::new(children, path.clone());
                        renderer.render( Some(path.to_str().unwrap().to_string()))?;
                    }
                    None => {
                        let mut md_content = String::new();
                        let mut html_content = String::new();
                        
                        if branch.path.file_name().unwrap().to_str().unwrap().ends_with(".md") {
                            match std::fs::File::open(&branch.path) {
                                Result::Ok(mut file) => {
                                    file.read_to_string(&mut md_content)?;
                                    html_content = to_html(&md_content);
                                    html_content = self.inner_md_to_hmtl(html_content); 
                                    let mut output_path = self.output_path.clone();
                                    output_path.push(&branch.path);
                                    output_path.set_extension("html");
                                    let mut file = fs::File::create(output_path)?;
                                    file.write_all(html_content.as_bytes())?;
                                }
                                Err(err) => {
                                    return Err(anyhow::Error::new(err));
                                }
                                
                            }
                        }
                    }
                };
            };
            Ok(())
        }
    }
}