use crate::prompt_parts::prompt_part::RenderablePromptPart;
use git2::Repository;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BranchName {
    pub color: Option<String>,
}

impl RenderablePromptPart for BranchName {
    fn render(self, repo: &Repository) -> Option<String> {
        let head = match repo.head() {
            Ok(r) => Some(r),
            Err(_) => None,
        }?;
        let name = head.name()?;
        let last = name.split('/').last()?;
        return Some(format!("{}{}", self.color.unwrap_or("".to_string()), last));
    }
}
