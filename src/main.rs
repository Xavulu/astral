

mod encrypt; 
mod server; 
mod client;
mod helpers;


use std::io::{self, Read, Write};
use client::{astral_project}; 
use server::{astral_plane}; 
use walkdir::{DirEntry, WalkDir};
use helpers::{is_hidden};
use std::path::Path;
use encrypt::{b3_checksum, verify_b3_checksum};

const APP_NAME: &str = r"˜”*°• 𝒂𝒔𝒕𝒓𝒂𝒍 •°*”˜"; 
 




fn main() {
    //sodiumoxide::init().unwrap();
    print!("\n{} : {}\n", APP_NAME, APP_NAME.len());
    /* 
    let walker = WalkDir::new("/Users/lain/compsci/rust-apps/astral").follow_links(false).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        match entry.path().is_file() {
            true => println!("{}", entry.path().display()), 
            _ => continue,
        };
    }*/ 
    let example = Path::new("Cargo.toml"); 
    let hash = b3_checksum(example).unwrap(); 
    let verify = verify_b3_checksum(example, hash).unwrap();
    println!("{}", hash);
    println!("{}", verify);
    
}
