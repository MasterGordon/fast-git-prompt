use crate::prompt_parts::icon::{Icon, RenderableIcon};
use crate::prompt_parts::prompt_part::RenderablePromptPart;
use git2::Repository;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BranchSync {
    pub ahead: Icon,
    pub behind: Icon,
}

impl RenderablePromptPart for BranchSync {
    fn render(self, repo: &Repository) -> Option<String> {
        let head = repo.head().ok()?;
        let head_oid = head.target()?;
        let remote = repo
            .find_branch("origin/main", git2::BranchType::Remote)
            .ok()?;
        let remote_oid = remote.get().target()?;
        let (ahead, behind) = repo.graph_ahead_behind(head_oid, remote_oid).ok()?;

        let mut parts = vec![];
        if ahead > 0 {
            parts.push(format!("{} {}", self.ahead.render(), ahead));
        }
        if behind > 0 {
            parts.push(format!("{} {}", self.behind.render(), behind));
        }

        return Some(parts.join(" "));
    }
}
