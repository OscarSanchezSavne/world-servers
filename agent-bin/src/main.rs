mod metrics;

use std::net::UdpSocket;
use std::time::Duration;

const VERSION: &str = "1.0.0";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "--version" {
        println!("{}", VERSION);
        return;
    }

    let dest_ip = parse_arg(&args, "--dest-ip").unwrap_or("127.0.0.1");
    let dest_port = parse_arg(&args, "--dest-port")
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(9876);
    let server_id = parse_arg(&args, "--id");
    let interval = parse_arg(&args, "--interval")
        .and_then(|i| i.parse::<u64>().ok())
        .unwrap_or(500);

    let dest = format!("{dest_ip}:{dest_port}");

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap_or_else(|e| {
        eprintln!("Failed to create UDP socket: {e}");
        std::process::exit(1);
    });

    let mut backoff_until: u64 = 0;
    let mut precheck_counter: u64 = 0;

    loop {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // ── Skip collection if in backoff ──
        if backoff_until > now {
            std::thread::sleep(Duration::from_millis(interval));
            continue;
        }

        // ── TCP pre-check every ~2 minutes (240 iterations at 500ms) ──
        precheck_counter += 1;
        if precheck_counter >= 240 {
            precheck_counter = 0;
            let addr: std::net::SocketAddr = match dest.parse() {
                Ok(a) => a,
                Err(_) => {
                    eprintln!("Invalid destination: {dest}");
                    std::thread::sleep(Duration::from_millis(interval));
                    continue;
                }
            };
            match std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
                Ok(stream) => drop(stream),
                Err(_) => {
                    eprintln!("Destination {dest} not reachable. Backing off 2 minutes...");
                    backoff_until = now + 120;
                    std::thread::sleep(Duration::from_millis(interval));
                    continue;
                }
            }
        }

        // ── Collect metrics ──
        let mut snapshot = match metrics::collect() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error collecting metrics: {e}");
                std::thread::sleep(Duration::from_secs(1));
                continue;
            }
        };

        if let Some(ref id) = server_id {
            snapshot.id = id.to_string();
        }

        // ── Serialize ──
        let json = match serde_json::to_string(&snapshot) {
            Ok(j) => j,
            Err(e) => {
                eprintln!("JSON serialization error: {e}");
                std::thread::sleep(Duration::from_millis(interval));
                continue;
            }
        };

        // ── Send ──
        if let Err(e) = socket.send_to(json.as_bytes(), &dest) {
            eprintln!("UDP send failed ({e}). Backing off for 2 minutes...");
            backoff_until = now + 120;
        }

        std::thread::sleep(Duration::from_millis(interval));
    }
}

fn parse_arg<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find_map(|w| {
        if w[0] == name { Some(w[1].as_str()) } else { None }
    })
}
