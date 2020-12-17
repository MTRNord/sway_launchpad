use crate::plugin_actions::PluginActions;
use std::convert::TryFrom;
use strum::EnumIter;

// TODO make array for layers
#[non_exhaustive]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum LaunchpadMapping {
    Back = 120,
    Firefox = 119,
    Chats = 118,
    Email = 117,
    Coding = 116,
    Steam = 115,
    Games = 114,
    Youtube = 113,
    Spotify = 112,
    Test = 0,
    FirefoxWebdriver = 1,
}

impl Into<PluginActions<'_>> for LaunchpadMapping {
    fn into(self) -> PluginActions<'static> {
        match self {
            LaunchpadMapping::Back => PluginActions::SwayBack,
            LaunchpadMapping::Firefox => PluginActions::SwayWorkspace("1:\u{e007}"),
            LaunchpadMapping::Chats => PluginActions::SwayWorkspace("2:\u{f086}"),
            LaunchpadMapping::Email => PluginActions::SwayWorkspace("3:\u{f0e0}"),
            LaunchpadMapping::Coding => PluginActions::SwayWorkspace("5:\u{f1c9}"),
            LaunchpadMapping::Steam => PluginActions::SwayWorkspace("11:\u{f1b6}"),
            LaunchpadMapping::Games => PluginActions::SwayWorkspace("12:\u{f11b}"),
            LaunchpadMapping::Youtube => PluginActions::SwayWorkspace("13:\u{f167}"),
            LaunchpadMapping::Spotify => PluginActions::SwayWorkspace("42:\u{f001}"),
            LaunchpadMapping::Test => PluginActions::ExamplePlugin,
            LaunchpadMapping::FirefoxWebdriver => PluginActions::SelectMatrixRoomByID(""),
        }
    }
}

impl TryFrom<i32> for LaunchpadMapping {
    type Error = ();

    fn try_from(input: i32) -> Result<LaunchpadMapping, Self::Error> {
        match input {
            120 => Ok(LaunchpadMapping::Back),
            119 => Ok(LaunchpadMapping::Firefox),
            118 => Ok(LaunchpadMapping::Chats),
            117 => Ok(LaunchpadMapping::Email),
            116 => Ok(LaunchpadMapping::Coding),
            115 => Ok(LaunchpadMapping::Steam),
            114 => Ok(LaunchpadMapping::Games),
            113 => Ok(LaunchpadMapping::Youtube),
            112 => Ok(LaunchpadMapping::Spotify),
            0 => Ok(LaunchpadMapping::Test),
            1 => Ok(LaunchpadMapping::FirefoxWebdriver),
            _ => Err(()),
        }
    }
}
