use base64::{prelude::BASE64_STANDARD, Engine};
use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};
pub mod utils;

#[repr(u8)]
#[derive(Debug,PartialEq)]
pub enum ELogEntryType {
    UnInitialized ,
    X509LogEntryType,
    PrecertLogEntryType,
}
//     # MerkleTreeHeader = Struct(
//     #     "Version"         / Byte,
//     #     "MerkleLeafType"  / Byte,
//     #     "Timestamp"       / Int64ub,
//     #     "LogEntryType"    / Enum(Int16ub, X509LogEntryType=0, PrecertLogEntryType=1),
//     #     "Entry"           / GreedyBytes
//     # )
pub struct MerkleTreeHeader {
    pub version: u8,
    pub merkle_leaf_type: u8,
    pub timestamp: u64,
    pub log_entry_type: ELogEntryType,
    pub entry: Vec<u8>,
}
impl MerkleTreeHeader {
    pub fn new(data:&Vec<u8>) -> MerkleTreeHeader {
        let mut header_bytes = Cursor::new(data);
        let version = header_bytes.read_u8().unwrap();
        let merkle_leaf_type = header_bytes.read_u8().unwrap();
        let timestamp = header_bytes.read_u64::<BigEndian>().unwrap();
        let log_entry_type = match header_bytes.read_u16::<BigEndian>().unwrap() {
            0 => ELogEntryType::X509LogEntryType,
            1 => ELogEntryType::PrecertLogEntryType,
            _ => ELogEntryType::UnInitialized,
        };
        let mut entry = Vec::new();
        header_bytes.read_to_end(&mut entry).unwrap();
        MerkleTreeHeader {
            version,
            merkle_leaf_type,
            timestamp,
            log_entry_type,
            entry,
        }
    }

    pub fn new_b64(data:&String) -> MerkleTreeHeader {
        let header_bytes = BASE64_STANDARD.decode(data).expect("Failed to decode Leaf Header Base64 data.");
        return Self::new(&header_bytes);
    }
    
    
}
// # Certificate = Struct(
//     #     "Length" / Int24ub,
//     #     "CertData" / Bytes(this.Length)
//     # )
pub struct Certificate{
    pub length: u32,
    pub cert_data: Vec<u8>,
}
impl Certificate {
    pub fn new(data:&Vec<u8>) -> Certificate {
        let mut certificate_bytes = Cursor::new(data);
        let length = certificate_bytes.read_u24::<BigEndian>().unwrap();
        let mut cert_data = Vec::new();
        certificate_bytes.read_to_end(&mut cert_data).unwrap();
        Certificate {
            length:length,
            cert_data:cert_data
        }
        
    }
}
// # CertificateChain = Struct(
//     #     "ChainLength" / Int24ub,
//     #     "Chain" / GreedyRange(Certificate),
//     # )
pub struct CertificateChain{
    chain_length:u32,
    chain:Vec<Certificate>
}
impl CertificateChain {
    pub fn new(data:&Vec<u8>)->CertificateChain{
        let mut cursor = Cursor::new(data);
        let chain_length : u32 = cursor.read_u24::<BigEndian>().unwrap();
        let mut chain : Vec<Certificate> = Vec::new(); 
        let cur_length = cursor.get_ref().len();
        while cursor.position() < cur_length as u64 {
            let cert_length = cursor.read_u24::<BigEndian>().unwrap();
            let mut cert_data : Vec<u8> = Vec::with_capacity(cert_length as usize);
            cert_data.resize(cert_length as usize, 0u8);
            cursor.read_exact(&mut cert_data).unwrap();
            let cert : Certificate = Certificate {
                length: cert_length,
                cert_data:cert_data,
            };
            chain.push(cert);
        }
        return CertificateChain{
            chain_length:chain_length,
            chain:chain
        }
       
    }
    pub fn new_b64(data:&String)->CertificateChain{
        let bytes  = BASE64_STANDARD.decode(data).expect("Unable to decode Certificate Chain");
        return Self::new(&bytes);
    }
    
}
// # PreCertEntry = Struct(
//     #     "LeafCert" / Certificate,
//     #     Embedded(CertificateChain),
//     #     Terminated
//     # )
pub struct PreCertEntry{
    leaf_cert:Certificate,
    chain:CertificateChain
} 
impl PreCertEntry {
    pub fn new(data:&Vec<u8>)->PreCertEntry{
        let mut cursor = Cursor::new(data);
        let leafcert_length = cursor.read_u24::<BigEndian>().unwrap();
        let mut cert_data:Vec<u8> = Vec::with_capacity(leafcert_length as usize);
        cert_data.resize(leafcert_length as usize, 0u8);
        cursor.read_exact(&mut cert_data).unwrap();
        let cert:Certificate = Certificate{length:leafcert_length,cert_data:cert_data};
        let mut chain_data:Vec<u8> = Vec::new();
        cursor.read_to_end(&mut chain_data).unwrap();
        let chain:CertificateChain = CertificateChain::new(&chain_data);
        return PreCertEntry{leaf_cert:cert,chain:chain};
    }
    pub fn new_b64(data:&String)->PreCertEntry{
        let bytes = BASE64_STANDARD.decode(&data).unwrap();
        return Self::new(&bytes);
    }
    
}