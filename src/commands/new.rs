use anyhow::Result;
use tracing::{debug, info};

use crate::{context::AppContext, utils::create_project};

pub fn handle(ctx: &AppContext, template: &str, name: &str) -> Result<()> {
    debug!(?ctx, "Creating new project");
    info!("Scaffolding project `{name}` from `{template}` template...");
    create_project(template, name)
}
