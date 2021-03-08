

mod encrypt; 
mod server; 
mod client;
mod helpers;


use std::io::{self, Read, Write};
use client::{astral_project}; 
use server::{astral_plane}; 


const APP_NAME: &str = r"˜”*°• 𝒂𝒔𝒕𝒓𝒂𝒍 •°*”˜"; 
 
fn main() {
    print!("\n{} : {}\n", APP_NAME, APP_NAME.len());
    client::astral_project("./Cargo.toml", "no re");
    let mut test = std::fs::File::create("./hello.toml").unwrap();
    let mut enc = std::fs::File::open("./Cargo.toml.astral").unwrap();
    let pass = rpassword::prompt_password_stdout("enter: ").expect("nothing given...");
    encrypt::decrypt(&mut enc, &mut test, &pass[..]).unwrap();
    
}
