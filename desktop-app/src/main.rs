use eframe::egui;
use lang::{Lang, Strings};
use serde::{Deserialize, Serialize};
use ssh::SshClient;
use std::net::IpAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};

mod lang;
mod ssh;

enum DeployEvent {
    Log(String),
    Done(Server),
    Error(String),
}

const SERVERS_PATH: &str = "servers.toml";
const CONFIG_PATH: &str = "config.toml";

#[derive(Serialize, Deserialize, Clone)]
struct AppConfig {
    central_host: String,
    central_port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            central_host: String::new(),
            central_port: 9876,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Server {
    id: String,
    name: String,
    ip: String,
    ssh_user: String,
    ssh_key_path: String,
    use_passphrase: bool,
    hostname: String,
    cpu_model: String,
    cpu_count: u32,
    ram_total_mb: u64,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
}

#[derive(Serialize, Deserialize, Default)]
struct ServersConfig {
    servers: Vec<Server>,
}

fn load_config() -> AppConfig {
    let path = Path::new(CONFIG_PATH);
    if path.exists() {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

fn save_config(config: &AppConfig) {
    let content = toml::to_string_pretty(config).expect("Failed to serialize config");
    std::fs::write(CONFIG_PATH, content).expect("Failed to write config.toml");
}

fn get_local_ips() -> Vec<String> {
    let mut ips: Vec<String> = Vec::new();
    if let Ok(ifaces) = local_ip_address::list_afinet_netifas() {
        for (_, ip) in ifaces {
            let s = ip.to_string();
            if !ips.contains(&s) {
                ips.push(s);
            }
        }
    }
    ips.sort();
    ips
}

fn load_servers() -> ServersConfig {
    let path = Path::new(SERVERS_PATH);
    if path.exists() {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    } else {
        ServersConfig::default()
    }
}

fn save_servers(config: &ServersConfig) {
    let content = toml::to_string_pretty(config).expect("Failed to serialize servers");
    std::fs::write(SERVERS_PATH, content).expect("Failed to write servers.toml");
}

fn load_favicon() -> egui::IconData {
    let bytes = include_bytes!("../assets/images/isotipo.png");
    let color_image = egui_extras::image::load_image_bytes(bytes)
        .expect("Failed to decode favicon");
    egui::IconData {
        rgba: color_image.pixels.iter().flat_map(|c| c.to_array()).collect(),
        width: color_image.width() as u32,
        height: color_image.height() as u32,
    }
}

fn main() -> eframe::Result {
    let icon = load_favicon();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 650.0])
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "WorldServers",
        options,
        Box::new(|cc| {
            configure_style(&cc.egui_ctx);
            Ok(Box::new(DesktopApp::new(&cc.egui_ctx)))
        }),
    )
}

fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.spacing.item_spacing = egui::vec2(12.0, 8.0);
    style.spacing.button_padding = egui::vec2(14.0, 6.0);

    let corner = egui::CornerRadius::same(6);
    style.visuals.widgets.noninteractive.corner_radius = corner;
    style.visuals.widgets.inactive.corner_radius = corner;
    style.visuals.widgets.active.corner_radius = corner;
    style.visuals.widgets.hovered.corner_radius = corner;

    let bg_deep     = egui::Color32::from_rgb(5,   8,  14);
    let bg_panel    = egui::Color32::from_rgb(11,  17, 27);
    let accent      = egui::Color32::from_rgb(22,  132, 255);
    let white_cold  = egui::Color32::from_rgb(215, 220, 232);

    style.visuals.dark_mode = true;
    style.visuals.panel_fill = bg_deep;
    style.visuals.window_fill = bg_panel;
    style.visuals.faint_bg_color = bg_deep;
    style.visuals.extreme_bg_color = bg_panel;

    style.visuals.override_text_color = Some(white_cold);

    let input_border = egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 65, 85));
    let input_border_focus = egui::Stroke::new(1.0, accent);

    style.visuals.widgets.noninteractive.bg_fill = bg_panel;
    style.visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgb(191, 199, 212);
    style.visuals.widgets.noninteractive.bg_stroke = input_border;

    style.visuals.widgets.inactive.bg_fill = bg_panel;
    style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgb(191, 199, 212);
    style.visuals.widgets.inactive.bg_stroke = input_border;

    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(14, 22, 35);
    style.visuals.widgets.active.fg_stroke.color = white_cold;
    style.visuals.widgets.active.bg_stroke = input_border_focus;

    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(18, 28, 42);
    style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgb(191, 199, 212);
    style.visuals.widgets.hovered.bg_stroke = input_border;

    style.visuals.selection.bg_fill = egui::Color32::from_rgba_premultiplied(22, 132, 255, 60);
    style.visuals.selection.stroke.color = accent;

    style.visuals.hyperlink_color = egui::Color32::from_rgb(54, 200, 255);

    ctx.set_style(style);
}

struct DesktopApp {
    show_register_form: bool,
    show_setup: bool,
    show_passphrase_modal: bool,
    passphrase_checked: bool,
    passphrase_keys: Vec<(String, String, Vec<usize>)>,
    update_running: bool,
    update_delay: u8,
    update_key_idx: usize,
    update_server_idx: usize,
    update_done: bool,
    initial_refresh: bool,
    refresh_running: bool,
    refresh_idx: usize,
    install_popup_idx: Option<usize>,
    install_server_idx: usize,
    install_phase: u8,
    form_ip: String,
    form_ssh_user: String,
    form_ssh_key: String,
    form_server_id: String,
    form_use_passphrase: bool,
    form_passphrase: String,
    form_error: String,
    form_ip_warning: String,
    setup_central_host: String,
    setup_port: u16,
    setup_error: String,
    deploy_running: bool,
    deploy_success: bool,
    deploy_failed: bool,
    deploy_client: Option<ssh::SshClient>,
    progress_log: Vec<String>,
    progress_running: bool,
    progress_done: bool,
    progress_failed: bool,
    progress_title: String,
    deploy_queue: Option<Arc<Mutex<Vec<DeployEvent>>>>,
    servers: Vec<Server>,
    local_ips: Vec<String>,
    config: AppConfig,
    logotipo_tex: egui::TextureHandle,
    isotipo_tex: egui::TextureHandle,
    lang: Lang,
}

impl DesktopApp {
    fn t(&self) -> Strings {
        self.lang.strings()
    }

    fn new(ctx: &egui::Context) -> Self {
        fn load_texture(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
            let color_image = egui_extras::image::load_image_bytes(bytes)
                .expect("Failed to decode image");
            ctx.load_texture(name, color_image, egui::TextureOptions::LINEAR)
        }

        let logotipo_tex = load_texture(ctx, "logotipo", include_bytes!("../assets/images/logotipo.png"));
        let isotipo_tex = load_texture(ctx, "isotipo", include_bytes!("../assets/images/isotipo.png"));
        let servers = load_servers().servers;
        let config = load_config();
        let local_ips = get_local_ips();
        let needs_setup = config.central_host.is_empty();

        Self {
            show_register_form: false,
            show_setup: needs_setup,
            show_passphrase_modal: false,
            passphrase_checked: false,
            passphrase_keys: Vec::new(),
            update_running: false,
            update_delay: 0,
            update_key_idx: 0,
            update_server_idx: 0,
            update_done: false,
            initial_refresh: false,
            refresh_running: false,
            refresh_idx: 0,
            install_popup_idx: None,
            install_server_idx: 0,
            install_phase: 0,
            form_ip: String::new(),
            form_ssh_user: String::new(),
            form_ssh_key: String::new(),
            form_server_id: String::new(),
            form_use_passphrase: false,
            form_passphrase: String::new(),
            form_error: String::new(),
            form_ip_warning: String::new(),
            setup_central_host: local_ips.first().cloned().unwrap_or_default(),
            setup_port: 9876,
            setup_error: String::new(),
            deploy_running: false,
            deploy_success: false,
            deploy_failed: false,
            deploy_client: None,
            progress_log: Vec::new(),
            progress_running: false,
            progress_done: false,
            progress_failed: false,
            progress_title: String::new(),
            deploy_queue: None,
            servers,
            local_ips,
            config,
            logotipo_tex,
            isotipo_tex,
            lang: Lang::En,
        }
    }

    fn reset_form(&mut self) {
        self.form_ip.clear();
        self.form_ssh_user.clear();
        self.form_ssh_key.clear();
        self.form_server_id.clear();
        self.form_use_passphrase = false;
        self.form_passphrase.clear();
        self.form_error.clear();
        self.form_ip_warning.clear();
        self.deploy_running = false;
        self.deploy_client = None;
        self.deploy_success = false;
        self.deploy_failed = false;
        self.deploy_queue = None;
    }

    fn reset_progress(&mut self) {
        self.progress_running = false;
        self.progress_done = false;
        self.progress_failed = false;
        self.progress_log.clear();
        self.progress_title.clear();
    }

    fn start_deploy(&mut self) {
        let t = self.t();

        let id = self.form_server_id.trim().to_string();
        let ip = self.form_ip.trim().to_string();
        let user = self.form_ssh_user.trim().to_string();
        let key = self.form_ssh_key.trim().to_string();

        if id.is_empty() { self.form_error = t.err_id_required.into(); return; }
        if ip.is_empty() { self.form_error = t.err_ip_required.into(); return; }
        if user.is_empty() { self.form_error = t.err_user_required.into(); return; }
        if key.is_empty() { self.form_error = t.err_key_required.into(); return; }

        if !Path::new(&key).exists() {
            let expanded = if key.starts_with('~') {
                if let Some(home) = std::env::var_os("HOME") {
                    let mut path = std::path::PathBuf::from(home);
                    let rest = &key[1..];
                    if rest.starts_with('/') || rest.starts_with('\\') { path.push(&rest[1..]); }
                    else { path.push(rest); }
                    path
                } else { std::path::PathBuf::from(&key) }
            } else { std::path::PathBuf::from(&key) };
            if !expanded.exists() { self.form_error = t.err_key_not_found_msg(&key); return; }
        }

        if self.servers.iter().any(|s| s.id == id) { self.form_error = t.err_id_exists_msg(&id); return; }
        if self.servers.iter().any(|s| s.ip == ip) { self.form_error = t.err_ip_exists_msg(&ip); return; }

        let passphrase_opt = if self.form_use_passphrase && !self.form_passphrase.is_empty() {
            Some(self.form_passphrase.clone())
        } else {
            None
        };

        self.progress_log.clear();
        self.progress_title = t.register_server.to_string();
        self.progress_running = true;
        self.progress_done = false;
        self.progress_failed = false;
        self.deploy_success = false;
        self.deploy_failed = false;
        self.form_error.clear();
        self.deploy_running = true;

        // Close register form immediately — progress modal handles everything
        self.show_register_form = false;

        let queue: Arc<Mutex<Vec<DeployEvent>>> = Arc::new(Mutex::new(Vec::new()));
        self.deploy_queue = Some(queue.clone());

        let id_for_thread = id.clone();
        let ip_for_thread = ip.clone();
        std::thread::spawn(move || {
            queue.lock().unwrap().push(DeployEvent::Log(format!("Connecting to {}...", ip_for_thread)));
            match SshClient::connect(&ip_for_thread, &user, &key, passphrase_opt.as_deref()) {
                Ok(client) => {
                    queue.lock().unwrap().push(DeployEvent::Log("Connected".into()));

                    let mut hostname = String::new();
                    let mut cpu_model = String::new();
                    let mut cpu_count: u32 = 0;
                    let mut ram_mb: u64 = 0;

                    queue.lock().unwrap().push(DeployEvent::Log("Fetching hostname...".into()));
                    match client.exec("hostname") {
                        Ok(h) => {
                            hostname = h.clone();
                            queue.lock().unwrap().push(DeployEvent::Log(format!("  {h}")));
                        }
                        Err(e) => {
                            queue.lock().unwrap().push(DeployEvent::Log(format!("  warn: {e}")));
                        }
                    }

                    queue.lock().unwrap().push(DeployEvent::Log("Fetching CPU model...".into()));
                    match client.exec(r#"grep -m1 "model name" /proc/cpuinfo | cut -d: -f2 | sed 's/^ //'"#) {
                        Ok(c) => {
                            cpu_model = c.clone();
                            queue.lock().unwrap().push(DeployEvent::Log(format!("  {c}")));
                        }
                        Err(e) => {
                            queue.lock().unwrap().push(DeployEvent::Log(format!("  warn: {e}")));
                        }
                    }
                    match client.exec("nproc") {
                        Ok(n) => {
                            cpu_count = n.trim().parse().unwrap_or(0);
                            queue.lock().unwrap().push(DeployEvent::Log(format!("  {} CPUs", cpu_count)));
                        }
                        Err(e) => {
                            queue.lock().unwrap().push(DeployEvent::Log(format!("  warn: {e}")));
                        }
                    }

                    queue.lock().unwrap().push(DeployEvent::Log("Fetching RAM...".into()));
                    match client.exec(r#"grep MemTotal /proc/meminfo | awk '{print $2}'"#) {
                        Ok(r) => {
                            let kb: u64 = r.trim().parse().unwrap_or(0);
                            ram_mb = kb / 1024;
                            queue.lock().unwrap().push(DeployEvent::Log(format!("  {} MB", ram_mb)));
                        }
                        Err(e) => {
                            queue.lock().unwrap().push(DeployEvent::Log(format!("  warn: {e}")));
                        }
                    }

                    queue.lock().unwrap().push(DeployEvent::Log("Saving server data...".into()));
                    let name = id_for_thread.clone();
                    let server = Server {
                        id: id_for_thread,
                        name,
                        ip: ip_for_thread,
                        ssh_user: user,
                        ssh_key_path: key,
                        use_passphrase: passphrase_opt.is_some(),
                        hostname,
                        cpu_model,
                        cpu_count,
                        ram_total_mb: ram_mb,
                        pos_x: 0.0,
                        pos_y: 0.0,
                        pos_z: 0.0,
                    };
                    queue.lock().unwrap().push(DeployEvent::Log("Done".into()));
                    queue.lock().unwrap().push(DeployEvent::Done(server));
                }
                Err(e) => {
                    queue.lock().unwrap().push(DeployEvent::Log(format!("Connection failed: {e}")));
                    queue.lock().unwrap().push(DeployEvent::Error(format!("SSH connection failed: {e}")));
                }
            }
        });
    }

    fn step_update(&mut self) {
        if self.update_key_idx >= self.passphrase_keys.len() {
            self.update_running = false;
            self.update_done = true;
            self.progress_done = true;
            self.progress_running = false;
            return;
        }

        let (_key_path, ref passphrase, ref indices) = self.passphrase_keys[self.update_key_idx].clone();
        if passphrase.is_empty() {
            self.update_key_idx += 1;
            self.update_server_idx = 0;
            return;
        }

        if self.update_server_idx >= indices.len() {
            self.update_key_idx += 1;
            self.update_server_idx = 0;
            return;
        }

        let idx = indices[self.update_server_idx];
        if idx >= self.servers.len() {
            self.update_server_idx += 1;
            return;
        }

        let ip = self.servers[idx].ip.clone();
        let user = self.servers[idx].ssh_user.clone();
        let key = self.servers[idx].ssh_key_path.clone();
        let name = self.servers[idx].name.clone();

        self.progress_log.push(format!("Updating {} ({})...", name, ip));

        let hostname_was = self.servers[idx].hostname.clone();
        self.servers[idx].hostname.clear();

        if let Ok(client) = SshClient::connect(&ip, &user, &key, Some(&passphrase)) {
            self.progress_log.push("Connected".into());

            self.progress_log.push("Fetching hostname...".into());
            match client.exec("hostname") {
                Ok(h) => {
                    self.servers[idx].hostname = h.clone();
                    self.progress_log.push(format!("  {h}"));
                }
                Err(e) => {
                    self.progress_log.push(format!("  warn: {e}"));
                    self.servers[idx].hostname = hostname_was;
                }
            }

            self.progress_log.push("Fetching CPU model...".into());
            match client.exec(r#"grep -m1 "model name" /proc/cpuinfo | cut -d: -f2 | sed 's/^ //'"#) {
                Ok(c) => {
                    self.servers[idx].cpu_model = c.clone();
                    self.progress_log.push(format!("  {c}"));
                }
                Err(e) => self.progress_log.push(format!("  warn: {e}")),
            }
            match client.exec("nproc") {
                Ok(n) => {
                    self.servers[idx].cpu_count = n.trim().parse().unwrap_or(0);
                    self.progress_log.push(format!("  {} CPUs", self.servers[idx].cpu_count));
                }
                Err(e) => self.progress_log.push(format!("  warn: {e}")),
            }

            self.progress_log.push("Fetching RAM...".into());
            match client.exec(r#"grep MemTotal /proc/meminfo | awk '{print $2}'"#) {
                Ok(r) => {
                    let kb: u64 = r.trim().parse().unwrap_or(0);
                    self.servers[idx].ram_total_mb = kb / 1024;
                    self.progress_log.push(format!("  {} MB", kb / 1024));
                }
                Err(e) => self.progress_log.push(format!("  warn: {e}")),
            }
        } else {
            self.progress_log.push("Connection failed".into());
            if self.servers[idx].hostname.is_empty() {
                self.servers[idx].hostname = hostname_was;
            }
        }

        save_servers(&ServersConfig { servers: self.servers.clone() });
        self.update_server_idx += 1;
    }

    fn step_refresh(&mut self) {
        // Refresh ONE non-passphrase server per frame, then stop
        if self.refresh_idx >= self.servers.len() {
            self.refresh_running = false;
            self.progress_done = true;
            self.progress_running = false;
            return;
        }

        let idx = self.refresh_idx;
        if self.servers[idx].use_passphrase || !self.servers[idx].hostname.is_empty() {
            self.refresh_idx += 1;
            return;
        }

        let ip = self.servers[idx].ip.clone();
        let user = self.servers[idx].ssh_user.clone();
        let key = self.servers[idx].ssh_key_path.clone();
        let name = self.servers[idx].name.clone();

        self.progress_log.push(format!("Fetching {} ({})...", name, ip));

        if let Ok(client) = SshClient::connect(&ip, &user, &key, None) {
            self.progress_log.push("Connected".into());
            match client.exec("hostname") {
                Ok(h) => {
                    self.servers[idx].hostname = h.clone();
                    self.progress_log.push(format!("  {h}"));
                }
                Err(e) => self.progress_log.push(format!("  warn: {e}")),
            }
            match client.exec(r#"grep -m1 "model name" /proc/cpuinfo | cut -d: -f2 | sed 's/^ //'"#) {
                Ok(c) => {
                    self.servers[idx].cpu_model = c.clone();
                    self.progress_log.push(format!("  {c}"));
                }
                Err(e) => self.progress_log.push(format!("  warn: {e}")),
            }
            match client.exec("nproc") {
                Ok(n) => {
                    self.servers[idx].cpu_count = n.trim().parse().unwrap_or(0);
                    self.progress_log.push(format!("  {} CPUs", self.servers[idx].cpu_count));
                }
                Err(e) => self.progress_log.push(format!("  warn: {e}")),
            }
            match client.exec(r#"grep MemTotal /proc/meminfo | awk '{print $2}'"#) {
                Ok(r) => {
                    let kb: u64 = r.trim().parse().unwrap_or(0);
                    self.servers[idx].ram_total_mb = kb / 1024;
                    self.progress_log.push(format!("  {} MB", kb / 1024));
                }
                Err(e) => self.progress_log.push(format!("  warn: {e}")),
            }
        } else {
            self.progress_log.push("Connection failed".into());
        }

        save_servers(&ServersConfig { servers: self.servers.clone() });
        self.refresh_idx += 1;
    }

    fn start_install(&mut self, idx: usize) {
        let t = self.t();
        self.install_server_idx = idx;
        self.install_phase = 1;
        self.install_popup_idx = None;
        self.progress_log.clear();
        self.progress_title = t.install_title.to_string();
        self.progress_running = true;
        self.progress_done = false;
        self.progress_failed = false;
    }

    fn step_install(&mut self) {
        let idx = self.install_server_idx;
        if idx >= self.servers.len() {
            self.progress_running = false;
            return;
        }

        let t = self.t();
        let server = self.servers[idx].clone();
        let ip = server.ip.clone();
        let user = server.ssh_user.clone();
        let key = server.ssh_key_path.clone();

        match self.install_phase {
            1 => {
                self.progress_log.push(t.install_log_connecting(&ip));
                match SshClient::connect(&ip, &user, &key, None) {
                    Ok(client) => {
                        self.progress_log.push(t.install_log_connected.into());
                        self.deploy_client = Some(client);
                        self.install_phase = 2;
                    }
                    Err(e) => {
                        self.progress_log.push(t.install_log_failed(&e));
                        self.progress_running = false;
                        self.progress_failed = true;
                    }
                }
            }
            2 => {
                self.progress_log.push(t.install_log_copy.into());
                let agent_bytes = include_bytes!("../assets/agent-bin");
                if let Some(ref client) = self.deploy_client {
                    match client.scp_send(
                        "/usr/local/bin/worldservers-agent",
                        agent_bytes,
                        0o755,
                    ) {
                        Ok(_) => {
                            self.progress_log.push(t.install_log_scp_ok.into());
                            self.install_phase = 3;
                        }
                        Err(e) => {
                            self.progress_log.push(t.install_log_scp_failed(&e));
                            self.progress_running = false;
                            self.progress_failed = true;
                        }
                    }
                }
            }
            3 => {
                self.progress_log.push(t.install_log_mkdir.into());
                if let Some(ref client) = self.deploy_client {
                    match client.exec("sudo mkdir -p /etc/worldservers") {
                        Ok(_) => {
                            self.progress_log.push(t.install_log_mkdir_ok.into());
                            self.install_phase = 4;
                        }
                        Err(e) => {
                            self.progress_log.push(t.install_log_mkdir_failed(&e));
                            self.progress_running = false;
                            self.progress_failed = true;
                        }
                    }
                }
            }
            4 => {
                self.progress_log.push(t.install_log_config.into());
                let config_body = format!(
                    "central_host = \"{host}\"\ncentral_port = {port}\nserver_id   = \"{sid}\"\naccept_unsecure = false\n",
                    host = self.config.central_host,
                    port = self.config.central_port,
                    sid = server.id,
                );
                if let Some(ref client) = self.deploy_client {
                    match client.write_sudo_file("/etc/worldservers/agent.conf", config_body.as_bytes()) {
                        Ok(_) => {
                            self.progress_log.push(t.install_log_config_ok.into());
                            self.install_phase = 5;
                        }
                        Err(e) => {
                            self.progress_log.push(t.install_log_config_failed(&e));
                            self.progress_running = false;
                            self.progress_failed = true;
                        }
                    }
                }
            }
            5 => {
                self.progress_log.push(t.install_log_service.into());
                let svc = "[Unit]
Description=WorldServers Agent
After=network.target

[Service]
ExecStart=/usr/local/bin/worldservers-agent
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
";
                if let Some(ref client) = self.deploy_client {
                    match client.write_sudo_file("/etc/systemd/system/worldservers-agent.service", svc.as_bytes()) {
                        Ok(_) => {
                            self.progress_log.push(t.install_log_service_ok.into());
                            self.install_phase = 6;
                        }
                        Err(e) => {
                            self.progress_log.push(t.install_log_service_failed(&e));
                            self.progress_running = false;
                            self.progress_failed = true;
                        }
                    }
                }
            }
            6 => {
                self.progress_log.push(t.install_log_enable.into());
                if let Some(ref client) = self.deploy_client {
                    match client.exec("sudo systemctl enable worldservers-agent") {
                        Ok(out) => {
                            self.progress_log.push(out);
                            self.install_phase = 7;
                        }
                        Err(e) => {
                            self.progress_log.push(t.install_log_enable_failed(&e));
                            self.progress_running = false;
                            self.progress_failed = true;
                        }
                    }
                }
            }
            7 => {
                self.progress_log.push(t.install_log_done.into());
                self.deploy_client = None;
                self.progress_running = false;
                self.progress_done = true;
            }
            _ => {}
        }
    }
}

fn is_private_ip(ip: &str) -> bool {
    if let Ok(addr) = ip.trim().parse::<std::net::IpAddr>() {
        match addr {
            std::net::IpAddr::V4(v4) => {
                let octets = v4.octets();
                if octets[0] == 10 { return true; }
                if octets[0] == 172 && (octets[1] & 0xF0) == 16 { return true; }
                if octets[0] == 192 && octets[1] == 168 { return true; }
                if octets[0] == 127 { return true; }
                false
            }
            std::net::IpAddr::V6(v6) => {
                v6.octets()[0] & 0xFE == 0xFC
                    || (v6.octets()[0] == 0xFE && (v6.octets()[1] & 0xC0) == 0x80)
                    || v6.is_loopback()
            }
        }
    } else {
        true
    }
}

impl eframe::App for DesktopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ── Poll SSH deployment thread ──
        if let Some(ref queue) = self.deploy_queue {
            let mut events: Vec<DeployEvent> = Vec::new();
            if let Ok(mut guard) = queue.lock() {
                events.append(&mut *guard);
            }
            if !events.is_empty() {
                for event in events {
                    match event {
                        DeployEvent::Log(msg) => {
                            self.progress_log.push(msg);
                        }
                        DeployEvent::Done(server) => {
                            self.servers.push(server);
                            save_servers(&ServersConfig { servers: self.servers.clone() });
                            self.deploy_running = false;
                            self.deploy_success = true;
                            self.progress_done = true;
                            self.progress_running = false;
                            self.deploy_queue = None;
                            break;
                        }
                        DeployEvent::Error(msg) => {
                            self.form_error = msg;
                            self.deploy_running = false;
                            self.deploy_failed = true;
                            self.progress_failed = true;
                            self.progress_running = false;
                            self.deploy_queue = None;
                            break;
                        }
                    }
                }
                ctx.request_repaint();
            }
        }

        // ── Step install agent (one phase per frame) ──
        if self.progress_running && self.install_phase > 0 && !self.progress_done && !self.progress_failed {
            self.step_install();
            ctx.request_repaint();
        }

        // ── Step refresh (fetch data for servers without passphrase) ──
        if self.refresh_running && self.progress_running {
            self.step_refresh();
            ctx.request_repaint();
        }

        // ── Step server updates (delay frames for UI to paint, then one server per frame) ──
        if self.update_running {
            if self.update_delay > 0 {
                self.update_delay -= 1;
                ctx.request_repaint();
            } else {
                self.step_update();
                ctx.request_repaint();
            }
        }

        let t = self.t();

        let accent_mid  = egui::Color32::from_rgb(15,  95, 234);
        let white_cold  = egui::Color32::from_rgb(215, 220, 232);
        let gray_soft   = egui::Color32::from_rgb(191, 199, 212);
        let gray_muted  = egui::Color32::from_rgb(142, 149, 166);
        let hint_color  = egui::Color32::from_rgb(75, 85, 100);
        let error_color = egui::Color32::from_rgb(230, 80, 80);
        let bg_panel    = egui::Color32::from_rgb(11,  17, 27);

        // ── Unified progress modal ──
        if self.progress_running || self.progress_done || self.progress_failed {
            let mut open = true;
            let mut close_clicked = false;
            let center = ctx.content_rect().center();
            egui::Window::new(&self.progress_title)
                .id("progress_modal".into())
                .open(&mut open)
                .default_pos(center - egui::vec2(240.0, 175.0))
                .fixed_size([480.0, 350.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    if !self.progress_log.is_empty() {
                        ui.add_space(8.0);
                        egui::Frame::NONE
                            .fill(egui::Color32::from_rgb(5, 8, 14))
                            .corner_radius(4)
                            .inner_margin(egui::Margin::symmetric(4, 6))
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                egui::ScrollArea::vertical()
                                    .max_height(260.0)
                                    .show(ui, |ui| {
                                        for line in &self.progress_log {
                                            if line.is_empty() {
                                                ui.add_space(2.0);
                                                continue;
                                            }
                                            if line.starts_with("warn:") || line.starts_with("  warn:") {
                                                ui.label(
                                                    egui::RichText::new(line)
                                                        .size(11.0)
                                                        .color(egui::Color32::from_rgb(230, 190, 50)),
                                                );
                                            } else {
                                                ui.label(
                                                    egui::RichText::new(line)
                                                        .size(11.0)
                                                        .color(gray_soft),
                                                );
                                            }
                                        }
                                    });
                            });
                    }

                    ui.add_space(8.0);
                    if self.progress_done || self.progress_failed {
                        ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let close_btn = egui::Button::new(
                                    egui::RichText::new(t.close).color(white_cold),
                                )
                                .fill(accent_mid)
                                .corner_radius(4.0);
                                if ui.add(close_btn).clicked() {
                                    close_clicked = true;
                                }
                            });
                        });
                    } else if self.deploy_running {
                        ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let cancel_btn = egui::Button::new(
                                    egui::RichText::new(t.cancel).color(white_cold),
                                )
                                .fill(error_color)
                                .corner_radius(4.0);
                                if ui.add(cancel_btn).clicked() {
                                    close_clicked = true;
                                }
                            });
                        });
                    }
                    ui.add_space(6.0);
                });

            if !open || close_clicked {
                if self.deploy_running {
                    self.deploy_running = false;
                    self.deploy_failed = true;
                    self.deploy_queue = None;
                }
                self.reset_progress();
                self.deploy_failed = false;
                self.deploy_success = false;
                self.form_error.clear();
            }
        }

        // ── Setup modal (first launch) ──
        if self.show_setup {
            let mut open = true;
            let setup_center = ctx.content_rect().center();
            egui::Window::new(t.setup_title)
                .open(&mut open)
                .default_pos(setup_center - egui::vec2(230.0, 170.0))
                .fixed_size([460.0, 340.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(t.setup_desc)
                            .size(13.0)
                            .color(gray_muted),
                    );
                    ui.add_space(12.0);

                    ui.label(
                        egui::RichText::new(t.setup_select_ip)
                            .size(12.0)
                            .color(gray_soft),
                    );
                    ui.add_space(4.0);
                    egui::ScrollArea::vertical()
                        .max_height(80.0)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            for ip in &self.local_ips {
                                let selected = self.setup_central_host == *ip;
                                let btn = egui::Button::new(ip)
                                    .fill(if selected { egui::Color32::from_rgb(22, 132, 255) } else { egui::Color32::TRANSPARENT });
                                if ui.add_sized([ui.available_width(), 22.0], btn).clicked() {
                                    self.setup_central_host = ip.clone();
                                }
                            }
                        });
                    ui.add_space(8.0);

                    ui.label(
                        egui::RichText::new(t.setup_manual)
                            .size(12.0)
                            .color(gray_soft),
                    );
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.setup_central_host)
                                .desired_width(200.0),
                        );
                        ui.label(
                            egui::RichText::new("Port:").size(12.0).color(gray_soft),
                        );
                        let mut port_str = self.setup_port.to_string();
                        if ui.add(
                            egui::TextEdit::singleline(&mut port_str).desired_width(60.0),
                        ).changed() {
                            if let Ok(p) = port_str.parse::<u16>() {
                                self.setup_port = p;
                            }
                        }
                    });

                    if !self.setup_central_host.is_empty() {
                        if let Ok(addr) = self.setup_central_host.trim().parse::<IpAddr>() {
                            let is_private = match addr {
                                IpAddr::V4(v4) => {
                                    let o = v4.octets();
                                    o[0] == 10 || (o[0] == 172 && (o[1] & 0xF0) == 16)
                                        || (o[0] == 192 && o[1] == 168) || o[0] == 127
                                }
                                IpAddr::V6(v6) => {
                                    v6.octets()[0] & 0xFE == 0xFC
                                        || (v6.octets()[0] == 0xFE && (v6.octets()[1] & 0xC0) == 0x80)
                                        || v6.is_loopback()
                                }
                            };
                            if !is_private {
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new(t.setup_warning)
                                        .size(11.0)
                                        .color(egui::Color32::from_rgb(230, 190, 50)),
                                );
                            }
                        }
                    }

                    if !self.setup_error.is_empty() {
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(&self.setup_error).size(12.0).color(error_color),
                        );
                    }

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let save_btn = egui::Button::new(
                                egui::RichText::new(t.setup_save).color(white_cold),
                            )
                            .fill(accent_mid)
                            .corner_radius(4.0);
                            if ui.add(save_btn).clicked() {
                                let host = self.setup_central_host.trim().to_string();
                                if host.is_empty() {
                                    self.setup_error = "IP address is required.".into();
                                } else if host.parse::<IpAddr>().is_ok() {
                                    self.config.central_host = host;
                                    self.config.central_port = self.setup_port;
                                    save_config(&self.config);
                                    self.show_setup = false;
                                } else {
                                    self.setup_error = "Invalid IP address.".into();
                                }
                            }
                        });
                    });
                    ui.add_space(8.0);
                });

            if !open {
                self.show_setup = false;
            }
        }

        // ── Passphrase collection modal ──
        if !self.show_setup && !self.passphrase_checked && !self.servers.is_empty() {
            self.passphrase_checked = true;
            let mut key_map: std::collections::BTreeMap<String, Vec<usize>> = std::collections::BTreeMap::new();
            for (i, srv) in self.servers.iter().enumerate() {
                if srv.use_passphrase {
                    key_map.entry(srv.ssh_key_path.clone()).or_default().push(i);
                }
            }
            let needs_update: std::collections::BTreeMap<String, Vec<usize>> = key_map.into_iter()
                .filter(|(_, indices)| {
                    indices.iter().any(|&i| i < self.servers.len() && self.servers[i].hostname.is_empty())
                })
                .collect();
            if !needs_update.is_empty() {
                self.passphrase_keys = needs_update.into_iter()
                    .map(|(k, indices)| (k, String::new(), indices))
                    .collect();
                self.show_passphrase_modal = true;
            }
        }

        // ── Initial refresh: load data for servers without passphrase ──
        if !self.show_setup && self.passphrase_checked && !self.initial_refresh
            && !self.show_passphrase_modal && !self.update_running
        {
            self.initial_refresh = true;
            let has_pending = self.servers.iter().any(|s| {
                !s.use_passphrase && s.hostname.is_empty()
            });
            if has_pending {
                self.progress_log.clear();
                self.progress_title = "Refreshing servers".into();
                self.progress_log.push("Fetching server data...".into());
                self.progress_running = true;
                self.progress_done = false;
                self.progress_failed = false;
                self.refresh_running = true;
                self.refresh_idx = 0;
            }
        }

        if self.show_passphrase_modal {
            let mut open = true;
            let pp_center = ctx.content_rect().center();
            egui::Window::new("SSH Passphrases")
                .id("passphrase_modal".into())
                .open(&mut open)
                .default_pos(pp_center - egui::vec2(200.0, 100.0))
                .fixed_size([400.0, 200.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new("Enter passphrases for the following SSH keys:")
                            .size(12.0).color(gray_muted),
                    );
                    ui.add_space(8.0);

                    let mut ready = true;
                    for idx in 0..self.passphrase_keys.len() {
                        let entry = &mut self.passphrase_keys[idx];
                        let key_path = &entry.0;
                        let pass = &mut entry.1;
                        let indices = &entry.2;
                        ui.horizontal(|ui| {
                            ui.set_width(ui.available_width());
                            ui.label(egui::RichText::new(key_path).size(11.0).color(gray_soft));
                            ui.add(
                                egui::TextEdit::singleline(pass)
                                    .password(true)
                                    .hint_text(egui::RichText::new("Passphrase").color(hint_color))
                                    .desired_width(160.0),
                            );
                            if pass.is_empty() {
                                ready = false;
                            }
                            ui.label(
                                egui::RichText::new(format!("({} server{})", indices.len(),
                                    if indices.len() > 1 { "s" } else { "" }))
                                    .size(10.0).color(gray_muted),
                            );
                        });
                        ui.add_space(4.0);
                    }

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let accept_btn = egui::Button::new(
                                egui::RichText::new("Accept").color(white_cold),
                            )
                            .fill(accent_mid)
                            .corner_radius(4.0);
                            if ui.add(accept_btn).clicked() && ready {
                                self.progress_log.clear();
                                self.progress_title = "Updating servers".into();
                                self.progress_log.push("Starting server updates...".into());
                                self.progress_running = true;
                                self.progress_done = false;
                                self.progress_failed = false;
                                self.update_running = true;
                                self.update_delay = 3;
                                self.update_key_idx = 0;
                                self.update_server_idx = 0;
                                self.update_done = false;
                                self.show_passphrase_modal = false;
                            }
                        });
                    });
                    ui.add_space(6.0);
                });

            if !open {
                self.show_passphrase_modal = false;
            }
        }

        // ── Register server modal ──
        if self.show_register_form {
            let mut open = true;
            let center = ctx.content_rect().center();
            egui::Window::new(t.register_server)
                .id("register_modal".into())
                .open(&mut open)
                .default_pos(center - egui::vec2(230.0, 175.0))
                .fixed_size([460.0, 350.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.set_min_size([460.0, 350.0].into());
                    let deploy_active = self.deploy_running || self.deploy_success;

                    if !deploy_active {
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(t.enter_details).size(13.0).color(gray_muted),
                        );
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(t.secure_network)
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(230, 190, 50)),
                            );
                            ui.label(
                                egui::RichText::new(t.secure_network_msg)
                                    .size(11.0).color(gray_muted),
                            );
                        });
                        ui.add_space(10.0);
                    }

                    if !deploy_active {
                        egui::Grid::new("register_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new(t.server_id).color(gray_soft));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.form_server_id)
                                        .hint_text(egui::RichText::new(t.hint_server_id).color(hint_color))
                                        .desired_width(200.0),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new(t.ip_address).color(gray_soft));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.form_ip).desired_width(200.0),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new(t.ssh_user).color(gray_soft));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.form_ssh_user).desired_width(200.0),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new(t.ssh_private_key).color(gray_soft));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.form_ssh_key)
                                        .hint_text(egui::RichText::new(t.hint_ssh_key).color(hint_color))
                                        .desired_width(200.0),
                                );
                                ui.end_row();

                                ui.label(egui::RichText::new(t.passphrase).color(gray_soft));
                                ui.vertical(|ui| {
                                    ui.add(
                                        egui::Checkbox::new(
                                            &mut self.form_use_passphrase,
                                            egui::RichText::new(t.use_passphrase_label).color(gray_soft),
                                        ),
                                    );
                                    if self.form_use_passphrase {
                                        ui.add(
                                            egui::TextEdit::singleline(&mut self.form_passphrase)
                                                .password(true).desired_width(200.0),
                                        );
                                    }
                                });
                                ui.end_row();
                            });

                        if !self.form_ip.is_empty() {
                            let ip_trimmed = self.form_ip.trim();
                            if !is_private_ip(ip_trimmed) {
                                self.form_ip_warning = t.ip_warning(ip_trimmed);
                            } else {
                                self.form_ip_warning.clear();
                            }
                        } else {
                            self.form_ip_warning.clear();
                        }

                        if !self.form_ip_warning.is_empty() {
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(&self.form_ip_warning)
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(230, 190, 50)),
                            );
                        }
                    }

                    if !self.form_error.is_empty() && !deploy_active {
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(&self.form_error).size(12.0).color(error_color),
                        );
                    }

                    ui.add_space(8.0);
                    if self.deploy_running {
                        ui.label(
                            egui::RichText::new("Processing...")
                                .size(12.0).color(gray_muted),
                        );
                    } else {
                        ui.horizontal(|ui| {
                            if ui.button(
                                egui::RichText::new(t.cancel).color(gray_soft),
                            ).clicked() {
                                self.show_register_form = false;
                                self.reset_form();
                            }
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let deploy_btn = egui::Button::new(
                                    egui::RichText::new(t.deploy_agent).color(white_cold),
                                )
                                .fill(accent_mid)
                                .corner_radius(4.0);
                                if ui.add(deploy_btn).clicked() {
                                    self.start_deploy();
                                }
                            });
                        });
                    }

                    if !deploy_active {
                        ui.add_space(12.0);
                        ui.separator();
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("🔒").size(11.0));
                            ui.label(
                                egui::RichText::new(t.local_storage_note)
                                    .size(11.0).color(gray_muted),
                            );
                        });
                        ui.add_space(8.0);
                    }
                });

            if !open {
                self.show_register_form = false;
                self.reset_form();
                self.reset_progress();
            }
        }

        // ── Top bar ──
        egui::TopBottomPanel::top("top_bar")
            .frame(egui::Frame {
                fill: bg_panel,
                inner_margin: egui::Margin::symmetric(12, 8),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::new(&self.logotipo_tex)
                            .max_height(28.0)
                            .max_width(220.0),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let btn = egui::Button::new(
                            egui::RichText::new(t.register_btn).color(white_cold),
                        )
                        .fill(accent_mid)
                        .stroke(egui::Stroke::NONE)
                        .corner_radius(4.0);
                        if ui.add(btn).clicked() {
                            self.show_register_form = true;
                        }
                    });
                });
            });

        // ── Central panel ──
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_rgb(5, 8, 14),
                inner_margin: egui::Margin::symmetric(12, 0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let card_frame = egui::Frame {
                    fill: bg_panel,
                    corner_radius: egui::CornerRadius::same(12),
                    stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(25, 35, 50)),
                    shadow: egui::epaint::Shadow {
                        offset: [0, 2].into(),
                        blur: 12,
                        spread: 0,
                        color: egui::Color32::from_black_alpha(60),
                    },
                    ..Default::default()
                };
                card_frame.show(ui, |ui| {
                    ui.add_space(16.0);
                    ui.vertical_centered(|ui| {
                        ui.add(
                            egui::Image::new(&self.isotipo_tex)
                                .max_width(64.0).max_height(64.0),
                        );
                        ui.add_space(12.0);
                        ui.add(egui::Label::new(
                            egui::RichText::new(t.welcome_title)
                                .size(28.0).color(white_cold).strong(),
                        ));
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(t.welcome_subtitle)
                                .size(14.0).color(gray_muted),
                        );
                    });
                    ui.add_space(16.0);
                });

                ui.add_space(24.0);

                let table_frame = egui::Frame {
                    fill: bg_panel,
                    corner_radius: egui::CornerRadius::same(12),
                    stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(25, 35, 50)),
                    shadow: egui::epaint::Shadow {
                        offset: [0, 2].into(),
                        blur: 12,
                        spread: 0,
                        color: egui::Color32::from_black_alpha(60),
                    },
                    inner_margin: egui::Margin::symmetric(4, 0),
                    ..Default::default()
                };
                table_frame.show(ui, |ui| {
                    ui.add_space(8.0);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let avail = ui.available_width();
                        let gap = 12.0;
                        let col_count = 7;
                        let spacing_total = gap * (col_count as f32 - 1.0);
                        let usable = avail - spacing_total;
                        let cw = [
                            usable * 0.16, usable * 0.13, usable * 0.17,
                            usable * 0.12, usable * 0.12, usable * 0.15,
                            usable * 0.15,
                        ];

                        if self.servers.is_empty() {
                            ui.add_space(48.0);
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new(t.no_servers).color(gray_soft));
                                ui.add_space(4.0);
                                ui.label(
                                    egui::RichText::new(t.no_servers_hint)
                                        .size(12.0).color(gray_muted),
                                );
                            });
                        } else {
                            egui::Grid::new("servers_grid")
                                .striped(true)
                                .min_col_width(0.0)
                                .show(ui, |ui| {
                                    let hdr = gray_muted;
                                    ui.add_sized([cw[0], 20.0], egui::Label::new(
                                        egui::RichText::new(t.col_name).color(hdr).strong()));
                                    ui.add_sized([cw[1], 20.0], egui::Label::new(
                                        egui::RichText::new(t.col_ip).color(hdr).strong()));
                                    ui.add_sized([cw[2], 20.0], egui::Label::new(
                                        egui::RichText::new(t.col_agent_version).color(hdr).strong()));
                                    ui.add_sized([cw[3], 20.0], egui::Label::new(
                                        egui::RichText::new(t.col_ram).color(hdr).strong()));
                                    ui.add_sized([cw[4], 20.0], egui::Label::new(
                                        egui::RichText::new(t.col_cpu).color(hdr).strong()));
                                    ui.add_sized([cw[5], 20.0], egui::Label::new(
                                        egui::RichText::new(t.col_status).color(hdr).strong()));
                                    ui.add_sized([cw[6], 20.0], egui::Label::new(
                                        egui::RichText::new(t.col_actions).color(hdr).strong()));
                                    ui.end_row();

                                    for (i, server) in self.servers.iter().enumerate() {
                                        ui.add_sized([cw[0], 20.0], egui::Label::new(&server.name));
                                        ui.add_sized([cw[1], 20.0], egui::Label::new(&server.ip));
                                        ui.add_sized([cw[2], 20.0], egui::Label::new(
                                            if server.hostname.is_empty() { t.unknown } else { &server.hostname }));
                                        let ram_str = if server.ram_total_mb > 0 {
                                            format!("{} MB", server.ram_total_mb)
                                        } else { t.unknown.into() };
                                        ui.add_sized([cw[3], 20.0], egui::Label::new(ram_str));
                                        let cpu_str = if server.cpu_model.is_empty() {
                                            t.unknown.into()
                                        } else if server.cpu_count > 0 {
                                            format!("{} ({}cpu)", server.cpu_model, server.cpu_count)
                                        } else { server.cpu_model.clone() };
                                        ui.add_sized([cw[4], 20.0], egui::Label::new(cpu_str));
                                        ui.add_sized([cw[5], 20.0], egui::Label::new(
                                            egui::RichText::new(t.online)
                                                .color(egui::Color32::from_rgb(70, 200, 120))));
                                        let resp = ui.add_sized([cw[6], 20.0],
                                            egui::Button::new(
                                                egui::RichText::new("\u{2699}").size(14.0).color(gray_soft),
                                            )
                                            .fill(egui::Color32::TRANSPARENT)
                                            .stroke(egui::Stroke::NONE),
                                        );
                                        if resp.clicked() {
                                            self.install_popup_idx = Some(i);
                                        }
                                        ui.end_row();
                                    }
                                });
                        }
                        ui.add_space(16.0);
                    });
                    ui.add_space(8.0);
                });
            });

        // ── Install agent popup ──
        if let Some(idx) = self.install_popup_idx {
            if idx < self.servers.len() {
                let server = self.servers[idx].clone();
                let mut open = true;
                egui::Window::new(t.actions_title)
                    .id(format!("install_popup_{idx}").into())
                    .open(&mut open)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .fixed_size([400.0, 220.0])
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.set_width(ui.available_width());
                        ui.add_space(12.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(t.actions_server_prefix)
                                    .size(14.0).color(gray_soft),
                            );
                            ui.label(
                                egui::RichText::new(&server.name)
                                    .size(14.0).color(white_cold).strong(),
                            );
                            ui.label(
                                egui::RichText::new(format!("<ip:{}>", server.ip))
                                    .size(12.0).color(gray_muted),
                            );
                        });
                        ui.add_space(8.0);
                        if self.config.central_host.is_empty() {
                            ui.label(
                                egui::RichText::new(t.install_popup_warning)
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(230, 190, 50)),
                            );
                        }
                        ui.add_space(12.0);
                        let install_btn = egui::Button::new(
                            egui::RichText::new(t.install_agent_btn).color(white_cold),
                        )
                        .fill(accent_mid)
                        .corner_radius(4.0);
                        if ui.add_sized([ui.available_width(), 0.0], install_btn).clicked()
                            && !self.config.central_host.is_empty()
                        {
                            self.start_install(idx);
                        }
                        ui.add_space(8.0);
                        let delete_btn = egui::Button::new(
                            egui::RichText::new(t.delete_server_btn).color(white_cold),
                        )
                        .fill(error_color)
                        .corner_radius(4.0);
                        if ui.add_sized([ui.available_width(), 0.0], delete_btn).clicked() {
                            self.servers.remove(idx);
                            save_servers(&ServersConfig { servers: self.servers.clone() });
                            self.install_popup_idx = None;
                        }
                        ui.add_space(12.0);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button(
                                egui::RichText::new(t.install_cancel).color(gray_soft),
                            ).clicked() {
                                self.install_popup_idx = None;
                            }
                        });
                        ui.add_space(8.0);
                    });
                if !open {
                    self.install_popup_idx = None;
                }
            } else {
                self.install_popup_idx = None;
            }
        }

        // ── Footer ──
        egui::TopBottomPanel::bottom("footer")
            .frame(egui::Frame {
                fill: bg_panel,
                inner_margin: egui::Margin::symmetric(12, 4),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let host_info = format!(
                        "{}  ·  {}:{}",
                        t.footer,
                        self.config.central_host,
                        self.config.central_port,
                    );
                    ui.label(
                        egui::RichText::new(host_info).size(11.0).color(gray_muted),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let lang_label = self.lang.label();
                        if ui.button(
                            egui::RichText::new(lang_label)
                                .size(11.0)
                                .color(egui::Color32::from_rgb(54, 200, 255)),
                        ).clicked() {
                            self.lang = self.lang.toggle();
                        }
                        ui.label(
                            egui::RichText::new("Language:").size(11.0).color(gray_muted),
                        );
                        ui.add_space(8.0);
                        if ui.button(
                            egui::RichText::new(t.setup_title)
                                .size(11.0)
                                .color(egui::Color32::from_rgb(54, 200, 255)),
                        ).clicked() {
                            self.show_setup = true;
                        }
                    });
                });
            });
    }
}
