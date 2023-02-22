# Making Commands

<img src="/readme/images/package.png" alt="pack.png" width="400"/>

The RustyShell's customizability is great, letting you add your own commands, but how?

This document will answer excactly that!

## 1. Make the command!

Go to `/src/terminal/c.rs`, in there is of the commands for the terminal, including your own! Make a function (Dont forget to make it public!) and your done with the first step! 

> (If you need packages, made sure to edit `cargo.toml`)

## 2. Make the command run!

Now that your done making the command, its time to execute it! Go to `/src/terminal/mod.rs`, and scroll down to the `exec` function. Copy and paste this code somewhere in the else if section:

```rust
} else if cmd == "command.name" {
        function_name();
```

> You can also add your command to the auto complete list! Just go to the `suggestions` function and add a string to the `options` array!

## 3. Test it!

You gotta make sure it works before you use/publish it! If it doesnt work, keep fixin it until it does!