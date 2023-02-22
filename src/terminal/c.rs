use colored::Colorize;

// Displays help menu; shell.help
pub fn shell_help() {
    println!("
Commands:
    shell.help        displays help menu
    shell.name        changes username
    shell.refresh     refreshes the shell
    shell.quit        ends the shell session
    
    file.list         lists all files
    file.make         creates a file
    file.edit         edit a file
    file.delete       deletes a file");
}

// Changes username for the CLI; shell.name
pub fn shell_name() {
    #![allow(unused)];
    
    use std::io::Write;
    use std::io;
    
    let mut file = std::fs::File::create("./src/name.txt").expect(&format!("err: File creation failed").red().bold());
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

// Ends shell session; shell.quit
pub fn shell_quit() {
    panic!("Shell session terminated");
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
pub fn file_create() {
    #![allow(unused)]
    
    use std::fs::File;
    use std::io;

    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename);
    let mut filename = String::from("./userfiles/") + filename.as_str();
    File::create(&mut filename);

    println!();
    print!("File created: {}", filename);
}

// Lets the user edit a file; file.edit
pub fn file_edit() {
    
}

// Deletes a file; file.delete
pub fn file_delete() {
    #![allow(unused)]
    
    use std::fs;
    use std::io;

    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename);
    let mut filename = String::from("./userfiles/") + filename.as_str();
    
    fs::remove_file(&mut filename)
      .expect(&format!("err: File delete failed").red().bold());

    println!();
    print!("File deleted: {}", filename);
}