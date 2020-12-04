use crate::reset_colors;
use color_eyre::Result;
use futures_util::stream::StreamExt;
use midir::MidiOutput;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use strum::EnumIter;
use swayipc_async::{Connection, Event, EventType};

pub async fn get_workspaces(connection: &mut Connection) -> Result<()> {
    let workspaces = connection.get_workspaces().await?;
    println!("{:#?}", workspaces);

    for space in workspaces {
        if space.focused {
            let midi_out = MidiOutput::new("My Test Output")?;
            let midi_out_ports = midi_out.ports();
            let out_port = midi_out_ports.get(1).unwrap();
            let mut conn_out = midi_out.connect(out_port, "midir-test").unwrap();
            if let Ok(workspace) = WorkspaceLaunchpadMapping::try_from(space.num) {
                reset_colors(&mut conn_out);
                conn_out
                    .send(&[144, (workspace as i32).try_into().unwrap(), 28])
                    .unwrap();
            }
        }
    }
    Ok(())
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum LaunchpadWorkspaceMapping {
    Firefox,
    Chats,
    Email,
    Steam,
    Games,
    Youtube,
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
pub enum WorkspaceLaunchpadMapping {
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

pub async fn listen_for_workspace_changes() -> Result<()> {
    // subscribe to a workspace events.
    let subs = [EventType::Workspace];
    let mut events = Connection::new().await?.subscribe(&subs).await?;
    let midi_out = MidiOutput::new("My Test Output")?;
    let midi_out_ports = midi_out.ports();
    let out_port = midi_out_ports.get(1).unwrap();
    let mut conn_out = midi_out.connect(out_port, "midir-test").unwrap();
    while let Some(event) = events.next().await {
        if let Ok(ref event) = event {
            if let Event::Workspace(ref w) = event {
                if let Some(v) = &w.current {
                    if let Some(num) = v.num {
                        // TODO make ? work
                        if let Ok(workspace) = WorkspaceLaunchpadMapping::try_from(num) {
                            reset_colors(&mut conn_out);
                            conn_out
                                .send(&[144, (workspace as i32).try_into().unwrap(), 28])
                                .unwrap();
                        }
                    }
                }
            }
        }
        //println!("{:?}", event)
    }

    Ok(())
}
