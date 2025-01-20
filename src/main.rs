#![allow(unused_must_use)]

use std::io::{self, Write};
use tokio::{join, task::spawn}; // runtime::Handle

pub mod distro;
#[macro_use]
pub mod macros;
pub mod packages;
pub mod shell;
pub mod uptime;
pub mod hostname;

#[tokio::main]
async fn main() -> io::Result<()> {
    let distro_thread = spawn(async { distro::get_distro_name().await });
    let username_thread = spawn(async { get_env_var!("USER") });
    let hostname_thread = spawn(async { hostname::get_hostname().await });
    let packages_thread = spawn(async { packages::get_current_packages().await });
    let shell_thread = spawn(async { shell::get_current_shell().await });
    let uptime_thread = spawn(async { uptime::get_uptime().await });
    let terminal_thread = spawn(async { get_env_var!("TERM") });

    let (username, hostname, distro, shell, pkg, uptime, terminal) = join!(
        username_thread,
        hostname_thread,
        distro_thread,
        shell_thread,
        packages_thread,
        uptime_thread,
        terminal_thread
    );

    let distro = distro.unwrap();
    let pkg = pkg.unwrap();
    let shell = shell.unwrap();
    let uptime = uptime.unwrap();
    let username = username.unwrap();
    let hostname = hostname.unwrap();
    let terminal = terminal.unwrap();
    let mut handle = io::stdout().lock(); // Lock stdout for slightly faster writing

    writeln!(handle, "{}@{}", username, hostname);
    writeln_to_handle_if_not_empty!(handle, "", distro);
    writeln_to_handle_if_not_empty!(handle, "", shell);
    writeln_to_handle_if_not_empty!(handle, "󰏗", pkg); // pacman pkgs only
    writeln_to_handle_if_not_empty!(handle, "", uptime);
    writeln_to_handle_if_not_empty!(handle, "", terminal);

    drop(handle);
    Ok(())
}
