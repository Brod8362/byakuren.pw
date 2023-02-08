use std::fs;

pub fn all_pages() -> Vec<String> {
    match fs::read_dir("pages") {
        Ok(paths) => {
            paths.into_iter()
                .filter_map(|f| {
                    let mut t = String::from(f.unwrap()
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap());

                    if t.ends_with(".md") {
                        t.pop();
                        t.pop();
                        t.pop();
                        Some(t)
                    } else {
                        None
                    }
                })
                .collect()
        }
        Err(_) => panic!("can't determine page list")
    }
}