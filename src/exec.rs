use anyhow::Result;
use std::process::{ExitStatus, Stdio};
use tokio::process::{Child, Command as TokioCommand};

#[cfg(unix)]
use nix::sys::signal::{Signal, kill as send_signal};
#[cfg(unix)]
use nix::unistd::Pid;
#[cfg(unix)]
use tokio::signal::unix::{SignalKind, signal};

/// Spawn a child process that inherits stdin/stdout/stderr, returning the handle.
pub fn spawn_command_interactive(cmd: &mut TokioCommand) -> Result<Child> {
    let child = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    Ok(child)
}

/// Run a child process in the background, returning the handle.
pub fn run_command_background(cmd: &mut TokioCommand) -> Result<Child> {
    let child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(child)
}

/// Wait for a child process, forwarding TERM and QUIT to it each time a signal
/// arrives, until the child exits. Ignores INT since it will already be sent to
/// the child automatically, and to prevent gcpsql from exiting before it.
#[cfg(unix)]
pub async fn wait_with_signal_forward(mut child: Child) -> Result<ExitStatus> {
    let mut sig_int = signal(SignalKind::interrupt())?;
    let mut sig_term = signal(SignalKind::terminate())?;
    let mut sig_quit = signal(SignalKind::quit())?;

    loop {
        tokio::select! {
            status = child.wait() => return Ok(status?),
            _ = sig_int.recv() => {
                // Ignore INT signals.
            },
            _ = sig_term.recv() => {
                forward_signal_to_child(&child, Signal::SIGTERM);
            },
            _ = sig_quit.recv() => {
                forward_signal_to_child(&child, Signal::SIGQUIT);
            },
        }
    }
}

/// Wait for a child process without signal forwarding on non-Unix platforms.
#[cfg(not(unix))]
pub async fn wait_with_signal_forward(mut child: Child) -> Result<ExitStatus> {
    Ok(child.wait().await?)
}

#[cfg(unix)]
fn forward_signal_to_child(child: &Child, sig: Signal) {
    if let Some(pid) = child.id() {
        let _ = send_signal(Pid::from_raw(pid as i32), sig);
    }
}
