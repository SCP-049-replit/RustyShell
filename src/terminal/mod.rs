// Imports
use crate::terminal::c::*;
use colored::Colorize;
use inquire::*;
use std::fs;
mod c;

struct Config {
    autocomplete: bool,
}

pub fn print_message() { 
    clearscreen::clear().expect("failed to clear screen");
    println!("{}", "Welcome to the RustyShell!".green());
    println!("{}", "Type `shell.help` to see commands".green());
    println!("");
}

// Runs the terminal
pub fn run() {
    loop {
        let settings = Config {
            autocomplete: false,
        };

        let username: String =
            fs::read_to_string("./src/name.txt").expect(&format!("err: Unable to read ./src/name.txt").red().bold());
        let username = username + "$";

        if settings.autocomplete == true {
            let cmd = Text::new(&username)
                .with_autocomplete(&suggestions)
                .prompt();
            match cmd {
                Ok(cmd) => exec(cmd),
                Err(_) => println!("err: Unable to run command"),
            }
        } else {
            let cmd = Text::new(&username).prompt();
            match cmd {
                Ok(cmd) => exec(cmd),
                Err(_) => println!("err: Unable to run command"),
            }
        }
    }
}

// Autocomplete suggestions
fn suggestions(val: &str) -> Result<Vec<String>, CustomUserError> {
    let options = [
        "shell.help",
        "shell.exec",
        "shell.name",
        "shell.time",
        "shell.refresh",
        "shell.quit",
        "file.list",
        "file.create",
        "file.edit",
        "file.exec",
        "file.delete",
    ];

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
    } else if cmd == "shell.exec" {
        shell_exec();
    } else if cmd == "shell.name" {
        shell_name();
    } else if cmd == "shell.time" {
        shell_time();
    } else if cmd == "shell.refresh" {
        shell_refresh();
    } else if cmd == "shell.quit" {
        shell_quit();
    } else if cmd == "file.list" {
        file_list();
    } else if cmd == "file.create" {
        file_create();
    } else if cmd == "file.edit" {
        file_edit();
    } else if cmd == "file.exec" {
        file_exec();
    } else if cmd == "file.delete" {
        file_delete();
    } else {
        println!();
        println!("{}", format!("err: Unknown command, `{cmd}`").red().bold());
    }
    println!();
}
