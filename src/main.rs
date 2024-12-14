#[macro_use]
extern crate lazy_static;

use clap::{Arg, Command as ClapCommand};
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

lazy_static! {
    static ref RUST_URLS: HashMap<std::string::String, std::string::String> = HashMap::from([
        ("rust_axum_svelte_sqlite".to_string(), "git@github.com:fuji-184/Rust-Svelte5-SSG-Embeded-Template.git".to_string()),
        ("go_fiber_svelte_sqlite".to_string(), "git@github.com:fuji-184/GoFiber-Svelte5-SSG-Embedded-Template.git".to_string()),
        ("go_inertia_svelte_sqlite".to_string(), "git@github.com:fuji-184/Go-Inertia-Svelte-Sqlite-SPA-SSR-Template.git".to_string()),
        ("go_chi_svelte_sqlite".to_string(), "git@github.com:fuji-184/GoChi-Svelte5-Embedded-Template.git".to_string())
    ]);
}

const RUST_FRAMEWORKS: [&str;1] = ["axum"];
const GO_FRAMEWORKS: [&str;3] = ["fiber", "chi", "go inertia"];

fn main() {
    let matches = ClapCommand::new("ufo")
        .version("0.1")
        .author("Fuji")
        .about("Scaffolding Fullstack Rust & JavaScript Easily")
        .subcommand(
            ClapCommand::new("create")
            .arg(
                Arg::new("project_name")
                .required(true)
                .help("The name of the project")
        ))
        .get_matches();

    let default = String::from("");
    let project_name = match matches.subcommand_matches("create") {
        Some(isi) => isi.get_one::<String>("project_name").unwrap(),
        None => {
            eprintln!("{}", String::from("Failed to read command"));
            &default
        }
    };

    if !project_name.is_empty() {
        create_project(project_name);
    }

}

fn create_project(project_name: &String) {
    let language_lists = vec!["rust", "go"];
    let language_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose backend language")
        .items(&language_lists)
        .default(0)
        .interact()
        .unwrap();

    let language = language_lists[language_choice];

    let backend = match language {
        "rust" => setup_backend(&RUST_FRAMEWORKS),
        "go" => setup_backend(&GO_FRAMEWORKS),
        _ => ""
    };

    let frontend_lists = vec!["svelte"];
    let frontend_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose frontend framework")
        .items(&frontend_lists)
        .default(0)
        .interact()
        .unwrap();

    let database_lists = vec!["sqlite", "postgresql", "mysql"];
    let database_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose database")
        .items(&database_lists)
        .default(0)
        .interact()
        .unwrap();

    let frontend = frontend_lists[frontend_choice];
    let database = database_lists[database_choice];

    println!("Creating projec with {} backend, {} frontend, and {} database. Please wait....", backend, frontend, database);

    let template_name = format!("{}_{}_{}_{}", language, backend, frontend, database).to_string();

    let template_url = RUST_URLS.get(&template_name).unwrap();

    setup_template(&template_url, &project_name);

    println!("Project is created successfully! For running the backend development server use cargo run or cargo watch. Then run the frontend development server independently by using npm run dev");
}

fn setup_template(template_url: &str, project_name: &str){
    let mut template_process = Command::new("git")
        .args(["clone", &template_url, &project_name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(stdout) = template_process.stdout.take() {
        let reader = BufReader::new(stdout);
        for _ in reader.lines() {
            println!("The project is being created....");
        }
    }

    template_process.wait().unwrap();

    Command::new("sh")
        .arg("-c")
        .arg(format!("cd {} && rm -rf .git && cd frontend && npm install && npm run build", project_name))
        .status()
        .unwrap();
}

fn setup_backend <'lifetime_1> (backend_lists: &'lifetime_1 [&'lifetime_1 str]) -> &'lifetime_1 str {
     let backend_choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose backend framework")
        .items(backend_lists)
        .default(0)
        .interact()
        .unwrap();

    backend_lists[backend_choice]
}
