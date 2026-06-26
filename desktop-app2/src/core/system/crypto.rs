const KEY: &[u8] = b"WorldServers2024!X";

pub fn obfuscate(s: &str) -> String {
    let bytes: Vec<u8> = s.bytes().enumerate().map(|(i, b)| b ^ KEY[i % KEY.len()]).collect();
    encode(&bytes)
}

pub fn deobfuscate(s: &str) -> String {
    let bytes = decode(s);
    let r: Vec<u8> = bytes.iter().enumerate().map(|(i, b)| b ^ KEY[i % KEY.len()]).collect();
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
    fn val(c: u8) -> u32 {
        match c {
            b'A'..=b'Z' => (c - b'A') as u32,
            b'a'..=b'z' => (c - b'a' + 26) as u32,
            b'0'..=b'9' => (c - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            _ => 0,
        }
    }
    let mut r = Vec::new();
    let b: Vec<u8> = s.bytes().filter(|&b| b != b'=').collect();
    for ch in b.chunks(4) {
        if ch.len() < 4 { break; }
        let t = (val(ch[0]) << 18) | (val(ch[1]) << 12) | (val(ch[2]) << 6) | val(ch[3]);
        r.push((t >> 16) as u8);
        r.push((t >> 8) as u8);
        r.push(t as u8);
    }
    let n = (s.len() / 4) * 3;
    while r.len() > n { r.pop(); }
    r
}