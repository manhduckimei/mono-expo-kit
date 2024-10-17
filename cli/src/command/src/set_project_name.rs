use inquire::Text;
use std::process;

use crate::constants::{CliResults, PROJECT_NAME};
use crate::themes::THEME;

pub fn set_project_name(cli_results: &mut CliResults) {
    let theme = THEME.lock().unwrap();

    let name: Result<String, inquire::InquireError> =
        Text::new("What do you want to name your project?")
            .with_default(PROJECT_NAME)
            .with_autocomplete(move |input: &str| {
                if input.is_empty() {
                    Ok(vec![])
                } else {
                    let variants = vec!["mobile", "app", "frontend"];
                    variants
                        .iter()
                        .map(|variant| Ok(format!("{}-{}", input, variant)))
                        .collect()
                }
            })
            .prompt();

    match name {
        Ok(project_name) => {
            cli_results.project_name = project_name
                .trim()
                .to_string()
                .is_empty()
                .then(|| PROJECT_NAME.to_string())
                .unwrap_or(project_name);
        }
        Err(e) => {
            if e.to_string() == "Operation was interrupted by the user" {
                println!("{}", theme.format_confirm("Cancelled... 👋"));
            } else {
                println!("{}", theme.format_confirm(&e.to_string()));
            }
            process::exit(0);
        }
    }
}
