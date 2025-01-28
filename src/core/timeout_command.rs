use std::process::Output;
use std::time::Duration;
use thiserror::Error;
use tokio::process::Command;
use tokio::sync::oneshot;
use tokio::time::timeout;

#[derive(Debug, Error)]
enum TimeoutCommandError {
    #[error("Sender was dropped")]
    SenderDropped,
    #[error("Timeout exceeded")]
    TimeoutExceeded,
    #[error("Failed to get output from process")]
    OutputError,
}

struct TimeoutCommand {
    command: Command,
    timeout: Duration,
}

impl TimeoutCommand {
    fn new(command: Command, timeout: Duration) -> Self {
        Self { command, timeout }
    }

    async fn output(mut self) -> Result<Output, TimeoutCommandError> {
        let (tx, rx) = oneshot::channel();

        // TODO: actually kill the process when the duration elapsed
        // will probably have to use two tasks instead of timeout
        tokio::spawn(async move {
            let res = self.command.output().await;
            let _ = tx.send(res);
        });

        timeout(self.timeout, rx)
            .await
            .map_err(|_| TimeoutCommandError::TimeoutExceeded)?
            .map_err(|_| TimeoutCommandError::SenderDropped)?
            .map_err(|_| TimeoutCommandError::OutputError)
    }
}
