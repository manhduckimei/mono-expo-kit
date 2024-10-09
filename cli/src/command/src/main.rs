use fs_extra::dir::create_all;
use handlebars::Handlebars;
use std::env;
use std::fs;
use std::io::{self, Write};

mod constants;
use constants::{ComponentData, COMPONENT_TEMPLATE};

fn main() {
    // Get project directory
    let current_dir = env::current_dir().expect("Unable to get current directory");
    let project_name = get_user_input("Enter the project name: ");
    let component_name = get_user_input("Enter the component name: ");
    println!("current_dir: {}", current_dir.display());
    // Create project directory
    let project_dir = current_dir.join(&project_name);
    create_all(&project_dir, true).expect("Failed to create project directory");

    // Create src/components directory
    let components_dir = project_dir.join("src/components");
    create_all(&components_dir, true).expect("Failed to create components directory");

    // // Load the component.hbs template
    // let template_content =
    //     fs::read_to_string("../template/base/component.hbs").expect("Failed to read template file");

    // Initialize handlebars
    let mut handlebars = Handlebars::new();

    // Register the template
    handlebars
        .register_template_string("component", COMPONENT_TEMPLATE)
        .expect("Failed to register template");

    // Create data for the template
    let data = ComponentData {
        name: component_name.clone(),
    };

    // Render the component template
    let rendered = handlebars
        .render("component", &data)
        .expect("Failed to render template");

    // Write the generated component to the project directory
    let component_file_path = components_dir.join(format!("{}.tsx", component_name));
    fs::write(component_file_path, rendered).expect("Failed to write component file");

    println!(
        "React Native project '{}' initialized successfully.",
        project_name
    );
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
