use crate::{context::AppContext, utils::create_project};

pub fn handle(ctx: &AppContext, template: &str, name: &str) {
    if ctx.verbose {
        println!("Creating project '{name}' from template '{template}'");
    }
    create_project(template, name).expect("Failed to create project");
}
