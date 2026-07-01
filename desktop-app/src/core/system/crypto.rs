use std::fs;
use sha2::{Sha256, Digest};

fn derive_key() -> [u8; 32] {
    let id = fs::read_to_string("/etc/machine-id")
        .or_else(|_| fs::read_to_string("/var/lib/dbus/machine-id"))
        .unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(b"WorldServers-agent-key-v1:");
    hasher.update(id.trim().as_bytes());
    hasher.finalize().into()
}

pub fn obfuscate(s: &str) -> String {
    let key= self::derive_key();
    let bytes: Vec<u8> = s.bytes().enumerate().map(|(i, b)| b ^ key[i % key.len()]).collect();
    encode(&bytes)
}

pub fn deobfuscate(s: &str) -> String {
    let key= self::derive_key();
    let bytes = decode(s);
    let r: Vec<u8> = bytes.iter().enumerate().map(|(i, b)| b ^ key[i % key.len()]).collect();
    String::from_utf8(r).unwrap_or_default()
}

fn encode(data: &[u8]) -> String {
    const C: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut r = String::new();
    for ch in data.chunks(3) {
        let b0 = ch[0] as u32;
        let b1 = ch.get(1).copied().unwrap_or(0) as u32;
        let b2 = ch.get(2).copied().unwrap_or(0) as u32;
        let t = (b0 << 16) | (b1 << 8) | b2;
        r.push(C[((t >> 18) & 0x3F) as usize] as char);
        r.push(C[((t >> 12) & 0x3F) as usize] as char);
        r.push(if ch.len() > 1 { C[((t >> 6) & 0x3F) as usize] as char } else { '=' });
        r.push(if ch.len() > 2 { C[(t & 0x3F) as usize] as char } else { '=' });
    }
    r
}

fn decode(s: &str) -> Vec<u8> {
    fn val(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some((c - b'A') as u32),
            b'a'..=b'z' => Some((c - b'a' + 26) as u32),
            b'0'..=b'9' => Some((c - b'0' + 52) as u32),
            b'+' => Some(62),
            b'/' => Some(63),
            b'=' => None,
            _ => Some(0),
        }
    }
    let mut r = Vec::new();
    let bytes: Vec<u8> = s.bytes().collect();
    for ch in bytes.chunks(4) {
        if ch.len() < 4 { break; }
        let d0 = val(ch[0]).unwrap_or(0);
        let d1 = val(ch[1]).unwrap_or(0);
        let d2 = val(ch[2]);
        let d3 = val(ch[3]);
        let pad2 = d2.is_none();
        let pad3 = d3.is_none();
        let d2 = d2.unwrap_or(0);
        let d3 = d3.unwrap_or(0);
        let t = (d0 << 18) | (d1 << 12) | (d2 << 6) | d3;
        r.push((t >> 16) as u8);
        if !pad2 { r.push((t >> 8) as u8); }
        if !pad3 { r.push(t as u8); }
    }
    r
}