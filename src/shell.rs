use std::{env, path::Path};

pub async fn get_current_shell() -> String {
    let shell = env::var("SHELL").unwrap();
    let path = Path::new(&shell);
    let name = path.file_name().unwrap_or_else(|| path.as_os_str()).to_string_lossy();
    name.to_string()
}
