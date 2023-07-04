use std::env;

use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufWriter, LineWriter, Write};
use std::io::copy;
use std::io::prelude::*;

use uuid::Uuid;
use regex::Regex;
use reqwest::blocking::get;
use reqwest::Url;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("need 1 param by MD_PATHS");
    }

    let arg_to_full_paths = |arg: &str|
        arg.split(",")
            .map(|path| path.trim())
            .for_each(|path| trans_md_pics_to_local(&path));

    // arg_to_full_paths(r"C:\Users\v\Downloads\html2markdown.md");
    arg_to_full_paths(&args[1])
}

pub fn trans_md_pics_to_local(md_path: &str) {
    // println!("find md local image: {}", &md_path);
    let file_name = Path::new(&md_path).file_name().unwrap().to_str().unwrap();
    let file = File::open(md_path).unwrap();
    let buf_reader = BufReader::new(file);

    let parent_path = Path::new(md_path).parent().unwrap();
    let parent_path_name = parent_path.to_str().unwrap();
    let out_path = Path::new(&parent_path_name).join("out");
    let out_path_name = out_path.to_str().unwrap();
    mkdir(&out_path_name);

    let new_file_path = out_path.join(file_name);
    let new_file = File::create(new_file_path).unwrap();
    let mut writer = LineWriter::new(BufWriter::new(new_file));

    buf_reader.lines()
        .filter(|line_r| line_r.is_ok())
        .map(|line_r| trans_line_pics_to_local(&out_path_name, &line_r.unwrap()))
        .for_each(|line| {
            writer.write_all(line.as_bytes()).unwrap();
            writer.write_all("\n".as_bytes()).unwrap();
        });
    writer.flush().unwrap()
}

pub fn trans_line_pics_to_local(parent_path_name: &str, line: &str) -> String {
    let re: Regex = Regex::new(r"!\[]\(([^)]+)\)").unwrap();
    re.replace_all(line, |capture: &regex::Captures| {
        let url: &str = capture.get(1).unwrap().as_str();

        let uuid = Uuid::new_v4().to_string();
        let mut url_parsed = Url::parse(url).expect("Failed to parse URL");
        let f_name_origin = url_parsed.path_segments().and_then(Iterator::last).unwrap();
        let suffix = Path::new(f_name_origin).extension().unwrap().to_str().unwrap();
        let f_name_new = format!("{}.{}", uuid, suffix);

        let asset_dir_name = ".asset";
        let parent_path = Path::new(parent_path_name);
        let asset_path = parent_path.clone().join(&asset_dir_name);
        mkdir(asset_path.clone().to_str().unwrap());

        let file_path = asset_path.join(&f_name_new);
        down(&url, &file_path);

        let url_new = format!("./{}/{}", &asset_dir_name, &f_name_new);
        format!("![]({})", &url_new)
    }).to_string()
}


pub fn mkdir(path: &str) {
    if !fs::metadata(path).is_ok() {
        fs::create_dir(path).unwrap()
    }
}

pub fn down<P: AsRef<Path>>(url: &str, file_path: P) {
    let response = get(url).unwrap();
    let mut file = File::create(file_path).unwrap();
    copy(&mut response.bytes().unwrap().as_ref(), &mut file).unwrap();
}