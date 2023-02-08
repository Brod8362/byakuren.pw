use std::fs;
use std::os::unix::fs::MetadataExt;
use rocket_dyn_templates::serde::Serialize;

pub fn all_pages() -> Vec<(String, i64)> {
    match fs::read_dir("pages") {
        Ok(paths) => {
            paths.into_iter()
                .filter_map(|f| {
                    let dir = f.unwrap();
                    let mut t = String::from(dir
                        .path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap());

                    if t.ends_with(".md") {
                        t.pop();
                        t.pop();
                        t.pop();
                        let metadata = fs::metadata(dir.path()).unwrap();
                        Some((t, metadata.ctime()))
                    } else {
                        None
                    }
                })
                .collect()
        }
        Err(_) => panic!("can't determine page list")
    }
}