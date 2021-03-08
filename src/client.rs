// the client sends the file (astral projects the file if we going by the theme lmaoooo)
use super::helpers::{SentMsg, CanSend, enc_name_builder, EXTENSION};
use super::encrypt::{encrypt}; 

use message_io::events::{EventQueue}; 
use message_io::network::{Network, NetEvent, Transport}; 
use rpassword; 
use argon2::{self, Config, hash_encoded}; 


use std::fs::{self, File}; 
use std::io::{Read, Write};
use rand::Rng;

enum Event {
    Network(NetEvent),
    SendChunk,
}

pub fn astral_project(file_name: &str, destination: &str) {
    //password stuff: 
    let password = rpassword::prompt_password_stdout("enter a file password: ")
        .expect("couldn't get a password...."); 
    if password.len() < 10 {
        return println!("you should have a longer password....");
    }; 
    let confirm = rpassword::prompt_password_stdout("confirm password: ")
        .expect("couldn't get a password....");
    if confirm.len() != password.len() || confirm != password {
        return println!("your passwords dont match...");
    };

    let to_hash = password.as_bytes(); 
    let salt: Vec<u8> = (0..128).map(|_| { rand::random::<u8>() }).collect(); 
    println!("{:?}", salt);
    let config = argon2::Config::default();
    let hashed_pass = argon2::hash_encoded(to_hash, &salt, &config).unwrap();
    
    //encryption stuff:
    let mut file = File::open(file_name).unwrap(); 
    let enc_name = format!("{}{}", file_name, EXTENSION);
    let mut enc = File::create(&enc_name).unwrap();
    encrypt(&mut file, &mut enc, &password[..]).unwrap();
    return println!("okiedokie");


    //network stuff:
    let (mut network, mut event_queue) = 
        Network::split_and_map(|net_event| Event::Network(net_event)); 
    let server_addr = destination; 
    let(server_id, _) = match network.connect(Transport::FramedTcp, server_addr) {
        Ok(server_id) => {
            println!("Astral projecting {} to {} 🧘", file_name, destination);
            server_id
        }, 
        Err(_) => return println!("{} did not welcome your presence....", destination)
    }; 
    









}