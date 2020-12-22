use crate::plugin_actions::PluginActions;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Color {
    Red = 15,
    Green = 60,
    Orange = 47,
    Yellow = 62,
    Amber = 63,
}

pub static LAYERS: [&'static [(i64, Color, PluginActions)]; 10] = [
    &[
        (120, Color::Orange, PluginActions::SwayBack),
        (119, Color::Red, PluginActions::SwayWorkspace("1:\u{e007}")),
        (118, Color::Red, PluginActions::SwayWorkspace("2:\u{f086}")),
        (117, Color::Red, PluginActions::SwayWorkspace("3:\u{f0e0}")),
        (116, Color::Red, PluginActions::SwayWorkspace("5:\u{f1c9}")),
        (115, Color::Red, PluginActions::SwayWorkspace("11:\u{f1b6}")),
        (114, Color::Red, PluginActions::SwayWorkspace("12:\u{f11b}")),
        (113, Color::Red, PluginActions::SwayWorkspace("13:\u{f167}")),
        (112, Color::Red, PluginActions::SwayWorkspace("42:\u{f001}")),
        (102, Color::Orange, PluginActions::MumbleToggleMute),
        (103, Color::Orange, PluginActions::MumbleToggleDeaf),
        (109, Color::Amber, PluginActions::SpotifyPrevTrack), //TODO investigate why 109-111 do not light up (They are undocumented)
        (110, Color::Amber, PluginActions::SpotifyPause),
        (111, Color::Amber, PluginActions::SpotifyNextTrack),
    ],
    &[(0, Color::Red, PluginActions::ExamplePlugin)],
    &[
        (
            0,
            Color::Amber,
            PluginActions::SelectMatrixRoomByID("!IFqsVhqzXtALgXIFig:bpulse.org"),
        ),
        (
            1,
            Color::Amber,
            PluginActions::SelectMatrixRoomByID("!FJymnHKxIlQeNDkaZW:ubports.chat"),
        ),
        (17, Color::Yellow, PluginActions::RunMatrixPreset("0")),
    ],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
];
