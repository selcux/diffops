// use std::path::PathBuf;
// use walkdir::{DirEntry, WalkDir};
use std::fs::{File, OpenOptions};
use std::error::Error;
use std::io::{ErrorKind, BufWriter, BufReader};
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::env;

const RESOURCES_FILE: &str = "resources.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource(String, String);

impl Resource {
    pub fn new() -> Self {
        Resource(String::new(), String::new())
    }

    pub fn put(&self) -> Result<(), Box<dyn Error>> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(RESOURCES_FILE)?;
        let f = BufWriter::new(f);

        match serde_json::to_writer(f, &self) {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn get() -> Result<Self, Box<dyn Error>> {
        let f = match File::open(RESOURCES_FILE) {
            Ok(file) => file,
            Err(err) => return match err.kind() {
                ErrorKind::NotFound => Ok(Resource::new()),
                _ => Err(err.into()),
            }
        };
        let f = BufReader::new(f);

        match serde_json::from_reader(f) {
            Ok(resource) => Ok(resource),
            Err(err) => Err(err.into()),
        }
    }

    pub fn set_reference(&mut self, dir_name: &str) -> Result<(), Box<dyn Error>> {
        let path = Self::dir_path(dir_name)?;
        self.0 = path;
        Ok(())
    }

    pub fn reference(&self) -> &str { &self.0.as_str() }

    pub fn set_target(&mut self, dir_name: &str) -> Result<(), Box<dyn Error>> {
        let path = Self::dir_path(dir_name)?;
        self.1 = path;
        Ok(())
    }

    pub fn target(&self) -> &str { &self.1.as_str() }

    fn dir_path(dir_name: &str) -> Result<String, Box<dyn Error>> {
        let path = Path::new(dir_name);

        if path.exists() && path.is_dir() {
            return Ok(path.to_str().unwrap().to_string());
        }

        let current_dir = env::current_dir()?;
        let pb = current_dir.join(path);
        if pb.exists() && pb.is_dir() {
            return Ok(pb.to_str().unwrap().to_string());
        }

        panic!("Invalid directory: {}", dir_name)
    }
}
/*
enum State {
    Identical,
    Deleted,
    Created,
    Copied,
}

pub fn traverse_folders(current_dir: &PathBuf) {
    WalkDir::new(current_dir)
        .into_iter()
        .filter_entry(is_not_hidden)
        .filter_map(|v| v.ok())
        .for_each(operate)
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn operate(e: DirEntry) {
    println!("{}", e.path().display())
}
*/