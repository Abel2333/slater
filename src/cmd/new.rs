use clap::Args;

use crate::error::Result;

#[derive(Debug, Clone, Args)]
pub struct NewOptions {
    pub title: Option<String>,
}

pub fn execute(options: NewOptions) -> Result<()> {
    let title = options.title.unwrap_or_else(|| "untitled-post".to_string());
    println!("new command is not implemented yet: requested post `{title}`");
    Ok(())
}
