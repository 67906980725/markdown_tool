use std::ffi::OsStr;
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

pub fn main(args: Vec<String>) {
  if args.len() < 3 {
    panic!("need 1 param by MD_PATHS");
  }
  args[2].split(",")
        .map(|path| path.trim())
        .for_each(|path| trans_md_pics_to_local(&path));
}

pub fn trans_md_pics_to_local(md_path: &str) {
  // println!("find md local image: {}", &md_path);
  let origin_file_name = Path::new(&md_path).file_name().unwrap().to_str().unwrap();
  let new_file_name = origin_file_name.replace(" ", "");
  let mut file_name = String::new();
  file_name.push_str(&new_file_name);
  let file_name_less_suffix = Path::new(&file_name).file_stem().unwrap().to_str().unwrap();

  let file = File::open(md_path).unwrap();
  let buf_reader = BufReader::new(file);

  let parent_path = Path::new(md_path).parent().unwrap();
  let parent_path_name = parent_path.to_str().unwrap();
  let out_path = Path::new(&parent_path_name).join("out");
  let out_path_name = out_path.to_str().unwrap();
  mkdir(&out_path_name);

  let new_file_path = out_path.join(&file_name);
  let new_file = File::create(new_file_path).unwrap();
  let mut writer = LineWriter::new(BufWriter::new(new_file));

  buf_reader.lines()
    .filter(|line_r| line_r.is_ok())
    .map(|line_r| trans_line_pics_to_local(line_r.unwrap().as_str(),&out_path_name, & &file_name_less_suffix))
    .for_each(|line| {
      writer.write_all(line.as_bytes()).unwrap();
      writer.write_all("\n".as_bytes()).unwrap();
    });
  writer.flush().unwrap()
}

pub fn mkdir(path: &str) {
  if !fs::metadata(path).is_ok() {
    fs::create_dir_all(path).unwrap()
  }
}

pub fn trans_line_pics_to_local(line: &str, parent_path_name: &str, md_name: &str) -> String {
  let re: Regex = Regex::new(r"!\[.*?]\((.*?)\)").unwrap();
  re.replace_all(&line, |capture: &regex::Captures| {
    trans_line_one_pic_to_local(capture, &parent_path_name, &md_name)
  }).to_string()
}

pub fn trans_line_one_pic_to_local(capture: &regex::Captures,
                                   parent_path_name: &str,
                                   md_name: &str) -> String {
  let url = match capture.get(1) {
    Some(matched) => matched.as_str(),
    None => return String::default(),
  };
  let uri = match Url::parse(url) {
    Ok(uri) => uri,
    Err(_) => return String::from(url),
  };
  let pic_origin_name = match uri.path_segments().and_then(Iterator::last) {
    Some(s) => s,
    None => return String::from(url),
  };
  let pic_suffix = Path::new(&pic_origin_name).extension().and_then(OsStr::to_str).unwrap_or("");
  let uuid = Uuid::new_v4().to_string();
  let f_name_new = format!("{}.{}", uuid, &pic_suffix);

  let asset_dir_name = ".asset";
  let parent_path = Path::new(parent_path_name);
  let asset_path = parent_path.clone().join(&asset_dir_name).join(&md_name);
  mkdir(asset_path.clone().to_str().unwrap());

  let file_path = asset_path.join(&f_name_new);
  down(&url, &file_path);

  let url_new = format!("./{}/{}/{}", &asset_dir_name, &md_name, &f_name_new);
  format!("![]({})", &url_new)
}

pub fn down<P: AsRef<Path>>(url: &str, file_path: P) {
  let response = get(url).unwrap();
  let mut file = File::create(file_path).unwrap();
  copy(&mut response.bytes().unwrap().as_ref(), &mut file).unwrap();
}