use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::time::Duration;

pub struct SshClient {
    session: Session,
}

impl SshClient {
    pub fn connect(ip: &str, user: &str, key_path: &str, passphrase: Option<&str>) -> Result<Self, String> {
        let addr = format!("{ip}:22");
        let timeout = Duration::from_secs(15);
        let tcp = TcpStream::connect_timeout(
            &addr.parse().map_err(|e| format!("Invalid address {addr}: {e}"))?,
            timeout,
        )
        .map_err(|e| format!("Connection to {addr} failed: {e}"))?;

        tcp.set_read_timeout(Some(timeout))
            .map_err(|e| format!("Failed to set read timeout: {e}"))?;
        tcp.set_write_timeout(Some(timeout))
            .map_err(|e| format!("Failed to set write timeout: {e}"))?;

        let mut session = Session::new()
            .map_err(|e| format!("Failed to create SSH session: {e}"))?;

        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| format!("SSH handshake failed: {e}"))?;

        let key_expanded = expand_path(key_path);

        session.userauth_pubkey_file(user, None, Path::new(&key_expanded), passphrase)
            .map_err(|e| format!("SSH authentication failed: {e}"))?;

        if !session.authenticated() {
            return Err("SSH authentication failed: not authenticated".into());
        }

        Ok(SshClient { session })
    }

    pub fn exec(&self, command: &str) -> Result<String, String> {
        let mut channel = self.session.channel_session()
            .map_err(|e| format!("Failed to open channel: {e}"))?;

        channel.exec(command)
            .map_err(|e| format!("Failed to execute command '{command}': {e}"))?;

        let mut output = String::new();
        channel.read_to_string(&mut output)
            .map_err(|e| format!("Failed to read command output: {e}"))?;

        channel.wait_close()
            .map_err(|e| format!("Failed to close channel: {e}"))?;

        Ok(output.trim().to_string())
    }

    pub fn write_sudo_file(&self, remote_path: &str, data: &[u8]) -> Result<(), String> {
        let tmp = "/tmp/worldservers_tmp";
        self.scp_send(tmp, data, 0o644)?;
        self.exec(&format!("sudo mv {tmp} {remote_path}"))
            .map_err(|e| format!("mv failed: {e}"))?;
        Ok(())
    }

    pub fn scp_send(&self, remote_path: &str, data: &[u8], mode: i32) -> Result<(), String> {
        let mut channel = self.session.scp_send(
            Path::new(remote_path),
            mode,
            data.len() as u64,
            None,
        ).map_err(|e| format!("SCP send failed (open): {e}"))?;

        channel.write_all(data)
            .map_err(|e| format!("SCP send failed (write): {e}"))?;

        channel.send_eof()
            .map_err(|e| format!("SCP send failed (eof): {e}"))?;

        channel.wait_close()
            .map_err(|e| format!("SCP send failed (close): {e}"))?;

        Ok(())
    }
}

fn expand_path(key_path: &str) -> std::path::PathBuf {
    if key_path.starts_with('~') {
        if let Some(home) = std::env::var_os("HOME") {
            let mut path = std::path::PathBuf::from(home);
            let rest = &key_path[1..];
            if rest.starts_with('/') || rest.starts_with('\\') {
                path.push(&rest[1..]);
            } else {
                path.push(rest);
            }
            path
        } else {
            std::path::PathBuf::from(key_path)
        }
    } else {
        std::path::PathBuf::from(key_path)
    }
}
