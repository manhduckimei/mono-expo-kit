use crate::themes::THEME;
use crate::CliResults;
use crate::Package;
use inquire::{InquireError, Select};
use serde_json::json;
use std::process;

pub fn set_navigation(cli_results: &mut CliResults) {
    println!();

    let theme = THEME.lock().unwrap();
    let options = vec!["React Navigation", "Expo Router", "None"];
    let navigation_type_options = vec!["Stack + Tabs", "Drawer + Tabs", "Tabs", "Includes All"];

    let ans: Result<&str, InquireError> =
        Select::new("What would you like to use for Navigation?", options).prompt();
    match ans {
        Ok(choice) if !choice.is_empty() && choice != "None" => {
            println!();
            let navigation_type = Select::new(
                "What type of navigation would you like to use?",
                navigation_type_options,
            )
            .prompt();
            match navigation_type {
                Ok(choice_navigation_type) if !choice_navigation_type.is_empty() => {
                    let package = Package::new(
                        choice.to_string(),
                        "navigation".to_string(),
                        json!({
                            "type": choice_navigation_type.to_string().to_lowercase()
                        }),
                    );

                    cli_results.packages.push(package);
                }
                _ => (),
            }
            println!(
                "{}",
                theme.format_confirm(&format!("Great, we'll use {} 🎉", choice))
            );
        }
        Ok(_) => {}
        Err(e) if e.to_string() == "Operation was interrupted by the user" => {
            println!("{}", theme.format_confirm("Cancelled... 👋"));
            process::exit(0)
        }
        Err(e) => println!("{}", theme.format_confirm(&e.to_string())),
    }
}
