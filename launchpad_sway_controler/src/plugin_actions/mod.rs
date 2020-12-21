use crate::reset_colors;
use crate::utils::globals::{LAYER_NUMBER, PRESELECTED_LAYER_NUMBER, SHOWING_NUMBER};
use crate::utils::show_number;
use color_eyre::Result;
use dbus::blocking::Connection as DBUSConnection;
use midir::MidiOutput;
use notify_rust::{Notification, Timeout};
use std::sync::atomic::Ordering;
use std::time::Duration;
use swayipc_async::Connection;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;
use tracing::*;
use users::get_current_uid;

pub enum PluginActions<'a> {
    SwayWorkspace(&'a str),
    SwayBack,
    ExamplePlugin,
    SelectMatrixRoomByID(&'a str),
    RunMatrixPreset(&'a str),
    ShowNumber(usize),
    SelectLayer,
    MumbleToggleMute,
    MumbleToggleDeaf,
    SpotifyNextTrack,
    SpotifyPrevTrack,
    SpotifyPause,
}

impl PluginActions<'_> {
    pub async fn run(&self) -> Result<()> {
        let midi_out = MidiOutput::new("My Test Output")?;
        let midi_out_ports = midi_out.ports();
        let out_port = midi_out_ports.get(1).unwrap();
        let mut conn_out = midi_out.connect(out_port, "midir-test").unwrap();
        match self {
            PluginActions::SwayWorkspace(workspace) => {
                notification(format!("Changed to workspace: {}", workspace).as_str());
                let mut connection = Connection::new().await?;
                connection
                    .run_command(format!("workspace --no-auto-back-and-forth {}", workspace))
                    .await
                    .unwrap();
            }
            PluginActions::SwayBack => {
                let mut connection = Connection::new().await?;
                connection
                    .run_command("workspace back_and_forth")
                    .await
                    .unwrap();
            }
            PluginActions::ExamplePlugin => {
                let user_id = get_current_uid();
                let path_string = format!("/run/user/{}/lsc_example_plugin.sock", user_id);

                let mut socket = UnixStream::connect(&path_string).await?;

                socket.write(b"test\n").await?;
            }
            PluginActions::SelectMatrixRoomByID(room_id) => {
                let user_id = get_current_uid();
                let path_string = format!("/run/user/{}/lsc_matrix_presets.sock", user_id);
                let mut socket = UnixStream::connect(&path_string).await?;

                socket
                    .write(format!("select {}\n", room_id).as_bytes())
                    .await?;
            }
            PluginActions::RunMatrixPreset(preset_id) => {
                let user_id = get_current_uid();
                let path_string = format!("/run/user/{}/lsc_matrix_presets.sock", user_id);
                let mut socket = UnixStream::connect(&path_string).await?;

                socket
                    .write(format!("do {}\n", preset_id).as_bytes())
                    .await?;
            }
            PluginActions::ShowNumber(number) => {
                notification(format!("Number shown: {}", number).as_str());
                show_number(&mut conn_out, *number);
            }
            PluginActions::SelectLayer => {
                notification(
                    format!(
                        "Selected layer: {}",
                        PRESELECTED_LAYER_NUMBER.load(Ordering::SeqCst)
                    )
                    .as_str(),
                );
                LAYER_NUMBER.store(
                    PRESELECTED_LAYER_NUMBER.load(Ordering::SeqCst),
                    Ordering::SeqCst,
                );
                SHOWING_NUMBER.store(false, Ordering::SeqCst);
                reset_colors(&mut conn_out);
            }
            PluginActions::MumbleToggleMute => {
                let conn = DBUSConnection::new_session()?;
                let proxy = conn.with_proxy(
                    "net.sourceforge.mumble.mumble",
                    "/",
                    Duration::from_millis(5000),
                );
                let (muted,): (bool,) =
                    proxy.method_call("net.sourceforge.mumble.Mumble", "isSelfMuted", ())?;
                notification(format!("Changed muted to: {}", !muted).as_str());
                let (): () = proxy.method_call(
                    "net.sourceforge.mumble.Mumble",
                    "setSelfMuted",
                    (!muted,),
                )?;
            }
            PluginActions::MumbleToggleDeaf => {
                let conn = DBUSConnection::new_session()?;
                let proxy = conn.with_proxy(
                    "net.sourceforge.mumble.mumble",
                    "/",
                    Duration::from_millis(5000),
                );
                let (deaf,): (bool,) =
                    proxy.method_call("net.sourceforge.mumble.Mumble", "isSelfDeaf", ())?;
                notification(format!("Changed Deaf to: {}", !deaf).as_str());
                let (): () =
                    proxy.method_call("net.sourceforge.mumble.Mumble", "setSelfDeaf", (!deaf,))?;
            }
            PluginActions::SpotifyNextTrack => {
                let conn = DBUSConnection::new_session()?;
                let proxy = conn.with_proxy(
                    "org.mpris.MediaPlayer2.spotify",
                    "/org/mpris/MediaPlayer2",
                    Duration::from_millis(5000),
                );
                let (): () = proxy.method_call("org.mpris.MediaPlayer2.Player", "Next", ())?;
            }
            PluginActions::SpotifyPrevTrack => {
                let conn = DBUSConnection::new_session()?;
                let proxy = conn.with_proxy(
                    "org.mpris.MediaPlayer2.spotify",
                    "/org/mpris/MediaPlayer2",
                    Duration::from_millis(5000),
                );
                // We send it twice as it first just goes back to the track start
                let (): () = proxy.method_call("org.mpris.MediaPlayer2.Player", "Previous", ())?;
                let (): () = proxy.method_call("org.mpris.MediaPlayer2.Player", "Previous", ())?;
            }
            PluginActions::SpotifyPause => {
                let conn = DBUSConnection::new_session()?;
                let proxy = conn.with_proxy(
                    "org.mpris.MediaPlayer2.spotify",
                    "/org/mpris/MediaPlayer2",
                    Duration::from_millis(5000),
                );
                let (): () = proxy.method_call("org.mpris.MediaPlayer2.Player", "PlayPause", ())?;
            }
        }
        Ok(())
    }
}

fn notification(text: &str) {
    Notification::new()
        .appname("Launchpad Macros")
        .summary("Launchpad Macros")
        .body(text)
        .timeout(Timeout::Milliseconds(700)) //milliseconds
        .show()
        .unwrap();
}
