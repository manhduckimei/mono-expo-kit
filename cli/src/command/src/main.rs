use expo_kit_cli::{themes::THEME, *};
fn main() -> Result<(), Box<dyn std::error::Error>> {
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
