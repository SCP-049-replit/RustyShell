// Displays help menu; shell.help
pub fn shell_help() {
    println!("
Commands:
    shell.help        displays help menu
    shell.config      edit shell settings
    shell.name        changes username
    shell.refresh     refreshes the shell
    shell.quit        ends the shell session");
}

// shell.config is in mod.rs

// Changes username for the CLI; shell.name
#[allow(unused_must_use)]
pub fn shell_name() {
    use std::io::Write;
    use std::io;
    
    let mut file = std::fs::File::create("./src/name.txt").expect("err: file creation failed");
    let mut new_name = String::new();
    
    io::stdin()
        .read_line(&mut new_name)
        .expect("Failed to read line");
    
    file.write_all(new_name.trim().as_bytes());
}

// Refreshes the CLI; shell.refresh
pub fn shell_refresh() {
    use crate::main;
    main();
}

// Ends shell session
pub fn shell_quit() {
    panic!("Shell session terminated");
}