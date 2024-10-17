use crate::themes::THEME;
use inquire::Confirm;

fn set_language() -> Result<bool, String> {
    println!();
    let use_typescript = Confirm::new("Would you like to use TypeScript? (Default: Yes)")
        .with_default(true)
        .prompt()
        .map_err(|_| "Error! Please try again later.")?;

    // Lock the theme only for the output, reducing the duration of the lock
    let theme_message = if use_typescript {
        "Cool! Your project is now using TypeScript."
    } else {
        "No problem! Your project is now using JavaScript."
    };

    let theme = THEME.lock().map_err(|_| "Failed to lock theme")?;
    println!("{}", theme.format_confirm(theme_message));

    Ok(use_typescript)
}

// Example of checking a command-line argument for language preference
pub fn check_language_arg() -> Result<bool, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "typescript" => Ok(true),
            "javascript" => Ok(false),
            _ => Err("Invalid argument. Use 'typescript' or 'javascript'.".to_string()),
        }
    } else {
        // Default behavior can still be interactive
        set_language()
    }
}
