use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use std::{error, fmt};

 
use sodiumoxide::crypto::secretstream::{Stream, Tag, KEYBYTES, HEADERBYTES, ABYTES};
use sodiumoxide::crypto::secretstream::xchacha20poly1305::{Header, Key};
use sodiumoxide::crypto::pwhash::{Salt, gen_salt, SALTBYTES, OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE};
use sodiumoxide::crypto::pwhash; 

use flate2::Compression; 
use flate2::read::DeflateEncoder;
use flate2::read::DeflateDecoder;
 
const ENCHANTED_NUM: [u8; 4] = [0x4D, 0x41, 0x47, 0x49]; //magi :) 
const CHUNK_SIZE: usize = 4096; 

#[derive(Debug)]
struct EncryptionErr { 
    message: String,
} 

impl EncryptionErr { 
    fn new(msg: &str) -> Self { 
        EncryptionErr{ 
            message: msg.to_string()
        }
    }
} 

impl fmt::Display for EncryptionErr { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}", self.message)
    }
} 

impl error::Error for EncryptionErr {} 

fn keygen(pass: &str, salt: &Salt ) -> Key { 
    let mut kpass = Key([0u8; KEYBYTES]);
    let Key(ref mut kb) = kpass;
    let (ops, mem) = (OPSLIMIT_SENSITIVE, MEMLIMIT_SENSITIVE);
    pwhash::derive_key(kb, pass.as_bytes(), &salt, ops, mem )
        .expect("failed to derive encryption key");  

    return kpass;
} 

pub fn verify(in_put: &mut File) -> bool {
    let fsize = in_put.metadata()
        .expect("\nFailed to retrieve metadata...")
        .len(); 

    if fsize < (ENCHANTED_NUM.len() + HEADERBYTES + SALTBYTES) as u64 { 
        return false;
    } 
    let mut magic = [0u8; ENCHANTED_NUM.len()]; 
    in_put.read_exact(&mut magic).expect("\nFailed to read magic numbers..."); 

    magic == ENCHANTED_NUM
} 

pub fn encrypt(input: &mut File, output: &mut File, pass: &str ) -> Result<(), Box<dyn error::Error>> { 
    input.seek(SeekFrom::Start(0))?; 
    let salt = gen_salt(); 
    let key = keygen(pass, &salt); 
    output.write(&ENCHANTED_NUM)?; 
    output.write(&salt.0)?; 

    let (mut stream, header) = Stream::init_push(&key)
        .map_err(|_| EncryptionErr::new("\nFailed to initialize encryption stream..."))?;  

    output.write(&header.0)?; 

    let mut remainder = input.metadata()?.len(); 
    let mut buffer = [0;CHUNK_SIZE]; 

    let mut input: Box<dyn std::io::Read> = Box::new(DeflateEncoder::new(input, Compression::fast()));

    loop {  
        match input.read(&mut buffer) { 
            Ok(bytes_read) if bytes_read > 0 => { 
                remainder -= bytes_read as u64; 
                let tag = match remainder { 
                    0 => Tag::Final, 
                    _ => Tag::Message,
                }; 
                let encrypted_bytes = &stream.push(&buffer[..bytes_read], None, tag) 
                    .map_err(|_| EncryptionErr::new("\nEncryption failed..."))?; 
                output.write(encrypted_bytes)?; 
                continue;
            }, 
            Err(e) => Err(e)?, 
            _ => break
        }
    }
    Ok(())
} 

pub fn decrypt(input: &mut File, output: &mut File, pass: &str ) -> Result<(), Box<dyn error::Error>> { 
    let mut salt = [0u8; SALTBYTES]; 
    input.read_exact(&mut salt)?; 
    let salt = Salt(salt); 
    let mut header = [0u8; HEADERBYTES]; 
    input.read_exact(&mut header)?; 
    let header = Header(header); 
    let key = keygen(pass, &salt);
     
    let mut stream = Stream::init_pull(&header, &key) 
        .map_err(|_| EncryptionErr::new("\nFailed to initialize decryption stream..."))?; 

    let mut buffer = [0u8; CHUNK_SIZE + ABYTES]; 

    let mut output: Box<dyn std::io::Write> = Box::new(DeflateDecoder::new(output)); 

    while stream.is_not_finalized() { 
        match input.read(&mut buffer) { 
            Ok(bytes_read) if bytes_read > 0 => { 
                let read = &buffer[..bytes_read]; 

                let (decrypted, _tag) = stream.pull(read, None)
                    .map_err(|_| EncryptionErr::new("\nIncorrect password..."))?; 
                
                output.write(&decrypted)?; 
                continue; 
            }, 
            Err(e) => Err(e)?, 
            _ => break
        }
    }

    Ok(())
}
