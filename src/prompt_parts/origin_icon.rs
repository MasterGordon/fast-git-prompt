use std::collections::HashMap;

use crate::prompt_parts::icon::{Icon, RenderableIcon};
use crate::prompt_parts::prompt_part::RenderablePromptPart;
use git2::Repository;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct OriginIcon {
    icons: HashMap<String, Icon>,
    default_icon: Icon,
}

impl RenderablePromptPart for OriginIcon {
    fn render(self, repo: &Repository) -> Option<String> {
        let host_regex = Regex::new(r"^(.*@)?(?<host>[^:\/]*)").ok()?;
        let origin = repo.find_remote("origin").ok()?;
        let url = origin.url()?;
        let captures = host_regex.captures(url)?;
        let host = captures.name("host")?.as_str();
        let icon = match self.icons.get(host) {
            Some(i) => i,
            None => &self.default_icon,
        };
        return Some(icon.render());
    }
}
