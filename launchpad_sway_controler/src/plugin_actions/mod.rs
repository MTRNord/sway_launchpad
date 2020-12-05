use color_eyre::Result;
use swayipc_async::Connection;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

pub enum PluginActions<'a> {
    SwayWorkspace(&'a str),
    ExamplePlugin,
}

impl PluginActions<'_> {
    pub async fn run(&self) -> Result<()> {
        match self {
            PluginActions::SwayWorkspace(workspace) => {
                let mut connection = Connection::new().await?;
                connection
                    .run_command(format!("workspace --no-auto-back-and-forth {}", workspace))
                    .await
                    .unwrap();
            }
            PluginActions::ExamplePlugin => {
                let mut socket = UnixStream::connect("/run/lsc_example_plugin.sock").await?;

                socket.write(b"test\n").await?;
            }
        }
        Ok(())
    }
}
