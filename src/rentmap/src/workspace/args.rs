use std::path::PathBuf;

use clap::Args;

use super::{Workspace, WorkspaceError};

#[derive(Debug, Args)]
pub struct WorkspaceArgs {
    /// The root directory of the workspace
    #[arg(long, short, default_value = ".rentmap")]
    pub workspace: PathBuf,
}

impl WorkspaceArgs {
    pub async fn build(self) -> Result<Workspace, WorkspaceError> {
        let workspace = Workspace::new(self.workspace);
        workspace.init().await?;
        Ok(workspace)
    }
}
