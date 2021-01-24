use std::path::Path;

use serde::{Serialize, Deserialize}; 

const EXTENSION: &str = ".astral"; 

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct FileInfo { 
    name: String, 
    size: usize,
}  

pub fn enc_name_builder(infile: &Path) -> String{ 
    let origin = Path::new(& infile).file_name().unwrap().to_str().unwrap(); 
    let outfile = format!("{}{}", origin, String::from(EXTENSION)); 
    return outfile;
}

pub fn dec_name_builder(infile: &Path) -> String{ 
    let origin = infile.to_str().unwrap().to_string(); 
    let stripped = format!("{}", origin[..origin.len() - EXTENSION.len()].to_string()); 
    return stripped;
}

