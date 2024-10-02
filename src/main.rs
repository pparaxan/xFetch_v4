#![allow(unused_must_use)]

use std::io::{self, Write};
use tokio::{join, task::spawn};

pub mod distro;
#[macro_use]
pub mod macros;
pub mod packages;
pub mod shell;
pub mod uptime;

#[tokio::main]
async fn main() -> io::Result<()> {
    let distro_thread = spawn(async { distro::get_distro_name().await });
    let name_thread = spawn(async { get_env_var!("USER") });
    let packages_thread = spawn(async { packages::get_current_packages() });
    let shell_thread = spawn(async { shell::get_current_shell().await });
    let uptime_thread = spawn(async { uptime::get_uptime().await });

    let (usr, distro, shell, pkg, uptime) = join!(
        name_thread,
        distro_thread,
        shell_thread,
        packages_thread,
        uptime_thread
    );

    let distro = distro.unwrap();
    let pkg = pkg.unwrap();
    let shell = shell.unwrap();
    let uptime = uptime.unwrap();
    let usr = usr.unwrap();
    let mut handle = io::stdout().lock(); // Lock stdout for slightly faster writing

    writeln!(handle, "\x1B[0;31m\x1B[1mx\x1B[0;36mFetch\x1B[0m - {}", usr);
    writeln_to_handle_if_not_empty!(handle, "Shell", shell);
    writeln_to_handle_if_not_empty!(handle, "PKGs", pkg);
    writeln_to_handle_if_not_empty!(handle, "Uptime", uptime);
    writeln_to_handle_if_not_empty!(handle, "Distro", distro);

    drop(handle);
    Ok(())
}
