use std::process;

use inquire::Select;

use crate::{themes::THEME, CliResults};

pub fn set_package_manager(cli_results: &mut CliResults) {
    println!();
    let theme = THEME.lock().unwrap();
    let options = ["npm", "yarn", "pnpm", "bun"]
        .into_iter()
        .collect::<Vec<_>>();

    match Select::new("Which package manager would you like to use?", options).prompt() {
        Ok(choice) if !choice.is_empty() => {
            cli_results.flags = Some(
                [("packageManager".to_string(), choice.to_string())]
                    .into_iter()
                    .collect(),
            );
        }
        Ok(_) => {}
        Err(e) if e.to_string() == "Operation was interrupted by the user" => {
            println!("{}", theme.format_confirm("Cancelled... "));
            process::exit(0);
        }
        Err(e) => println!("{}", theme.format_confirm(&e.to_string())),
    }
}
