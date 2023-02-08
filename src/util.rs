use std::fs;
use std::os::unix::fs::MetadataExt;

pub fn all_pages() -> Vec<(String, i64)> {
    match fs::read_dir("pages") {
        Ok(paths) => {
            let mut posts: Vec<(String, i64)> = paths.into_iter()
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
                .collect();
                posts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                posts
        }
        Err(_) => panic!("can't determine page list")
    }
}