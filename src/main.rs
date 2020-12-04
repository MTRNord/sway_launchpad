use crate::sway::{
    get_workspaces, listen_for_workspace_changes, LaunchpadWorkspaceMapping,
    WorkspaceLaunchpadMapping,
};
use color_eyre::Result;
use midir::{Ignore, MidiInput, MidiOutputConnection};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::io::stdin;
use std::process::exit;
use strum::IntoEnumIterator;
use swayipc_async::Connection;
use tokio::runtime::Runtime;
use tracing::*;

mod sway;

fn reset_colors(conn_out: &mut MidiOutputConnection) {
    for value in WorkspaceLaunchpadMapping::iter() {
        conn_out
            .send(&[144, (value as i32).try_into().unwrap(), 15])
            .unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    if let Err(_) = env::var("RUST_LOG") {
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
    /*for (i, p) in midi_in_ports.iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p)?);

        // TODO get by name
    }*/
    let in_port = midi_in_ports.get(1).unwrap();

    info!("Opening connections");

    // Create the runtime
    let rt = Runtime::new()?;

    // One could get the log back here out of the error
    let conn_in = midi_in
        .connect(
            in_port,
            "midir-test",
            move |_, message, _| {
                if message[2] == 127 {
                    debug!("{:?}", message);
                    if let Ok(v) = LaunchpadWorkspaceMapping::try_from(message[1] as i32) {
                        rt.block_on(async {
                            let mut connection = Connection::new().await.unwrap();
                            connection
                                .run_command(format!("workspace --no-auto-back-and-forth {}", v))
                                .await
                                .unwrap();
                        });
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
