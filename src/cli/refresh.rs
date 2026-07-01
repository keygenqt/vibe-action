//! Refresh command handler.
//! Manages refreshing actions cache and resetting context.

use clap::{Args, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum RefreshTarget {
    /// Reload actions from disk
    Actions,
    /// Reset context
    Context,
}

#[derive(Args)]
pub struct RefreshArgs {
    /// Refresh actions or context
    #[arg(long, short = 't')]
    target: RefreshTarget,
}

/// Execute the `refresh` command.
pub async fn execute(args: RefreshArgs) {
    match args.target {
        RefreshTarget::Actions => {
            println!("TODO: refresh actions — reload from disk");
        }
        RefreshTarget::Context => {
            println!("TODO: refresh context — reset context");
        }
    }
}
