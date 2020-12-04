use color_eyre::Result;
use futures_util::stream::StreamExt;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use strum::EnumIter;
use swayipc_async::{Connection, Event, EventType};

pub async fn get_workspaces(connection: &mut Connection) -> Result<()> {
    // request and print the i3 version
    println!("{:#?}", connection.get_workspaces().await?);
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
    while let Some(event) = events.next().await {
        if let Ok(ref event) = event {
            if let Event::Workspace(ref w) = event {
                println!("{:?}", w.current);
                /*
                TODO make this somehow work but I really dont know how as the midir crate is horrible
                let workspace = WorkspaceLaunchpadMapping::try_from(w.current.unwrap().num)?;
                reset_colors(&mut conn_out);
                conn_out.send(&[144, (workspace as i32).try_into().unwrap(), 28]).unwrap();*/
            }
        }
        //println!("{:?}", event)
    }

    Ok(())
}
