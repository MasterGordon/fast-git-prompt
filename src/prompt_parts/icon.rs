use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::colors::{color, Color};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Icon {
    pub color: Option<Color>,
    pub icon: String,
}

pub trait RenderableIcon {
    fn render(&self) -> String;
}

impl RenderableIcon for Icon {
    fn render(&self) -> String {
        return format!("{}{}", color(self.color), self.icon);
    }
}
