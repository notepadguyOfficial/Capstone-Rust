use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
type Aes128Cbc = Cbc<Aes128, Pkcs7>;
use serde::{Serialize, Deserialize};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::error::Error;

pub const AES_IV: &[u8; 16] = b"abcdef0123456789";

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    m_timestamp: u64,
    m_polarity: bool,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    m_version: u32,
    m_timestamp_count: u64,
    m_timestamp_resolution: u64,
    m_size: u32,
    m_type: u32,
    m_aes_key: [u8; 16],
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub websocket_port: u16,
    pub http_port: u16,
    pub host: String,
}


#[allow(dead_code)]
pub fn write_encrypted_dat(path: &str, header: &Header, events: &[Event], settings: &Settings) -> Result<(), Box<dyn Error>> {
    let all_data = (header, events, settings);
    let serialized = bincode::serialize(&all_data)?;
    let cipher = Aes128Cbc::new_from_slices(&header.m_aes_key, AES_IV)?;
    let ciphertext = cipher.encrypt_vec(&serialized);
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?;
    file.write_all(&ciphertext)?;
    Ok(())
}

impl Header {
    #[allow(dead_code)]
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, Box<dyn Error>> {
        let m_version = reader.read_u32::<BigEndian>()?;
        let m_timestamp_count = reader.read_u64::<BigEndian>()?;
        let m_timestamp_resolution = reader.read_u64::<BigEndian>()?;
        let m_size = reader.read_u32::<BigEndian>()?;
        let m_type = reader.read_u32::<BigEndian>()?;
        let mut m_aes_key = [0u8; 16];
        reader.read_exact(&mut m_aes_key)?;
        
        Ok(Header {
            m_version,
            m_timestamp_count,
            m_timestamp_resolution,
            m_size,
            m_type,
            m_aes_key,
        })
    }

    #[allow(dead_code)]
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
        writer.write_u32::<BigEndian>(self.m_version)?;
        writer.write_u64::<BigEndian>(self.m_timestamp_count)?;
        writer.write_u64::<BigEndian>(self.m_timestamp_resolution)?;
        writer.write_u32::<BigEndian>(self.m_size)?;
        writer.write_u32::<BigEndian>(self.m_type)?;
        writer.write_all(&self.m_aes_key)?;
        Ok(())
    }
}

#[allow(dead_code)]
pub fn read_encrypted_dat(path: &str, aes_key: &[u8; 16]) -> Result<(Header, Vec<Event>, Settings), Box<dyn Error>> {
    let file = File::open(path);
    if let Ok(mut file) = file {
        let mut ciphertext = Vec::new();
        file.read_to_end(&mut ciphertext)?;
        let cipher = Aes128Cbc::new_from_slices(aes_key, AES_IV)?;
        let decrypted = cipher.decrypt_vec(&ciphertext)?;
        let (header, events, settings): (Header, Vec<Event>, Settings) = bincode::deserialize(&decrypted)?;
        store_settings_in_global_state(settings.clone());
        Ok((header, events, settings))
    } else {
        let default_header = Header {
            m_version: 1,
            m_timestamp_count: 0,
            m_timestamp_resolution: 1000,
            m_size: 9,
            m_type: 0,
            m_aes_key: *AES_IV,
        };
        let default_settings = Settings {
            websocket_port: 8080,
            http_port: 8000,
            host: "127.0.0.1".to_string(),
        };
        write_encrypted_dat(path, &default_header, &[], &default_settings)?;
        Ok((default_header, Vec::new(), default_settings))
    }
}

lazy_static::lazy_static! {
    static ref SETTINGS: std::sync::RwLock<Option<Settings>> = std::sync::RwLock::new(None);
}

#[allow(dead_code)]
pub fn store_settings_in_global_state(settings: Settings) {
    let mut settings_lock = SETTINGS.write().unwrap();
    *settings_lock = Some(settings);
}

#[allow(dead_code)]
pub fn get_settings() -> Option<Settings> {
    let settings_lock = SETTINGS.read().unwrap();
    settings_lock.clone()
}
