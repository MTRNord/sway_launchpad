use color_eyre::Result;
use std::io::stdin;
use strum::EnumIter;

use futures_util::stream::StreamExt;
use midir::{Ignore, MidiInput, MidiOutput, MidiOutputConnection};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use std::process::exit;
use swayipc_async::{Connection, Event, EventType};
use tokio::runtime::Runtime;use strum::IntoEnumIterator;

fn reset_colors(conn_out: &mut MidiOutputConnection) {
    for value in WorkspaceLaunchpadMapping::iter() {
        conn_out.send(&[144, (value as i32).try_into().unwrap(), 15]).unwrap();
    }
}

async fn get_workspaces(connection: &mut Connection) -> Result<()> {
    // request and print the i3 version
    println!("{:#?}", connection.get_workspaces().await?);
    Ok(())
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
enum LaunchpadWorkspaceMapping {
    Firefox,
    Chats,
    Email,
    Steam,
    Games,
    Youtube
}

impl Display for LaunchpadWorkspaceMapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Firefox => write!(f, "1:\u{e007}"),
            Self::Chats => write!(f, "2:\u{f086}"),
            Self::Email => write!(f, "3:\u{f0e0}"),
            Self::Steam => write!(f, "11:\u{f1b6}"),
            Self::Games => write!(f, "12:\u{f11b}"),
            Self::Youtube => write!(f, "13:\u{f167}"),
            _ => Ok(()),
        }
    }
}

impl TryFrom<i32> for LaunchpadWorkspaceMapping {
    type Error = ();

    fn try_from(input: i32) -> Result<LaunchpadWorkspaceMapping, Self::Error> {
        match input {
            119 => Ok(LaunchpadWorkspaceMapping::Firefox),
            118 => Ok(LaunchpadWorkspaceMapping::Chats),
            117 => Ok(LaunchpadWorkspaceMapping::Email),
            116 => Ok(LaunchpadWorkspaceMapping::Steam),
            115 => Ok(LaunchpadWorkspaceMapping::Games),
            114 => Ok(LaunchpadWorkspaceMapping::Youtube),
            _ => Err(()),
        }
    }
}
#[non_exhaustive]
#[derive(Debug, Copy, Clone, EnumIter)]
enum WorkspaceLaunchpadMapping {
    Firefox = 119,
    Chats = 118,
    Email = 117,
    Steam = 116,
    Games = 115,
    Youtube = 114,
}

impl TryFrom<i32> for WorkspaceLaunchpadMapping {
    type Error = ();

    fn try_from(input: i32) -> Result<WorkspaceLaunchpadMapping, Self::Error> {
        match input {
            1 => Ok(WorkspaceLaunchpadMapping::Firefox),
            2 => Ok(WorkspaceLaunchpadMapping::Chats),
            3 => Ok(WorkspaceLaunchpadMapping::Email),
            11 => Ok(WorkspaceLaunchpadMapping::Steam),
            12 => Ok(WorkspaceLaunchpadMapping::Games),
            13 => Ok(WorkspaceLaunchpadMapping::Youtube),
            _ => Err(()),
        }
    }
}

async fn listen_for_workspace_changes() -> Result<()> {
    // subscribe to a workspace events.
    let subs = [EventType::Workspace];
    let mut events = Connection::new().await?.subscribe(&subs).await?;
    while let Some(event) = events.next().await {
        if let Ok(ref event) = event {
            if let Event::Workspace(ref w) = event {
                println!("{:?}", w.current);
                // TODO change light
            }
        }
        //println!("{:?}", event)
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // establish a connection to sway over a unix socket
    let mut connection = Connection::new().await?;

    get_workspaces(&mut connection).await?;
    tokio::spawn(async {
        listen_for_workspace_changes().await;
    });
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("My Test Output")?;

    let midi_in_ports = midi_in.ports();
    /*for (i, p) in midi_in_ports.iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p)?);

        // TODO get by name
    }*/
    let in_port = midi_in_ports.get(1).unwrap();

    let midi_out_ports = midi_out.ports();
    let out_port = midi_out_ports.get(1).unwrap();

    println!("\nOpening connections");

    let mut conn_out = midi_out.connect(out_port, "midir-test").unwrap();
    reset_colors(&mut conn_out);

    // Create the runtime
    let rt = Runtime::new()?;

    // One could get the log back here out of the error
    let conn_in = midi_in
        .connect(
            in_port,
            "midir-test",
            move |stamp, message, _| {
                // The last of the three callback parameters is the object that we pass in as last parameter of `connect`.
                println!("{}: {:?} (len = {})", stamp, message, message.len());
                if message[2] == 127 {
                    if let Ok(v) = LaunchpadWorkspaceMapping::try_from(message[1] as i32) {

                        rt.block_on(async {
                            let mut connection = Connection::new().await.unwrap();
                            connection
                                .run_command(format!(
                                    "workspace --no-auto-back-and-forth {}",
                                    v
                                ))
                                .await
                                .unwrap();
                        });
                        reset_colors(&mut conn_out);
                        conn_out.send(&[144, message[1], 28]).unwrap();
                    }
                }
            },
            (),
        )
        .unwrap();

    println!("Connections open, enter `q` to exit ...");

    loop {
        input.clear();
        stdin().read_line(&mut input)?;
        if input.trim() == "q" {
            println!("Closing connections");
            conn_in.close();
            //conn_out.close();
            println!("Connections closed");
            exit(0);
        }
    }
}
