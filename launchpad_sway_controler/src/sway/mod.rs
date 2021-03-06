use crate::reset_colors;
use crate::utils::globals::{CURRENT_WORKSPACE, SHOWING_NUMBER};
use color_eyre::Result;
use futures_util::stream::StreamExt;
use midir::MidiOutput;
use std::convert::TryFrom;
use std::sync::atomic::Ordering;
use swayipc_async::{Connection, Event, EventType, WorkspaceChange};
use tracing::*;

pub async fn get_workspaces(connection: &mut Connection) -> Result<()> {
    let workspaces = connection.get_workspaces().await?;
    debug!("{:#?}", workspaces);

    for space in workspaces {
        if space.focused {
            let midi_out = MidiOutput::new("My Test Output")?;
            let midi_out_ports = midi_out.ports();
            let out_port = midi_out_ports.get(1).unwrap();
            let mut conn_out = midi_out.connect(out_port, "midir-test").unwrap();
            CURRENT_WORKSPACE.store(space.num as i64, Ordering::SeqCst);
            if !SHOWING_NUMBER.load(Ordering::SeqCst) {
                reset_colors(&mut conn_out);
            }
        }
    }
    Ok(())
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum WorkspaceLaunchpadMapping {
    Firefox = 119,
    Chats = 118,
    Email = 117,
    Coding = 116,
    Steam = 115,
    Games = 114,
    Youtube = 113,
    Spotify = 112,
}

impl WorkspaceLaunchpadMapping {
    pub fn get_layer(&self) -> i64 {
        match self {
            WorkspaceLaunchpadMapping::Firefox => 0,
            WorkspaceLaunchpadMapping::Chats => 0,
            WorkspaceLaunchpadMapping::Email => 0,
            WorkspaceLaunchpadMapping::Coding => 0,
            WorkspaceLaunchpadMapping::Steam => 0,
            WorkspaceLaunchpadMapping::Games => 0,
            WorkspaceLaunchpadMapping::Youtube => 0,
            WorkspaceLaunchpadMapping::Spotify => 0,
        }
    }
}

impl TryFrom<i32> for WorkspaceLaunchpadMapping {
    type Error = ();

    fn try_from(input: i32) -> Result<WorkspaceLaunchpadMapping, Self::Error> {
        match input {
            1 => Ok(WorkspaceLaunchpadMapping::Firefox),
            2 => Ok(WorkspaceLaunchpadMapping::Chats),
            3 => Ok(WorkspaceLaunchpadMapping::Email),
            5 => Ok(WorkspaceLaunchpadMapping::Coding),
            11 => Ok(WorkspaceLaunchpadMapping::Steam),
            12 => Ok(WorkspaceLaunchpadMapping::Games),
            13 => Ok(WorkspaceLaunchpadMapping::Youtube),
            42 => Ok(WorkspaceLaunchpadMapping::Spotify),
            _ => Err(()),
        }
    }
}

pub async fn listen_for_workspace_changes() -> Result<()> {
    // subscribe to a workspace events.
    let subs = [EventType::Workspace];
    let mut events = Connection::new().await?.subscribe(&subs).await?;
    let midi_out = MidiOutput::new("My Test Output").unwrap();
    let midi_out_ports = midi_out.ports();
    let out_port = midi_out_ports.get(1).unwrap();
    let mut conn_out = midi_out.connect(out_port, "midir-test").unwrap();
    while let Some(event) = events.next().await {
        if let Ok(ref event) = event {
            if let Event::Workspace(ref w) = event {
                if w.change == WorkspaceChange::Focus {
                    if let Some(v) = &w.current {
                        if let Some(num) = v.num {
                            CURRENT_WORKSPACE.store(num as i64, Ordering::SeqCst);
                            // TODO make ? work
                            if !SHOWING_NUMBER.load(Ordering::SeqCst) {
                                reset_colors(&mut conn_out);
                            }
                        }
                    }
                }
            }
        }
        //println!("{:?}", event)
    }

    Ok(())
}
