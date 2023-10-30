use clap::Parser;
use serde::Serialize;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Debug)]
struct Config {
    project: String,
    author: String,
    email: String,
    version: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(disable_version_flag = true)]
struct Args {
    /// Name of the project
    #[clap(short, long, default_value = "default_project")]
    name: String,

    /// Author of the project
    #[clap(short, long, default_value = "Default Author")]
    author: String,

    /// Email of the author
    #[clap(short, long, default_value = "author@example.com")]
    email: String,

    /// Version of the project
    #[clap(short, long, default_value = "0.1.0", name = "proj_version")]
    version: String,
}

fn main() {
    let mut args = Args::parse();

    // Check for default values and launch interactive mode if needed
    if args.name == "default_project" || args.author == "Default Author" || args.email == "author@example.com" || args.version == "0.1.0" {
        println!("Some fields have default values. Would you like to customize them? (yes/no)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() == "yes" {
            if args.name == "default_project" {
                args.name = prompt("Enter project name:");
            }
            if args.author == "Default Author" {
                args.author = prompt("Enter author name:");
            }
            if args.email == "author@example.com" {
                args.email = prompt("Enter author email:");
            }
            if args.version == "0.1.0" {
                args.version = prompt("Enter project version:");
            }
        }
    }

    // Define the path where the Grit project will be initialized
    let project_path = Path::new(".");

    // Initialize the Grit project
    match init_grit_project(project_path, &args) {
        Ok(_) => println!("Grit project initialized successfully!"),
        Err(e) => eprintln!("Error initializing Grit project: {}", e),
    }
}

fn init_grit_project(path: &Path, args: &Args) -> Result<(), std::io::Error> {
    // Create the .grit directory
    let grit_dir = path.join(".grit");
    fs::create_dir_all(&grit_dir)?;

    // Generate the configuration
    let config = Config {
        project: args.name.clone(),
        author: args.author.clone(),
        email: args.email.clone(),
        version: args.version.clone(),
    };

    // Serialize the configuration to TOML format
    let config_toml = toml::to_string_pretty(&config).unwrap();

    // Write the configuration to the config.toml file
    let config_path = grit_dir.join("config.toml");
    let mut config_file = File::create(config_path)?;
    write!(config_file, "{}", config_toml)?;

    Ok(())
}

fn prompt(message: &str) -> String {
    print!("{} ", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
