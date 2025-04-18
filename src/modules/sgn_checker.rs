use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignatureList {
    pub extension: String,
    pub description: String,
}

pub fn search() -> io::Result<()> {
    let sig_list = std::fs::read_to_string("src/assets/signatures.toml").expect("[!] Error trying to access the signatures list file");

    let rtable: toml::Value = toml::from_str(&sig_list).expect("[!] Unable to parse the signatures.");

    let mut sigmap: HashMap<Vec<u8>, Vec<SignatureList>> = HashMap::new();

    for (key, value) in rtable.as_table().unwrap() {
        let entries: Vec<SignatureList> = value
            .as_array().unwrap()
            .iter()
            .map(|v| v.clone().try_into().unwrap()).collect();

        let sigbytes = if let Ok(single) = key.parse::<u8>() {
            vec![single]
        } else {
            key.split_whitespace()
                .map(|byte| u8::from_str_radix(byte, 16).expect("[!] Invalid signature key")).collect()
        };

        sigmap
            .entry(sigbytes)
            .or_default()
            .extend(entries);
    }

    let mut input = String::new();
    print!("[>] Enter the path of the file you want to examine:");
    io::stdout().flush().unwrap(); 
    io::stdin().read_line(&mut input).expect("[!] Failed to read input");
    let path = input.trim();

    let mut file = File::open(path)?;
    let mut buffer = [0u8; 32];
    let _ = file.read(&mut buffer)?;

    let mut matched = false;

    for (sig_bytes, entries) in &sigmap {
        if buffer.starts_with(sig_bytes) {
            for entry in entries {
                println!(
                    "[✓] Matched Signature: {:02X?}\nExtension: {}\nDescription: {}",
                    sig_bytes, entry.extension, entry.description
                );
            }
            matched = true;
        }
    }

    if !matched {
        println!("[!] No matching signature found.");
    }

    Ok(())
}
