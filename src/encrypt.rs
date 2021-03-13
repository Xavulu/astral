use std::io::{Read, Write, Seek, SeekFrom};
use std::{error, fmt};
use std::path::Path;
use std::fs::{self};

use arrayvec::ArrayString;
use secrecy::Secret;



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




fn encrypt(in_put: Vec<u8>, pass: String) -> Result<Vec<u8>, Box<dyn error::Error> >{
    let encryption = age::Encryptor::with_user_passphrase(Secret::new(pass)); 
    let mut data: Vec<u8> = vec![]; 
    let mut writer = encryption.wrap_output(&mut data).unwrap(); 
    writer.write_all(&in_put)?;
    writer.finish()?;
    Ok(data)
} 


fn decrypt(in_put: Vec<u8>, pass: String)-> Result<Vec<u8>, Box<dyn error::Error>>{
    let decryption = match age::Decryptor::new(&in_put[..])? {
        age::Decryptor::Passphrase(p) => p, 
        _ => unreachable!(),
    }; 
    let mut data: Vec<u8> = vec![];
    let mut reader =  decryption.decrypt(&Secret::new(pass), None)?; 
    reader.read_to_end(&mut data)?;
    Ok(data)
}

pub fn encrypt_file(in_put: &Path, out_put: &Path, pass: String) -> Result<(), Box<dyn error::Error>>{
    let file_data = fs::read(in_put)?; 
    let encrypted = encrypt(file_data, pass)
        .map_err(|_| EncryptionErr::new("encryption failed :("))?; 
    fs::write(out_put, encrypted)?;
    Ok(())
}

pub fn decrypt_file(in_put: &Path, out_put: &Path, pass: String) -> Result<(), Box<dyn error::Error>>{
    let file_data = fs::read(in_put)?;
    let decrypted = decrypt(file_data, pass)
        .map_err(|_| EncryptionErr::new("decryption failed :("))?; 
    fs::write(out_put, decrypted)?;
    Ok(())
} 

pub fn b3_checksum(in_put: &Path) -> Result<ArrayString<[u8; 64]>, Box<dyn error::Error>>{
    let data = fs::read(in_put)?;
    let hashed = blake3::hash(&data); 
    let b3_hash = hashed.to_hex();

    Ok(b3_hash)
}

pub fn verify_b3_checksum(in_put: &Path, checksum: ArrayString<[u8; 64]>) -> Result<bool, Box<dyn error::Error>> {
    let data = fs::read(in_put)?;
    let hashed = blake3::hash(&data); 
    let b3_hash = hashed.to_hex();
    Ok(b3_hash == checksum)
}