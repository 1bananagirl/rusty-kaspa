use crate::fd_budget;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub system_id: Vec<u8>,
    pub cpu_physical_cores: u16,
    pub total_memory: u64,
    pub fd_limit: u32,
}

static SYSTEM_INFO: OnceLock<SystemInfo> = OnceLock::new();

impl Default for SystemInfo {
    fn default() -> Self {
        let system_info = SYSTEM_INFO.get_or_init(|| {
            let mut system = sysinfo::System::new();
            system.refresh_memory();
            let cpu_physical_cores = num_cpus::get() as u16;
            let total_memory = system.total_memory();
            // ~
            let fd_limit = fd_budget::limit() as u32;
            // ~
            let some_id = if let Ok(mut file) = File::open("/etc/machine-id") {
                // fetch the system id from /etc/machine-id
                let mut machine_id = String::new();
                file.read_to_string(&mut machine_id).ok();
                machine_id
            } else if let Ok(Some(mac)) = mac_address::get_mac_address() {
                // fallback on the mac address
                mac.to_string()
            } else {
                // fallback on a random value - should never happen
                uuid::Uuid::new_v4().to_string()
            };
            let mut sha256 = Sha256::default();
            sha256.update(some_id.as_bytes());
            let system_id = sha256.finalize().to_vec();

            SystemInfo { system_id, cpu_physical_cores, total_memory, fd_limit }
        });
        (*system_info).clone()
    }
}

impl AsRef<SystemInfo> for SystemInfo {
    fn as_ref(&self) -> &SystemInfo {
        self
    }
}
