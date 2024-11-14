//!
//! [![release](https://img.shields.io/github/v/release/manhduckimei/mono-expo-kit?display_name=tag&include_prereleases)](https://github.com/manhduckimei/mono-expo-kit/releases/latest)
//! [![npm](https://img.shields.io/npm/v/create-expo-kit)](https://www.npmjs.com/package/create-expo-kit)
//! [![crates.io](https://img.shields.io/crates/v/orogene.svg)](https://crates.io/crates/create-expo-kit)
//! [![CI](https://img.shields.io/github/checks-status/manhduckimei/mono-expo-kit/main)](https://github.com/manhduckimei/mono-expo-kit/actions/workflows/ci.yml?query=branch%3Amain)
//! [![Project
//! Roadmap](https://img.shields.io/badge/Roadmap-create--expo--kit--v1.0-informational)](https://github.com/users/manhduckimei/projects/1/views/1)

//!
//! Check [the roadmap](https://github.com/orgs/orogene/projects/2)
//!
//! ## Getting Started
//!
//! You can install Create Expo Kit in various ways:
//!
//! NPM:
//! ```sh
//! $ npm install -g create-expo-kit
//! ```
//!
//! Cargo:
//! ```sh
//! $ cargo install create-expo-kit
//! ```
//!
//! Homebrew:
//! ```sh
//! $ brew install create-expo-kit
//! ```
//!
//! ## Usage
//!
//! For usage documentation
//!
//! ## Contributing
//!
//! ## License
//!

pub mod constants;
mod render_logo;
mod set_language;
mod set_navigation;
mod set_package_manager;
mod set_project_name;
pub mod themes;

use crate::themes::THEME;
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

pub struct CreateExpoKit;

impl CreateExpoKit {
    pub fn load() -> Result<(), Box<dyn std::error::Error>> {
        clear_screen()?;
        let current_dir = env::current_dir()?;

        let mut cli_results = CliResults::default();

        let config_text = THEME
            .lock()
            .unwrap()
            .format_confirm("Your project configuration\n");

        render_logo();

        set_project_name(&mut cli_results);

        let _ = check_language_arg();

        set_package_manager(&mut cli_results);

        set_navigation(&mut cli_results);

        let project_dir = current_dir.join(cli_results.project_name.clone());

        create_all(&project_dir, true)?;

        pretty_json(cli_results, config_text)?;

        Ok(())
    }
}
