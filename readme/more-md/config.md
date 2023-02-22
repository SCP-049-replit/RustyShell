# Shell Configuration 

<img src="/readme/images/config.png" alt="config.png" width="400"/>

Believe it or not, but the RustyShell has some *special* settings! Inside of `./src/terminal/mod.rs`, is a Struct called `Config`. This Struct holds the secret settings of the RustyShell

## A look at the Struct
Below is the stucts values, so you can see what kind of things there are to try!

```rust
struct Config {
    autocomplete: bool,
}
```
Now you may be wondering, "how do I enable some of these?" Well it couldnt be more simple!

One of the first things you see in the `run` function is the Struct definition, all you got to do is change the values to true or false, as to your needs!

```rust
pub fn run() {
    loop {
        let settings = Config {
            autocomplete: true,
        };
        //...
```

Thats all it takes to enable the special settings!