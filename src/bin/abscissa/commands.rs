//! Abscissa CLI Subcommands

mod new;
mod version;

use self::{new::NewCommand, version::VersionCommand};
use super::config::CliConfig;
use abscissa::{Callable, Command, Configurable, Options};
use std::path::PathBuf;

/// Abscissa CLI Subcommands
#[derive(Callable, Command, Debug, Options)]
pub enum CliCommand {
    #[options(help = "create a new Abscissa application from a template")]
    New(NewCommand),

    #[options(help = "display version information")]
    Version(VersionCommand),
}

impl Configurable<CliConfig> for CliCommand {
    fn config_path(&self) -> Option<PathBuf> {
        None
    }
}