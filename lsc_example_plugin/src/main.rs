use color_eyre::Result;
use futures_util::StreamExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, fs};
use tokio::io::{AsyncBufReadExt, BufStream};
use tokio::net::UnixListener;
use tracing::*;
use users::get_current_uid;

#[tokio::main]
async fn main() -> Result<()> {
    // Enable the logging crates
    color_eyre::install()?;
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();
    let user_id = get_current_uid();
    let path_string = format!("/run/user/{}/lsc_example_plugin.sock", user_id);
    let socket_path = Path::new(&path_string);
    if socket_path.exists() {
        fs::remove_file(socket_path)?;
    }

    // First we create a unix socket. This is required to be a) The same as the crate name and b) to be in /run
    let mut listener = UnixListener::bind(socket_path).unwrap();

    let mut perms = fs::metadata(socket_path)?.permissions();
    perms.set_mode(0o666);
    fs::set_permissions(socket_path, perms)?;
    // This listens for new connections
    while let Some(stream) = listener.next().await {
        match stream {
            Ok(stream) => {
                tokio::spawn(async move {
                    println!("new client!");
                    let buffer = BufStream::new(stream);
                    let mut lines = buffer.lines();
                    while let Ok(line) = lines.next_line().await {
                        if let Some(line) = line {
                            println!("{}", line);
                        }
                    }
                });
            }
            Err(e) => {
                error!("Unix socket connection failed: {}", e);
            }
        }
    }
    Ok(())
}
