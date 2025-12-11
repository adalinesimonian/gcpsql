use crate::exec::{spawn_command_interactive, wait_with_signal_forward};
use anyhow::Result;
use std::process::ExitStatus;

/// Constructs a Postgres URL for the given port and, optionally, database.
pub fn create_postgres_url(port: u16, database: Option<&str>) -> String {
    let db_part = database.unwrap_or("postgres");
    format!("postgresql://localhost:{}/{}", port, db_part)
}

/// Runs psql against the given Postgres URL, handling signals until it exits.
pub async fn run_psql(
    url: &str,
    username: Option<&str>,
    password: Option<&str>,
) -> Result<ExitStatus> {
    let mut cmd = tokio::process::Command::new("psql");
    cmd.env("PGPASSWORD", password.unwrap_or_default());
    cmd.env("PGUSER", username.unwrap_or_default());
    cmd.arg(url);
    let child = spawn_command_interactive(&mut cmd)?;
    wait_with_signal_forward(child).await
}
