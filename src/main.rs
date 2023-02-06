use indicatif::ProgressBar;
use crate::terminal::*;
use colored::Colorize;
pub mod terminal;
use clearscreen;

fn main() {
    clearscreen::clear().expect("failed to clear screen");
    println!("RustyShell");
    println!("Version: 0.1.0");
    println!("Booting...");
    
    let bar = ProgressBar::new(10000000);
    for _ in 0..10000000 {
        bar.inc(1);
    }
    bar.finish();
    
    clearscreen::clear().expect("failed to clear screen");
    println!("{}", "Welcome to the RustyShell".green());
    println!("{}", "Type `shell.help` to see commands".green());
    println!("");
    run();
}