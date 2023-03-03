# RustyShell

<img src="./images/shell.png" alt="shell.png"/>

*The new and improved version of the [PyLine Terminal](https://replit.com/@SCP-049/PyLine-Terminal?v=1)!*

The RustyShell is made to be the continuation of my old project, but written in a new programming language im learning!

Below this is section is how to set up the shell, the commands more in-depth, and the roadmap for the shell!

> NOTE: Still trying to re-add all the old features, so nothing new until it's all ported. (Unless I find something cool)

### Other things to check out:
- [Github Repository](https://github.com/SCP-049-replit/RustyShell)
- [LICENSE.md](LICENSE.md)
- [command.md](./more-md/command.md)
- [config.md](./more-md/config.md)

# Setup

Unlike the PyLine Terminal, installing dependencies is easy thanks to rusts Cargo package manager! All *you* need to do is to install the Shell, move to the Shell's directory, and run it!

1. Install Rust: Visit [rust.org](https://www.rust-lang.org/tools/install) to find out how to install! (Make sure to install everythin! Skip if you already have it installed!)

2. Install the RustyShell: `git clone https://github.com/SCP-049-replit/RustyShell.git`

3. Type `cd RustyShell`

4. Type `cargo run` to start it!

# Commands

Below are tables of every command, with their description.

## Shell Commands

| Command | Description |
| ----------- | ----------- |
| shell.help | Displays a list of commands in the shell|
| shell.exec | Lets the user enter a linux command, and runs it |
| shell.name | Edit the name of the user in the shell |
| shell.refresh | Clears the shell and reprints the welcome message |
| shell.quit | Stops the shell and returns you to the terminal |

## File Commands

| Command | Description |
| ----------- | ----------- |
| file.list | Lists all files found in the `./userfiles` directory |
| file.make | Creates a file named by user in the `./userfiles` directory |
| file.edit | Opens a text editor on the file stated by the user |
| file.exec | Compiles then runs a rust file's binary |
| file.delete | Deletes a file in the `./userfiles` directory |

# Development

This section is a personal checklist so I can keep track of what to do (and for users to see the development history!), Click the boxes to display!

### Version 0.0.2

- [x] Add file managment
- [x] Add linux command prompt
- [ ] Add text editor/file executing
- [x] Create Github repository

### Version 0.0.1

- [x] Add shell commands
- [x] Create markdown files
- [x] Make shell logo

# Final words

> A Shell written in rust! Made as a fun project to learn more about the language!

"I so happy you decided to look at (and maybe use) the RustyShell! It means a **lot** to me, and I hope you enjoy it!"

--- [SCP-049](https://replit.com/@SCP-049)

<img src="./images/shell.png" alt="shell.png" width="100"/>

---