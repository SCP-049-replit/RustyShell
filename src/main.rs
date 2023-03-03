use indicatif::ProgressBar;
use crate::terminal::*;
use colored::Colorize;
use std::path::Path;
pub mod terminal;
use clearscreen;

fn main() {
    clearscreen::clear().expect("failed to clear screen");
    println!("RustyShell");
    println!("Version: 0.0.2");
    println!("Booting...");
    
    let bar = ProgressBar::new(100000);
    for _ in 0..100000 {
        bar.inc(1);

        // Saves files to variables
        let license = Path::new("./README/LICENSE.md").exists();
        let mod_rs = Path::new("./src/terminal/mod.rs").exists();
        let c_rs = Path::new("./src/terminal/mod.rs").exists();

        // Checks if files exist, panicks if they don't
        if license == false {
            panic!("err: ./readme/LICENSE.md Not found!");
        }
        if mod_rs == false {
            panic!("err: ./src/terminal/mod.rs Not found!");
        }
        if c_rs == false {
            panic!("err: ./src/terminal/c.rs Not found!");
        }
    }
    bar.finish();
    
    clearscreen::clear().expect("failed to clear screen");
    println!("{}", "Welcome to the RustyShell".green());
    println!("{}", "Type `shell.help` to see commands".green());
    println!("");
    run();
}