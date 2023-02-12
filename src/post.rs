use std::{fs::{File, self}, io::BufReader, io::{BufRead, Lines}};

use comrak::{ComrakOptions, Arena, parse_document, format_html};
use rocket::{http::Status};
use serde::Serialize;

#[derive(Serialize)]
pub struct PostInfo {
    pub title: String,
    pub timestamp: i64,
    pub html: String,
    pub id: String
}

fn parse_from_reader(lines: &mut Lines<BufReader<File>>) -> (String, i64) {
    let title_line = lines.next().unwrap().unwrap();
    let timestamp_line: i64 = lines.next().unwrap().unwrap().parse().unwrap_or(0);
    (title_line, timestamp_line)
}

pub fn parse_min(filename: &String) -> Result<PostInfo, Status> {
    let path = format!("pages/{}.md", filename);
    let fd = match File::open(path) {
        Ok(p) => p,
        Err(_) => return Err(Status::NotFound)
    };

    let reader = BufReader::new(fd);
    let mut lines = reader.lines();
    
    let res = parse_from_reader(&mut lines);
    Ok(PostInfo {
        title: res.0,
        timestamp: res.1,
        html: String::new(),
        id: filename.clone()
    })
}

pub fn parse_full(filename: &String) -> Result<PostInfo, Status> {
    let arena = Arena::new();
    let path = format!("pages/{}.md", filename);


    let fd = match File::open(path) {
        Ok(p) => p,
        Err(_) => return Err(Status::NotFound)
    };

    let reader = BufReader::new(fd);
    let mut lines = reader.lines();
    
    let (title, timestamp) = parse_from_reader(&mut lines);

    let mut content = String::new();
    //TODO is there a better way?
    for rl in lines {
        content = content + &rl.unwrap() + "\n";
    }
    let root = parse_document(&arena, &*content, &ComrakOptions::default());

    let mut html = vec![];
    format_html(root, &ComrakOptions::default(), &mut html).unwrap();
    Ok(PostInfo {
        title: title,
        timestamp: timestamp,
        html: String::from_utf8(html).unwrap(),
        id: filename.clone()
    })
}

pub fn all_min() -> Vec<PostInfo> {
    match fs::read_dir("pages") {
        Ok(paths) => {
            let mut posts: Vec<PostInfo> = paths.into_iter()
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
                        match parse_min(&t) {
                            Ok(r) => Some(r),
                            Err(_) => None
                        }
                    } else {
                        None
                    }
                })
                .collect();
                posts.sort_by(|a, b| b.timestamp.partial_cmp(&a.timestamp).unwrap());
                posts
        }
        Err(_) => panic!("can't determine page list")
    }
}