use crate::plugin_actions::PluginActions;
use crate::sway::{get_workspaces, listen_for_workspace_changes};
use color_eyre::Result;
use midir::{Ignore, MidiInput, MidiOutputConnection};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::io::stdin;
use std::process::exit;
use strum::EnumIter;
use strum::IntoEnumIterator;
use swayipc_async::Connection;
use tokio::runtime::Runtime;
use tracing::*;

mod plugin_actions;
mod sway;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum LaunchpadMapping {
    Firefox = 119,
    Chats = 118,
    Email = 117,
    Coding = 116,
    Steam = 115,
    Games = 114,
    Youtube = 113,
    Test = 0,
}

impl Into<PluginActions<'_>> for LaunchpadMapping {
    fn into(self) -> PluginActions<'static> {
        match self {
            LaunchpadMapping::Firefox => PluginActions::SwayWorkspace("1:\u{e007}"),
            LaunchpadMapping::Chats => PluginActions::SwayWorkspace("2:\u{f086}"),
            LaunchpadMapping::Email => PluginActions::SwayWorkspace("3:\u{f0e0}"),
            LaunchpadMapping::Coding => PluginActions::SwayWorkspace("5:\u{f1c9}"),
            LaunchpadMapping::Steam => PluginActions::SwayWorkspace("11:\u{f1b6}"),
            LaunchpadMapping::Games => PluginActions::SwayWorkspace("12:\u{f11b}"),
            LaunchpadMapping::Youtube => PluginActions::SwayWorkspace("13:\u{f167}"),
            LaunchpadMapping::Test => PluginActions::ExamplePlugin,
        }
    }
}

impl TryFrom<i32> for LaunchpadMapping {
    type Error = ();

    fn try_from(input: i32) -> Result<LaunchpadMapping, Self::Error> {
        match input {
            119 => Ok(LaunchpadMapping::Firefox),
            118 => Ok(LaunchpadMapping::Chats),
            117 => Ok(LaunchpadMapping::Email),
            116 => Ok(LaunchpadMapping::Coding),
            115 => Ok(LaunchpadMapping::Steam),
            114 => Ok(LaunchpadMapping::Games),
            113 => Ok(LaunchpadMapping::Youtube),
            0 => Ok(LaunchpadMapping::Test),
            _ => Err(()),
        }
    }
}

fn reset_colors(conn_out: &mut MidiOutputConnection) {
    conn_out.send(&[176, 0, 0]).unwrap();
    for value in LaunchpadMapping::iter() {
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
                    if let Ok(v) = LaunchpadMapping::try_from(message[1] as i32) {
                        rt.block_on(async {
                            if let Err(e) = Into::<PluginActions>::into(v).run().await {
                                error!("Error while executing action: {}", e);
                            }
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
