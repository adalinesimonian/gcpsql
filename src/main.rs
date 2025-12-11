mod cli;
mod exec;
mod gcp_sql_proxy;
mod net;
mod psql;

use clap::Parser;
use cli::Cli;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file if present.
    dotenv().ok();

    let cli = Cli::parse();

    let verbose = cli.verbose;

    // Start the GCP SQL Proxy.

    if verbose {
        eprintln!("Starting GCP SQL Proxy for instance: {}", cli.instance);
    }
    let mut proxy = gcp_sql_proxy::start_gcp_sql_proxy(&cli.instance).await?;

    if verbose {
        eprintln!("Started GCP SQL Proxy on port: {}", proxy.port);
    }

    let database = cli.database.as_deref();
    let username = cli.user.as_deref();
    let password = cli.password.as_deref();

    // Construct the Postgres URL.
    let url = psql::create_postgres_url(proxy.port, database);

    // Run psql against the Postgres URL.
    if verbose {
        eprintln!("Connecting to database with psql at URL: {}", url);
    }
    let psql_status = psql::run_psql(&url, username, password).await?;

    if verbose {
        eprintln!("psql session ended, shutting down GCP SQL Proxy");
    }

    // Clean up the proxy process after psql exits (or is terminated).
    if proxy.child.try_wait()?.is_none() {
        let _ = proxy.child.kill().await;
    }
    proxy.child.wait().await?;

    if !psql_status.success() {
        anyhow::bail!("psql exited with status: {}", psql_status);
    }

    Ok(())
}
