use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

const ESC: &str = "\x1b";

pub trait Ansi {
    fn to_ansi_code(&self) -> String;
}

pub fn color(color: Option<Color>) -> String {
    if let Some(color) = color {
        return color.to_ansi_code();
    }
    return "".to_string();
}

impl Ansi for Color {
    fn to_ansi_code(&self) -> String {
        match self {
            Color::Black => format!("{}[30m", ESC),
            Color::Red => format!("{}[31m", ESC),
            Color::Green => format!("{}[32m", ESC),
            Color::Yellow => format!("{}[33m", ESC),
            Color::Blue => format!("{}[34m", ESC),
            Color::Magenta => format!("{}[35m", ESC),
            Color::Cyan => format!("{}[36m", ESC),
            Color::White => format!("{}[37m", ESC),
            Color::BrightBlack => format!("{}[90m", ESC),
            Color::BrightRed => format!("{}[91m", ESC),
            Color::BrightGreen => format!("{}[92m", ESC),
            Color::BrightYellow => format!("{}[93m", ESC),
            Color::BrightBlue => format!("{}[94m", ESC),
            Color::BrightMagenta => format!("{}[95m", ESC),
            Color::BrightCyan => format!("{}[96m", ESC),
            Color::BrightWhite => format!("{}[97m", ESC),
        }
    }
}
