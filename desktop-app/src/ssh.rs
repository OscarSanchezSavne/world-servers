use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;

pub struct SshClient {
    session: Session,
}

impl SshClient {
    pub fn connect(ip: &str, user: &str, key_path: &str, passphrase: Option<&str>) -> Result<Self, String> {
        let addr = format!("{ip}:22");
        let tcp = TcpStream::connect(&addr)
            .map_err(|e| format!("Connection to {addr} failed: {e}"))?;

        let mut session = Session::new()
            .map_err(|e| format!("Failed to create SSH session: {e}"))?;

        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| format!("SSH handshake failed: {e}"))?;

        let key_expanded = if key_path.starts_with('~') {
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
        };

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
}
