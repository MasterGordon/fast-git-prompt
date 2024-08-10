use crate::prompt_parts::icon::{Icon, RenderableIcon};
use crate::prompt_parts::prompt_part::RenderablePromptPart;
use git2::{Repository, Status};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BranchStatus {
    pub dirty: Icon,
    pub clean: Icon,
    pub deleted: Icon,
    pub changed: Icon,
    pub new: Icon,
}

impl RenderablePromptPart for BranchStatus {
    fn render(self, repo: &Repository) -> Option<String> {
        let state = repo.state();
        let changes = repo.statuses(None).ok()?;
        let has_new = changes
            .iter()
            .any(|status| status.status() == Status::WT_NEW);
        let has_changed = changes.iter().any(|status| {
            status.status() == Status::INDEX_MODIFIED
                || status.status() == Status::WT_MODIFIED
                || status.status() == Status::INDEX_RENAMED
                || status.status() == Status::WT_RENAMED
                || status.status() == Status::WT_NEW
        });
        let has_deleted = changes.iter().any(|status| {
            status.status() == Status::INDEX_DELETED || status.status() == Status::WT_DELETED
        });
        let is_clean =
            state == git2::RepositoryState::Clean && !has_changed && !has_deleted && !has_new;

        let mut parts = vec![];
        if is_clean {
            parts.push(self.clean.render());
        } else {
            parts.push(self.dirty.render());
        }
        if has_changed {
            parts.push(self.changed.render());
        }
        if has_new {
            parts.push(self.new.render());
        }
        if has_deleted {
            parts.push(self.deleted.render());
        }
        return Some(parts.join(" "));
    }
}
