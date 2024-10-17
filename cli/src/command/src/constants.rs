pub(crate) type ObjectType = std::collections::HashMap<String, String>;

// Component data structure for Handlebars
#[derive(Serialize)]
pub struct CliResults {
    pub project_name: String,
    pub packages: Vec<Package>,
    pub flags: Option<ObjectType>,
}

use std::default::Default;

use serde::Serialize;

impl Default for CliResults {
    fn default() -> Self {
        CliResults {
            project_name: String::new(),
            packages: Vec::new(),
            flags: None,
        }
    }
}

pub const KIMEI: &str = r#"

██╗  ██╗██╗███╗   ███╗███████╗██╗                 
██║ ██╔╝██║████╗ ████║██╔════╝██║                 
█████╔╝ ██║██╔████╔██║█████╗  ██║                 
██╔═██╗ ██║██║╚██╔╝██║██╔══╝  ██║                 
██║  ██╗██║██║ ╚═╝ ██║███████╗██║                 
╚═╝  ╚═╝╚═╝╚═╝     ╚═╝╚══════╝╚═╝                 
 ██████╗ ██╗      ██████╗ ██████╗  █████╗ ██╗     
██╔════╝ ██║     ██╔═══██╗██╔══██╗██╔══██╗██║     
██║  ███╗██║     ██║   ██║██████╔╝███████║██║     
██║   ██║██║     ██║   ██║██╔══██╗██╔══██║██║     
╚██████╔╝███████╗╚██████╔╝██████╔╝██║  ██║███████╗
 ╚═════╝ ╚══════╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝
 
"#;

pub const PROJECT_NAME: &str = "my-expo-app";
#[derive(Serialize)]
pub struct Package {
    name: String,
    package_type: String,
    options: serde_json::Value,
}

impl Package {
    pub fn new(name: String, package_type: String, options: serde_json::Value) -> Self {
        Package {
            name,
            package_type,
            options,
        }
    }
}
