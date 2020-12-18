use crate::config::Config;
use color_eyre::Result;
use futures_util::StreamExt;
use matrix_sdk::{
    events::{
        room::message::{MessageEventContent, NoticeMessageEventContent},
        AnyMessageEventContent,
    },
    identifiers::RoomId,
    SyncSettings,
};
use matrix_sdk::{Client, ClientConfig};
use once_cell::sync::{Lazy, OnceCell};
use std::convert::TryFrom;
use std::path::Path;
use std::{env, fs};
use tokio::io::{AsyncBufReadExt, BufStream};
use tokio::net::UnixListener;
use tokio::sync::Mutex;
use tracing::*;
use url::Url;
use users::get_current_uid;

mod config;

static ROOM_ID: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static CLIENT: OnceCell<Mutex<Client>> = OnceCell::new();
static PRESETS: Lazy<Vec<AnyMessageEventContent>> = Lazy::new(|| {
    vec![AnyMessageEventContent::RoomMessage(
        MessageEventContent::Notice(NoticeMessageEventContent {
            body: String::from("This is the first macro message sent using a midi controller"),
            formatted: None,
            relates_to: None,
        }),
    )]
});

async fn setup_matrix() -> Result<()> {
    info!("Beginning Matrix Setup");
    let config = Config::load().unwrap();
    let store_path_string = config.store_path.to_string();
    let store_path = Path::new(&store_path_string);
    if !store_path.exists() {
        fs::create_dir_all(store_path)?;
    }
    let client_config = ClientConfig::new().store_path(fs::canonicalize(&store_path)?);

    let homeserver_url =
        Url::parse(&config.homeserver_url).expect("Couldn't parse the homeserver URL");

    let client = Client::new_with_config(homeserver_url, client_config).unwrap();

    client
        .login(
            &config.mxid,
            &config.password,
            None,
            Some(&"macro-bot".to_string()),
        )
        .await?;
    info!("logged in as {}", config.mxid);

    let mutexed_client = Mutex::new(client.clone());
    CLIENT.set(mutexed_client);

    tokio::spawn(async move {
        info!("Starting full Sync...");
        client.sync(SyncSettings::default()).await;
    });
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Enable the logging crates
    color_eyre::install()?;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();
    setup_matrix().await?;

    let user_id = get_current_uid();
    let path_string = format!("/run/user/{}/lsc_matrix_presets.sock", user_id);
    let socket_path = Path::new(&path_string);
    if socket_path.exists() {
        fs::remove_file(socket_path)?;
    }

    // First we create a unix socket. This is required to be a) The same as the crate name and b) to be in /run
    let mut listener = UnixListener::bind(socket_path).unwrap();

    //let mut perms = fs::metadata(socket_path)?.permissions();
    //perms.set_mode(0o666);
    //fs::set_permissions(socket_path, perms)?;
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
                            info!("{}", line);
                            let split: Vec<&str> = line.split_whitespace().collect();
                            if split[0] == "select" {
                                let mut state = ROOM_ID.lock().await;
                                *state = String::from(split[1]);
                            }
                            if split[0] == "do" {
                                let preset = PRESETS[split[1].parse::<usize>().unwrap()].clone();
                                info!("preset: {:?}", preset);
                                let room_id = ROOM_ID.lock().await;
                                let cloned_id = (*room_id).clone();
                                let client = CLIENT.get().unwrap().lock().await;
                                client
                                    .room_send(&RoomId::try_from(cloned_id).unwrap(), preset, None)
                                    .await
                                    .unwrap();
                                info!("executed preset");
                            }
                            let room_id = ROOM_ID.lock().await;
                            info!("room_id: {}", room_id);
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
