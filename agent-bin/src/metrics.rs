use serde::Serialize;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Debug)]
pub struct Snapshot {
    pub id: String,
    pub ts: u64,
    pub cpu: f64,
    pub load: [f64; 3],
    pub ram_used_mb: u64,
    pub rx: u64,
    pub tx: u64,
    pub targets: Vec<String>,
}

pub fn collect() -> Result<Snapshot, String> {
    let hostname = fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "unknown".into())
        .trim()
        .to_string();

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let cpu = read_cpu_percent()?;
    let load = read_loadavg();
    let ram_used_mb = read_ram_mb();
    let (rx, tx) = read_net_traffic();
    let targets = read_targets();

    Ok(Snapshot {
        id: hostname,
        ts,
        cpu,
        load,
        ram_used_mb,
        rx,
        tx,
        targets,
    })
}

/// CPU usage percentage from /proc/stat (delta between two reads)
fn read_cpu_percent() -> Result<f64, String> {
    let idle1 = cpu_idle()?;
    let total1 = cpu_total()?;
    std::thread::sleep(std::time::Duration::from_millis(100));
    let idle2 = cpu_idle()?;
    let total2 = cpu_total()?;

    let idle_delta = idle2.saturating_sub(idle1);
    let total_delta = total2.saturating_sub(total1);

    if total_delta == 0 {
        return Ok(0.0);
    }

    let busy = total_delta.saturating_sub(idle_delta);
    Ok((busy as f64 / total_delta as f64) * 100.0)
}

fn cpu_idle() -> Result<u64, String> {
    let content = fs::read_to_string("/proc/stat")
        .map_err(|e| format!("Failed to read /proc/stat: {e}"))?;
    for line in content.lines() {
        if line.starts_with("cpu ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 4 {
                return parts[4].parse::<u64>()
                    .map_err(|_| "Invalid idle value".into());
            }
        }
    }
    Err("No cpu line found in /proc/stat".into())
}

fn cpu_total() -> Result<u64, String> {
    let content = fs::read_to_string("/proc/stat")
        .map_err(|e| format!("Failed to read /proc/stat: {e}"))?;
    for line in content.lines() {
        if line.starts_with("cpu ") {
            let sum: u64 = line
                .split_whitespace()
                .skip(1)
                .filter_map(|v| v.parse::<u64>().ok())
                .sum();
            return Ok(sum);
        }
    }
    Err("No cpu line found in /proc/stat".into())
}

/// Load average from /proc/loadavg — 1, 5, 15 min
fn read_loadavg() -> [f64; 3] {
    let content = fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() >= 3 {
        let l1 = parts[0].parse::<f64>().unwrap_or(0.0);
        let l5 = parts[1].parse::<f64>().unwrap_or(0.0);
        let l15 = parts[2].parse::<f64>().unwrap_or(0.0);
        [l1, l5, l15]
    } else {
        [0.0, 0.0, 0.0]
    }
}

/// RAM usage in MB from /proc/meminfo
fn read_ram_mb() -> u64 {
    let content = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mut total_kb = 0u64;
    let mut available_kb = 0u64;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            if let Some(val) = line.split_whitespace().nth(1) {
                total_kb = val.parse().unwrap_or(0);
            }
        } else if line.starts_with("MemAvailable:") {
            if let Some(val) = line.split_whitespace().nth(1) {
                available_kb = val.parse().unwrap_or(0);
            }
        }
    }

    let used_kb = total_kb.saturating_sub(available_kb);
    used_kb / 1024
}

/// Network traffic (RX/TX bytes) from /proc/net/dev, summed across all interfaces
fn read_net_traffic() -> (u64, u64) {
    let content = fs::read_to_string("/proc/net/dev").unwrap_or_default();
    let mut rx_total = 0u64;
    let mut tx_total = 0u64;

    for line in content.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 10 {
            if let Ok(rx) = parts[1].parse::<u64>() {
                rx_total += rx;
            }
            if let Ok(tx) = parts[9].parse::<u64>() {
                tx_total += tx;
            }
        }
    }

    (rx_total, tx_total)
}

/// Outbound connection targets (unique remote IPs) from /proc/net/tcp
fn read_targets() -> Vec<String> {
    let content = fs::read_to_string("/proc/net/tcp").unwrap_or_default();
    let mut targets: Vec<String> = Vec::new();

    for line in content.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        // State 01 = ESTABLISHED
        if parts[3] != "01" {
            continue;
        }
        // Remote address is in parts[2], format: hex_ip:hex_port
        let remote = parts[2];
        if let Some(hex_ip) = remote.split(':').next() {
            if let Some(ip) = hex_to_ipv4(hex_ip) {
                if !ip.starts_with("127.") && !ip.starts_with("0.") && !targets.contains(&ip) {
                    targets.push(ip);
                }
            }
        }
    }

    targets
}

/// Convert hex-encoded IPv4 (little-endian) to dotted decimal string
fn hex_to_ipv4(hex: &str) -> Option<String> {
    if hex.len() < 8 {
        return None;
    }
    // /proc/net/tcp stores IP in hex, little-endian: "0100007F" -> 127.0.0.1
    let bytes = u32::from_str_radix(&hex[..8], 16).ok()?;
    let octets = bytes.to_le_bytes();
    Some(format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3]))
}
