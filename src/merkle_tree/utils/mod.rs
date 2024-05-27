use std::collections::HashMap;
use crate::merkle_tree::{MerkleTreeHeader, ELogEntryType,Certificate,CertificateChain};
use openssl::x509::X509;
use crate::generic_utils::Entry;
use openssl::nid::Nid;


use super::PreCertEntry;


pub async fn read_entry(entry:&Entry) -> HashMap<String,Vec<String>>{
    let mkl = MerkleTreeHeader::new_b64(&entry.leaf_input);
    let mut return_data : HashMap<String,Vec<String>> = HashMap::new();
    if mkl.log_entry_type == ELogEntryType::X509LogEntryType {
        let leaf_cert = X509::from_der(& Certificate::new(&mkl.entry).cert_data[0..]).unwrap();
        let certificate_chain = CertificateChain::new_b64(&entry.extra_data);
        let mut chain:Vec<X509> = Vec::new(); 
        for cert in certificate_chain.chain{
            let temp_cert = X509::from_der(&cert.cert_data).unwrap();
            chain.push(temp_cert);
        }
        return_data = read_domains(&leaf_cert,&chain).await;
    }
    else if mkl.log_entry_type == ELogEntryType::PrecertLogEntryType {
        let extra_data = PreCertEntry::new_b64(&entry.extra_data);
        let leaf_cert = X509::from_der(&extra_data.leaf_cert.cert_data).unwrap();
        let mut chain :Vec<X509> = Vec::new();
        for cert in extra_data.chain.chain{
            let temp_cert = X509::from_der(&cert.cert_data).unwrap();
            chain.push(temp_cert);
        }
        return_data = read_domains(&leaf_cert,&chain).await;
    }
    return return_data;
}
pub async fn read_domains(leaf_cert:&X509,cert_chain:&Vec<X509>)->HashMap<String,Vec<String>>{
    let mut all_domains:Vec<String> = Vec::new();
    let mut leaf_domain:Vec<String> = Vec::new();
    match leaf_cert.subject_name().entries_by_nid(Nid::COMMONNAME).next()
    {
        Some(domain )=>leaf_domain.push(domain.data().as_utf8().unwrap().to_string()),
        None=>{},

    };
    match leaf_cert.subject_alt_names(){
        Some(domains)=>{
            for domain in domains{
                if domain.dnsname().is_some(){
                    all_domains.push(domain.dnsname().unwrap().to_string())
                }
            }
        }
        None=>{},
    };
    for cert in cert_chain{
        match cert.subject_alt_names(){
            Some(domains)=>{
                let all_domains_size = all_domains.len();
                for domain in domains{
                    if domain.dnsname().is_some(){
                        all_domains.push(domain.dnsname().unwrap().to_string());
                    }
                }
                if all_domains.len() > all_domains_size{
                    match cert.subject_name().entries_by_nid(Nid::COMMONNAME).next(){
                        Some(domain) => leaf_domain.push(domain.data().as_utf8().unwrap().to_string()),
                        None => {}
                    };
                }
            },
            None=>{}
        };
    }
    return HashMap::from([
        ("leaf".to_string(),leaf_domain),
        ("all_domains".to_string(),all_domains)
    ]);
}
    


