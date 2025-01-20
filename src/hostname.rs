use std::fs;

pub async fn get_hostname() -> String {
    let hostname = fs::read_to_string("/etc/hostname");
        hostname.unwrap_or_else(|_| "unknown".to_string()).trim().to_string()
}
