use crate::mappings::LaunchpadMapping;
use crate::plugin_actions::PluginActions;
use crate::sway::{get_workspaces, listen_for_workspace_changes};
use crate::utils::globals::{CURRENT_WORKSPACE, PRESELECTED_LAYER_NUMBER};
use color_eyre::Result;
use midir::{Ignore, MidiInput, MidiOutputConnection};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::io::stdin;
use std::process::exit;
use std::sync::atomic::Ordering;
use strum::IntoEnumIterator;
use swayipc_async::Connection;
use tokio::runtime::Runtime;
use tracing::*;

mod mappings;
mod plugin_actions;
mod sway;
mod utils;

fn reset_colors(conn_out: &mut MidiOutputConnection) {
    conn_out.send(&[176, 0, 0]).unwrap();
    for value in LaunchpadMapping::iter() {
        conn_out
            .send(&[144, (value as i32).try_into().unwrap(), 15])
            .unwrap();
    }
    conn_out.send(&[144, 24, 15]).unwrap();
    conn_out.send(&[144, 8, 15]).unwrap();
    conn_out.send(&[144, 40, 15]).unwrap();
    if let Ok(workspace) =
        sway::WorkspaceLaunchpadMapping::try_from(CURRENT_WORKSPACE.load(Ordering::SeqCst) as i32)
    {
        conn_out
            .send(&[144, (workspace as i32).try_into().unwrap(), 28])
            .unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();

    // establish a connection to sway over a unix socket
    let mut connection = Connection::new().await?;

    get_workspaces(&mut connection).await?;
    tokio::spawn(async {
        listen_for_workspace_changes().await.unwrap();
    });
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    let midi_in_ports = midi_in.ports();

    let in_port = midi_in_ports.get(1).unwrap();

    info!("Opening connections");

    // One could get the log back here out of the error
    let conn_in = midi_in
        .connect(
            in_port,
            "midir-test",
            move |_, message, _| {
                if message[2] == 127 {
                    debug!("{:?}", message);
                    if (message[1] as i32) == 8 {
                        if PRESELECTED_LAYER_NUMBER.load(Ordering::SeqCst) < 9 {
                            PRESELECTED_LAYER_NUMBER.fetch_add(1, Ordering::SeqCst);
                        }

                        run_action(
                            Runtime::new().unwrap(),
                            PluginActions::ShowNumber(
                                PRESELECTED_LAYER_NUMBER.load(Ordering::SeqCst) as usize,
                            ),
                        );
                    }
                    if (message[1] as i32) == 24 {
                        if PRESELECTED_LAYER_NUMBER.load(Ordering::SeqCst) > 0 {
                            PRESELECTED_LAYER_NUMBER.fetch_sub(1, Ordering::SeqCst);
                        }

                        run_action(
                            Runtime::new().unwrap(),
                            PluginActions::ShowNumber(
                                PRESELECTED_LAYER_NUMBER.load(Ordering::SeqCst) as usize,
                            ),
                        );
                    }
                    if (message[1] as i32) == 40 {
                        run_action(Runtime::new().unwrap(), PluginActions::SelectLayer);
                    }

                    if let Ok(v) = LaunchpadMapping::try_from(message[1] as i32) {
                        run_action(Runtime::new().unwrap(), v.into());
                    }
                }
            },
            (),
        )
        .unwrap();

    info!("Connections open, enter `q` to exit ...");

    loop {
        input.clear();
        stdin().read_line(&mut input)?;
        if input.trim() == "q" {
            info!("Closing connections");
            conn_in.close();
            //conn_out.close();
            info!("Connections closed");
            exit(0);
        }
    }
}

fn run_action(mut rt: Runtime, action: PluginActions) {
    rt.block_on(async {
        if let Err(e) = action.run().await {
            error!("Error while executing action: {}", e);
        }
    });
}
