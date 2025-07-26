use crate::context::AppContext;

pub fn handle(ctx: &AppContext, name: &str) {
    println!("Verbose: {}", ctx.verbose);
    println!("Creating project: {name}");
    // TODO: Implement actual project creation logic
}
