use console::Style;
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::constants::KIMEI;

pub trait Theme {
    fn logo_color(&self) -> String {
        format!("{}", Style::new().cyan().apply_to(KIMEI))
    }
    fn format_confirm(&self, title: &str) -> String {
        let inactive_style: &Style = &self.placeholder_style();
        let divider = inactive_style.apply_to("| ").to_string();
        if title.contains("Error") || title.contains("Cancelled") {
            return format!("\n{divider}{}", Style::new().red().apply_to(title));
        } else {
            return format!("\n{divider}{}", Style::new().green().apply_to(title));
        }
    }
    fn placeholder_style(&self) -> Style {
        Style::new().dim()
    }
}

struct ExpoKitTheme;

impl Theme for ExpoKitTheme {}

pub static THEME: Lazy<Mutex<Box<dyn Theme + Send + Sync>>> =
    Lazy::new(|| Mutex::new(Box::new(ExpoKitTheme)));
