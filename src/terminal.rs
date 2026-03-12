use russh::server::Handle;
use russh::ChannelId;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

/// Bridge between ratatui's crossterm backend and the SSH channel.
///
/// Implements `std::io::Write` so that crossterm can write ANSI escape
/// sequences into a buffer, which is then flushed over the SSH channel
/// via a tokio mpsc channel.
pub struct TerminalHandle {
    sender: UnboundedSender<Vec<u8>>,
    sink: Vec<u8>,
}

impl TerminalHandle {
    /// Spawn a background task that forwards buffered writes to the SSH channel.
    pub async fn start(handle: Handle, channel_id: ChannelId) -> Self {
        let (sender, mut receiver) = unbounded_channel::<Vec<u8>>();

        tokio::spawn(async move {
            while let Some(data) = receiver.recv().await {
                if let Err(e) = handle.data(channel_id, data.into()).await {
                    log::error!("Failed to send data to SSH channel: {e:?}");
                    break;
                }
            }
        });

        Self {
            sender,
            sink: Vec::new(),
        }
    }
}

impl std::io::Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.sender.send(self.sink.clone()).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::BrokenPipe, e)
        })?;
        self.sink.clear();
        Ok(())
    }
}
