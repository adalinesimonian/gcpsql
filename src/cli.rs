use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "gcpsql",
    about = "Run psql against a GCP Cloud SQL instance",
    long_about = "
A command-line tool to connect to a Google Cloud SQL instance using the
Cloud SQL Proxy and psql. It starts the proxy, connects to the database,
and cleans up after the session ends.

You need to have the 'cloud_sql_proxy' and 'psql' command-line tools installed
and accessible in your PATH.",
    version = env!("CARGO_PKG_VERSION")
)]
pub struct Cli {
    #[arg(help = "Instance identifier in the format <PROJECT>:<REGION>:<INSTANCE>")]
    pub instance: String,

    #[arg(help = "Database name to connect to")]
    pub database: Option<String>,

    #[arg(
        short,
        long,
        help = "Username for the database connection",
        env = "PGUSER"
    )]
    pub user: Option<String>,

    #[arg(
        short,
        long,
        help = "Password for the database connection",
        env = "PGPASSWORD"
    )]
    pub password: Option<String>,

    #[arg(
        short,
        long,
        help = "If set, enables verbose output",
        action = ArgAction::SetTrue
    )]
    pub verbose: bool,
}
