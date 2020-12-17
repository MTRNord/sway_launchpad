use crate::reset_colors;
use crate::utils::globals::{LAYER_NUMBER, PRESELECTED_LAYER_NUMBER, SHOWING_NUMBER};
use crate::utils::show_number;
use color_eyre::Result;
use midir::MidiOutput;
use std::sync::atomic::Ordering;
use swayipc_async::Connection;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

pub enum PluginActions<'a> {
    SwayWorkspace(&'a str),
    SwayBack,
    ExamplePlugin,
    SelectMatrixRoomByID(&'a str),
    RunMatrixPreset(&'a str),
    ShowNumber(usize),
    SelectLayer,
}

impl PluginActions<'_> {
    pub async fn run(&self) -> Result<()> {
        let midi_out = MidiOutput::new("My Test Output")?;
        let midi_out_ports = midi_out.ports();
        let out_port = midi_out_ports.get(1).unwrap();
        let mut conn_out = midi_out.connect(out_port, "midir-test").unwrap();
        match self {
            PluginActions::SwayWorkspace(workspace) => {
                let mut connection = Connection::new().await?;
                connection
                    .run_command(format!("workspace --no-auto-back-and-forth {}", workspace))
                    .await
                    .unwrap();
            }
            PluginActions::SwayBack => {
                let mut connection = Connection::new().await?;
                connection
                    .run_command("workspace back_and_forth")
                    .await
                    .unwrap();
            }
            PluginActions::ExamplePlugin => {
                let mut socket = UnixStream::connect("/run/lsc_example_plugin.sock").await?;

                socket.write(b"test\n").await?;
            }
            PluginActions::SelectMatrixRoomByID(room_id) => {
                let mut socket = UnixStream::connect("/run/lsc_matrix_presets.sock").await?;

                socket
                    .write(format!("select {}\n", room_id).as_bytes())
                    .await?;
            }
            PluginActions::RunMatrixPreset(preset_id) => {
                let mut socket = UnixStream::connect("/run/lsc_matrix_presets.sock").await?;

                socket
                    .write(format!("do {}\n", preset_id).as_bytes())
                    .await?;
            }
            PluginActions::ShowNumber(number) => {
                show_number(&mut conn_out, *number);
            }
            PluginActions::SelectLayer => {
                LAYER_NUMBER.store(
                    PRESELECTED_LAYER_NUMBER.load(Ordering::SeqCst),
                    Ordering::SeqCst,
                );
                SHOWING_NUMBER.store(false, Ordering::SeqCst);
                reset_colors(&mut conn_out);
            }
        }
        Ok(())
    }
}
