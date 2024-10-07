# The simplest hotreload example in Rust

# Quick start
```console
$ cargo run
```

# Important
> Run or compile the project only in the root directory, so `PLUG_PATH` in `src/main.rs` is correct.

# Check if hotreloading works fr fr
> To achieve this, start by `cargo run` in the root directory in a separate process. 
> Next, go to `plug/plug.rs` and modify the `"hello world"` string to something else.
> After making the change, recompile the project with `cargo build` and check if the updated string is reflected in the previously running process.
