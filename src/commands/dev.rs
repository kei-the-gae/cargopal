use tracing::{debug, info};

use crate::context::AppContext;

pub fn handle(ctx: &AppContext) -> anyhow::Result<()> {
    debug!(?ctx, "Starting dev server...");
    info!("Starting dev server...");
    // TODO: Implement actual dev server logic
    Ok(())
}
