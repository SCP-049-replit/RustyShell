use colored::Colorize;

// Displays help menu; shell.help
pub fn shell_help() {
    println!("
Commands:
    shell.help        displays help menu
    shell.exec        lets user run linux
    shell.name        changes username
    shell.time        displays the time
    shell.refresh     refreshes the shell
    shell.quit        ends the shell session
    
    file.list         lists all files
    file.make         creates a file
    file.edit         opens nano text editor
    file.exec         runs a rust file
    file.delete       deletes a file");
}

// Enters user into linux mode; shell.exec
pub fn shell_exec() {
    #![allow(unused)]
    
    use std::process::Command;
    use std::{thread, time};
    use std::io;

    let mut cmd = String::new();

    println!("{}", format!("Enter Linux command:").blue());
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read line");
    println!();
    
    thread::spawn(|| {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn();
    });
    thread::sleep(time::Duration::from_millis(10));
}

// Changes username for the CLI; shell.name
pub fn shell_name() {
    #![allow(unused)];
    
    use std::io::Write;
    use std::io;
    
    let mut file = std::fs::File::create("./src/name.txt").expect(&format!("err: File creation failed").red().bold());
    let mut new_name = String::new();

    println!("{}", format!("Enter new name:").blue());
    io::stdin()
        .read_line(&mut new_name)
        .expect("Failed to read line");
    
    file.write_all(new_name.trim().as_bytes());
}

// Shows current date and time; shell.time
pub fn shell_time() {
    #![allow(unused)];
    use std::{thread, time};

    println!();
    thread::spawn(move || {
        std::process::Command::new("sh")
            .arg("-c")
            .arg("date")
            .spawn();
    });
    thread::sleep(time::Duration::from_millis(10));
}

// Refreshes the CLI; shell.refresh
pub fn shell_refresh() {
    use crate::print_message;
    print_message();
}

// Ends shell session; shell.quit
pub fn shell_quit() {
    println!();
    println!("{}", format!("Shell session terminated").red());
    println!();
    
    use std::process::exit;
    exit(0);
}

// Lists files in the userfiles directory; file.list
pub fn file_list() {
    use std::fs;
    
    let paths = fs::read_dir("./userfiles").unwrap();
    
    println!("");
    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}

// Creates a file in userfiles; file.make
pub fn file_make() {
    #![allow(unused)]
    
    use std::fs::File;
    use std::io;

    let mut filename = String::new();
    println!("{}", format!("Enter file name:").blue());
    io::stdin()
        .read_line(&mut filename);
    let mut filename = String::from("./userfiles/") + filename.as_str();
    File::create(&mut filename);

    println!();
    print!("File created: {}", filename);
}

// Lets the user edit a file; file.edit
pub fn file_edit() {
    #![allow(unused)]
    use std::process::Command;
    use std::{thread, time};
    use std::io;

    let mut filename = String::new();
    println!("{}", format!("Enter file name:").blue());
    io::stdin()
        .read_line(&mut filename);
    let mut filename = String::from("nano ./userfiles/") + filename.as_str();

    thread::spawn(move || {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(filename.as_str())
            .spawn();
        thread::park();
    });
}

// Deletes a file; file.delete
pub fn file_delete() {
    #![allow(unused)]
    
    use std::fs;
    use std::io;

    let mut filename = String::new();
    println!("{}", format!("Enter file name:").blue());
    io::stdin()
        .read_line(&mut filename);
    let mut filename = String::from("./userfiles/") + filename.as_str();
    
    fs::remove_file(&mut filename)
      .expect(&format!("err: File delete failed").red().bold());

    println!();
    print!("File deleted: {}", filename);
}

// Compiles and runs a rust file; file.exec
pub fn file_exec() {
    #![allow(unused)]
    
    use std::process::Command;
    use std::{thread, time};
    use std::io;

    let mut path = String::new();
    let mut executable = String::new();

    println!("{}", format!("Enter file name:").blue());
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");
    let compile = String::from("rustc ./userfiles/") + &path;
        println!("{}", format!("Enter executable name:").blue());
        io::stdin()
            .read_line(&mut executable)
            .expect("Failed to read line");
    
    thread::spawn(move || {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(compile.as_str())
            .spawn();
        thread::sleep(time::Duration::from_secs(5));
        let executable = String::from("./") + executable.as_str();
        std::process::Command::new("sh")
            .arg("-c")
            .arg(executable.as_str())
            .spawn();
    });
    thread::sleep(time::Duration::from_secs(10));
}