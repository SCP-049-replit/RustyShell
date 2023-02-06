// Imports
use inquire::list_option::ListOption;
use crate::terminal::c::*;
use colored::Colorize;
use inquire::*;
use std::fs;
mod c;

struct Config {
    autocomplete: bool,
}

// Runs the terminal
pub fn run() {
    loop {
        let choices = Config {
            autocomplete: false,
        };

        let username = fs::read_to_string("./src/name.txt").expect("err: Unable to read name.txt");
        let username = username + ":";

        if choices.autocomplete == true {
            let cmd = Text::new(&username)
                .with_autocomplete(&suggestions)
                .prompt();
            match cmd {
                Ok(cmd) => exec(cmd),
                Err(_) => println!("err: Unable to run command"),
            }
        } else {
            let cmd = Text::new(&username)
                .prompt();
            match cmd {
                Ok(cmd) => exec(cmd),
                Err(_) => println!("err: Unable to run command"),
            }
        }
    }
}

// Autocomplete suggestions
fn suggestions(val: &str) -> Result<Vec<String>, CustomUserError> {
    let options = ["shell.help", "shell.config", "shell.name", "shell.refresh", "shell.quit"];

    let val_lower = val.to_lowercase();

    Ok(options
        .iter()
        .filter(|s| s.to_lowercase().contains(&val_lower))
        .map(|s| String::from(*s))
        .collect())
}

// Runs the command
fn exec(cmd: String) {
    if cmd == "shell.help" {
        shell_help();
    } else if cmd == "shell.config" {
        shell_config(); 
    } else if cmd == "shell.name" {
        shell_name();
    } else if cmd == "shell.refresh" {
        shell_refresh();
    } else if cmd == "shell.quit" {
        shell_quit();
    } else {
        println!("");
        println!("{}", format!("err: Unknown command, `{cmd}`").red().bold());
    }
    println!("");
}

// Edit shell settings; shell.config
fn shell_config(){
    let options = vec![
        "Autocomplete",
    ];
        let edits = MultiSelect::new("Terminal Configuration settings:", options).raw_prompt();

    
}