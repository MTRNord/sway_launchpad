use crate::plugin_actions::PluginActions;

pub static LAYERS: [&'static [(i64, PluginActions)]; 10] = [
    &[
        (120, PluginActions::SwayBack),
        (119, PluginActions::SwayWorkspace("1:\u{e007}")),
        (118, PluginActions::SwayWorkspace("2:\u{f086}")),
        (117, PluginActions::SwayWorkspace("3:\u{f0e0}")),
        (116, PluginActions::SwayWorkspace("5:\u{f1c9}")),
        (115, PluginActions::SwayWorkspace("11:\u{f1b6}")),
        (114, PluginActions::SwayWorkspace("12:\u{f11b}")),
        (113, PluginActions::SwayWorkspace("13:\u{f167}")),
        (112, PluginActions::SwayWorkspace("42:\u{f001}")),
        (102, PluginActions::MumbleToggleMute),
        (103, PluginActions::MumbleToggleDeaf),
        (109, PluginActions::SpotifyPrevTrack), //TODO investigate why 109-111 do not light up (They are undocumented)
        (110, PluginActions::SpotifyPause),
        (111, PluginActions::SpotifyNextTrack),
    ],
    &[(0, PluginActions::ExamplePlugin)],
    &[
        (
            0,
            PluginActions::SelectMatrixRoomByID("!IFqsVhqzXtALgXIFig:bpulse.org"),
        ),
        (
            1,
            PluginActions::SelectMatrixRoomByID("!FJymnHKxIlQeNDkaZW:ubports.chat"),
        ),
        (17, PluginActions::RunMatrixPreset("0")),
    ],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
    &[],
];
