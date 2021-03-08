use std::path::Path;

use serde::{Serialize, Deserialize}; 

pub const EXTENSION: &str = ".astral"; 
pub struct FileInfo { 
    pub name: String, 
    pub encname: String,
    pub current: usize,
    pub encsize: usize, 
}  

#[derive(Serialize, Deserialize)]
pub enum SentMsg {
    Info(String, String, usize, usize),
    Data(Vec<u8>),
    Hash(Vec<u8>),
    Pass(String),
}

#[derive(Serialize, Deserialize)]
pub enum CanSend{
    PassCheck(bool),
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

