use super::encrypt::*;
use super::helpers::{FileInfo, SentMsg, CanSend};

use message_io::events::{EventQueue};
use message_io::network::{Network, NetEvent, Endpoint, Transport};
use rpassword;


use std::collections::{HashMap};
use std::fs::{File};
use std::io::{Write};




pub fn astral_plane (port: &str) {
    let (mut network, mut event_queue) = Network::split(); 
    let addr = format!("127.0.0.1:{}", port); 
    let server_addr = &addr[..]; 
    match network.listen(Transport::FramedTcp, server_addr) {
        Ok(_) => println!("Running on {}", server_addr), 
        Err(_) => return println!("failed lol"),
    }

}