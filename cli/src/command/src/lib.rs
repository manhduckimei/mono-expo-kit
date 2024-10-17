pub mod constants;
mod render_logo;
mod set_language;
mod set_navigation;
mod set_package_manager;
mod set_project_name;
pub mod themes;

use std::io;

pub use crate::set_language::check_language_arg;
pub use render_logo::render_logo;
pub use set_navigation::set_navigation;
pub use set_package_manager::set_package_manager;
pub use set_project_name::set_project_name;

pub use constants::*;

pub use console::{Style, Term};
pub use fs_extra::dir::create_all;
pub use handlebars::Handlebars;
pub use serde_json::{json, to_string_pretty};
pub use std::{env, fs};

pub fn clear_screen() -> io::Result<()> {
    Term::stdout().clear_screen()?;
    Term::stderr().clear_screen()
}

pub fn pretty_json(
    cli_results: CliResults,
    config_text: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let properties = to_string_pretty(&json!({
        "project_name": cli_results.project_name,
        "flags": cli_results.flags,
        "packages": cli_results.packages
    }))?;
    println!(
        "\nProject {} initialized successfully 🔥.\n{}\n{}\n",
        Style::new().cyan().apply_to(cli_results.project_name),
        config_text,
        properties
    );
    Ok(())
}
