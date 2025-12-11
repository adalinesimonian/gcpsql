use crate::exec::run_command_background;
use crate::net::{find_free_port, is_address_responsive_within_timeout_async};
use anyhow::{Result, anyhow};
use tokio::process::{Child, Command};

/// Represents a GCP SQL Proxy instance.
pub struct GcpSqlProxy {
    pub port: u16,
    pub child: Child,
}

/// Start the gcloud sql proxy for the given instance identifier.
pub async fn start_gcp_sql_proxy(instance: &str) -> Result<GcpSqlProxy> {
    let port = find_free_port().ok_or_else(|| anyhow!("No free port found"))?;

    let mut cmd = Command::new("cloud_sql_proxy");
    cmd.arg(format!("--instances={}=tcp:{}", instance, port));

    let child = run_command_background(&mut cmd)?;

    // See if the proxy is responsive.
    if !is_address_responsive_within_timeout_async("127.0.0.1", port, 5).await {
        return Err(anyhow!("GCP SQL Proxy did not become responsive in time"));
    }

    Ok(GcpSqlProxy { port, child })
}
