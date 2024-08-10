use std::env;
use std::path::Path;
use std::process::exit;

use git2::Repository;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
mod prompt_parts;
use crate::prompt_parts::branch_name::BranchName;
use crate::prompt_parts::branch_status::BranchStatus;
use crate::prompt_parts::branch_sync::BranchSync;
use crate::prompt_parts::origin_icon::OriginIcon;
use crate::prompt_parts::prompt_part::RenderablePromptPart;
mod colors;
use crate::colors::{color, Color};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
enum PromptPart {
    #[serde(alias = "branchName")]
    BranchName(BranchName),
    #[serde(alias = "originIcon")]
    OriginIcon(OriginIcon),
    #[serde(alias = "branchStatus")]
    BranchStatus(BranchStatus),
    #[serde(alias = "branchSync")]
    BranchSync(BranchSync),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct Config {
    #[serde(rename = "$schema")]
    schema: Option<String>,
    #[serde(rename = "version-do-not-modify")]
    version: Option<String>,
    base_color: Option<Color>,
    prompt: Vec<PromptPart>,
}

fn get_config_dir() -> String {
    let config_dir_base = env::var("XDG_CONFIG_HOME").unwrap_or(format!(
        "{}/.config",
        match env::var("HOME") {
            Ok(home) => home,
            Err(_) => {
                println!("Could not find HOME env variable");
                exit(1);
            }
        }
    ));
    format!("{}/fast-git-prompt", config_dir_base)
}

fn write_schema() {
    let schema = schemars::schema_for!(Config);
    let mut file = std::fs::File::create(format!("{}/schema.json", get_config_dir())).unwrap();
    serde_json::to_writer_pretty(&mut file, &schema).unwrap();
}

fn read_config() -> Config {
    let json = std::fs::read_to_string(format!("{}/config.json", get_config_dir())).unwrap();
    serde_json::from_str(&json).unwrap()
}

fn write_config(config: Config) {
    let mut config_file =
        std::fs::File::create(format!("{}/config.json", get_config_dir())).unwrap();
    serde_json::to_writer_pretty(&mut config_file, &config).unwrap()
}

fn setup() {
    let config_dir = get_config_dir();
    if !Path::new(&config_dir).exists() {
        std::fs::create_dir_all(&config_dir).unwrap();
        write_schema();
        let config = Config {
            version: Some(VERSION.to_string()),
            schema: Some(format!("{}/schema.json", config_dir)),
            base_color: None,
            prompt: vec![],
        };
        write_config(config);
    } else {
        let mut config = read_config();
        if config.version != Some(VERSION.to_string()) {
            write_schema();
            config.version = Some(VERSION.to_string());
            write_config(config);
        }
    }
}

fn main() {
    setup();
    let mut prompt: Vec<String> = vec![];
    let config = read_config();

    // let repo = match Repository::open(".") {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(_) => {
            exit(0);
        }
    };
    for part in config.prompt {
        match part {
            PromptPart::BranchName(branch_name) => {
                let branch_name = branch_name.render(&repo);
                if branch_name.is_some() {
                    prompt.push(branch_name.unwrap());
                }
            }
            PromptPart::OriginIcon(origin_icon) => {
                let origin_icon = origin_icon.render(&repo);
                if origin_icon.is_some() {
                    prompt.push(origin_icon.unwrap());
                }
            }
            PromptPart::BranchStatus(branch_status) => {
                let branch_status = branch_status.render(&repo);
                if branch_status.is_some() {
                    prompt.push(branch_status.unwrap());
                }
            }
            PromptPart::BranchSync(branch_sync) => {
                let branch_sync = branch_sync.render(&repo);
                if branch_sync.is_some() {
                    prompt.push(branch_sync.unwrap());
                }
            }
        }
    }
    prompt = prompt.drain(..).filter(|s| !s.is_empty()).collect();

    print!(
        "{}{}",
        color(config.base_color),
        prompt.join(format!("{} ", color(config.base_color)).as_str())
    );
}
