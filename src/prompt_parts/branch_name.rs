use crate::colors::{color, Color};
use crate::prompt_parts::prompt_part::RenderablePromptPart;
use git2::Repository;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BranchName {
    pub color: Option<Color>,
}

impl RenderablePromptPart for BranchName {
    fn render(self, repo: &Repository) -> Option<String> {
        let head = match repo.head() {
            Ok(r) => Some(r),
            Err(_) => None,
        }?;
        let name = head.name()?;
        let last = name.split('/').last()?;
        let head_commit = head.peel_to_commit().unwrap();
        let mut current_tags: Vec<String> = vec![];
        let tag_names = repo.tag_names(None);
        for tag_name in tag_names.iter().flatten() {
            let tag_object = repo.revparse_single(&format!("refs/tags/{}", tag_name?));

            if let Ok(tag_commit) = tag_object.unwrap().peel_to_commit() {
                if head_commit.id() == tag_commit.id() {
                    if let Some(tag) = tag_name {
                        current_tags.push(tag.to_string());
                    }
                }
            }
        }
        let last_tag = current_tags.last();
        if let Some(tag) = last_tag {
            return Some(format!("{}{} ({})", color(self.color), last, tag));
        } else {
            return Some(format!("{}{}", color(self.color), last));
        }
    }
}
