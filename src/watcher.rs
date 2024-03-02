pub mod loader {
    use std::path::PathBuf;
    use anyhow::Result;

    #[derive(Debug)]
    pub struct Branch {
        path: PathBuf, 
        taipe: ContentCategories,
        children: Option<Vec<Branch>>,
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