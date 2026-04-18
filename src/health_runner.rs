//! Ties health checking into the handoff lifecycle.

use crate::health_config::HealthCheckSpec;
use crate::health::wait_healthy;
use crate::error::Result;

/// Run the configured health check, logging progress.
pub fn run_health_check(spec: &HealthCheckSpec) -> Result<()> {
    let (check, timeout) = spec.resolve()?;
    eprintln!(
        "[baton] running health check {:?} (timeout={}s)",
        check, spec.timeout_secs
    );
    wait_healthy(&check, timeout)?;
    eprintln!("[baton] health check passed");
    Ok(())
}

/// Run health check only if a spec is provided.
pub fn run_optional_health_check(spec: Option<&HealthCheckSpec>) -> Result<()> {
    match spec {
        Some(s) => run_health_check(s),
        None => {
            eprintln!("[baton] no health check configured, proceeding immediately");
            Ok(())
        }
    }
}
