#[derive(Clone, Copy, PartialEq)]
pub enum Lang {
    En,
    Es,
}

impl Lang {
    pub fn toggle(self) -> Self {
        match self {
            Lang::En => Lang::Es,
            Lang::Es => Lang::En,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Lang::En => "EN",
            Lang::Es => "ES",
        }
    }

    pub fn strings(self) -> Strings {
        match self {
            Lang::En => Strings::en(),
            Lang::Es => Strings::es(),
        }
    }

}

pub struct Strings {
    // Modal title & description
    pub register_server: &'static str,
    pub enter_details: &'static str,

    // Network recommendation
    pub secure_network: &'static str,
    pub secure_network_msg: &'static str,

    // Form labels
    pub server_id: &'static str,
    pub ip_address: &'static str,
    pub ssh_user: &'static str,
    pub ssh_private_key: &'static str,
    pub passphrase: &'static str,
    pub use_passphrase_label: &'static str,
    pub cancel: &'static str,
    pub deploy_agent: &'static str,

    // Placeholders
    pub hint_server_id: &'static str,
    pub hint_ssh_key: &'static str,

    // IP warning template — use ip_warning() to fill {ip}
    ip_warning_template: &'static str,

    // Setup
    pub setup_title: &'static str,
    pub setup_desc: &'static str,
    pub setup_select_ip: &'static str,
    pub setup_manual: &'static str,
    pub setup_save: &'static str,
    pub setup_warning: &'static str,

    // Connection result
    pub conn_success: String,
    pub retry: &'static str,
    pub accept: &'static str,

    // Local storage note
    pub local_storage_note: &'static str,

    // Top bar
    pub register_btn: &'static str,

    // Welcome card
    pub welcome_title: &'static str,
    pub welcome_subtitle: &'static str,

    // Table headers
    pub col_name: &'static str,
    pub col_ip: &'static str,
    pub col_agent_version: &'static str,
    pub col_ram: &'static str,
    pub col_cpu: &'static str,
    pub col_status: &'static str,

    // Empty state
    pub no_servers: &'static str,
    pub no_servers_hint: &'static str,

    // Status
    pub online: &'static str,
    pub unknown: &'static str,

    // Footer
    pub footer: &'static str,

    // Validation error templates — use helper methods to fill placeholders
    pub err_id_required: &'static str,
    pub err_ip_required: &'static str,
    pub err_user_required: &'static str,
    pub err_key_required: &'static str,
    err_key_not_found: &'static str,
    err_id_exists: &'static str,
    err_ip_exists: &'static str,
}

impl Strings {
    fn en() -> Self {
        Strings {
            register_server: "Register server",
            enter_details: "Enter the server details to deploy the agent",

            secure_network: "⚠ Secure network recommended",
            secure_network_msg: "We strongly recommend using a private network (e.g., WireGuard)",

            server_id: "Server ID *",
            ip_address: "IP address *",
            ssh_user: "SSH user *",
            ssh_private_key: "SSH private key *",
            passphrase: "Passphrase:",
            use_passphrase_label: "Key uses a passphrase",
            cancel: "Cancel",
            deploy_agent: "Deploy agent",

            hint_server_id: "web-server",
            hint_ssh_key: "~/.ssh/id_rsa",

            ip_warning_template: "⚠ \"{ip}\" is not a private IP. It is recommended to only connect within private networks.",

            setup_title: "Configure central host",
            setup_desc: "Select the IP address that agents will use to connect to this computer.",
            setup_select_ip: "Detected IPs:",
            setup_manual: "Or enter manually:",
            setup_save: "Save",
            setup_warning: "⚠ This IP is not a private address. It is recommended to use a secure network (e.g., WireGuard).",

            conn_success: "✅ Connection successful".into(),
            retry: "Retry",
            accept: "Accept",
            local_storage_note: "This information is stored locally and will not be transmitted or shared outside this device.",

            register_btn: "+ Register server",

            welcome_title: "Welcome",
            welcome_subtitle: "Remote server management and monitoring",

            col_name: "Name",
            col_ip: "IP",
            col_agent_version: "Agent version",
            col_ram: "RAM",
            col_cpu: "CPU",
            col_status: "Status",

            no_servers: "No servers registered yet.",
            no_servers_hint: "Press \"Register server\" to get started",

            online: "Online",
            unknown: "—",

            footer: "Made with love by Savne · info@savne.net",

            err_id_required: "Server ID is required.",
            err_ip_required: "IP address is required.",
            err_user_required: "SSH user is required.",
            err_key_required: "SSH key path is required.",
            err_key_not_found: "SSH key not found: {key}",
            err_id_exists: "Server ID \"{id}\" already exists.",
            err_ip_exists: "Server IP \"{ip}\" already exists.",
        }
    }

    fn es() -> Self {
        Strings {
            register_server: "Registrar servidor",
            enter_details: "Ingrese los detalles del servidor para desplegar el agente",

            secure_network: "⚠ Red segura recomendada",
            secure_network_msg: "Recomendamos usar una red privada (ej. WireGuard)",

            server_id: "ID del servidor *",
            ip_address: "Dirección IP *",
            ssh_user: "Usuario SSH *",
            ssh_private_key: "Clave privada SSH *",
            passphrase: "Contraseña:",
            use_passphrase_label: "La clave usa contraseña",
            cancel: "Cancelar",
            deploy_agent: "Desplegar agente",

            hint_server_id: "web-server",
            hint_ssh_key: "~/.ssh/id_rsa",

            ip_warning_template: "⚠ \"{ip}\" no es una IP privada. Se recomienda conectar solo dentro de redes privadas.",

            setup_title: "Configurar host central",
            setup_desc: "Seleccione la dirección IP que los agentes usarán para conectarse a este equipo.",
            setup_select_ip: "IPs detectadas:",
            setup_manual: "O ingrese manualmente:",
            setup_save: "Guardar",
            setup_warning: "⚠ Esta IP no es una dirección privada. Se recomienda usar una red segura (ej. WireGuard).",

            conn_success: "✅ Conexión exitosa".into(),
            retry: "Reintentar",
            accept: "Aceptar",
            local_storage_note: "Esta información se almacena localmente y no será transmitida ni compartida fuera de este dispositivo.",

            register_btn: "+ Registrar servidor",

            welcome_title: "Bienvenido",
            welcome_subtitle: "Administración y monitoreo remoto de servidores",

            col_name: "Nombre",
            col_ip: "IP",
            col_agent_version: "Versión agente",
            col_ram: "RAM",
            col_cpu: "CPU",
            col_status: "Estado",

            no_servers: "Aún no hay servidores registrados.",
            no_servers_hint: "Presione \"Registrar servidor\" para comenzar",

            online: "En línea",
            unknown: "—",

            footer: "Hecho con amor por Savne · info@savne.net",

            err_id_required: "El ID del servidor es obligatorio.",
            err_ip_required: "La dirección IP es obligatoria.",
            err_user_required: "El usuario SSH es obligatorio.",
            err_key_required: "La ruta de la clave SSH es obligatoria.",
            err_key_not_found: "Clave SSH no encontrada: {key}",
            err_id_exists: "El ID del servidor \"{id}\" ya existe.",
            err_ip_exists: "La IP del servidor \"{ip}\" ya existe.",
        }
    }

    pub fn ip_warning(&self, ip: &str) -> String {
        self.ip_warning_template.replace("{ip}", ip)
    }

    pub fn err_key_not_found_msg(&self, key: &str) -> String {
        self.err_key_not_found.replace("{key}", key)
    }

    pub fn err_id_exists_msg(&self, id: &str) -> String {
        self.err_id_exists.replace("{id}", id)
    }

    pub fn err_ip_exists_msg(&self, ip: &str) -> String {
        self.err_ip_exists.replace("{ip}", ip)
    }
}
