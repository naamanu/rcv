# Usage Guide

RCV (Resume CV Generator) is a CLI tool built in Rust. It quickly unpackages a declarative `<name>.rcv` configuration into standard Markdown and a beautifully styled PDF.

## Compilation

You can compile RCV directly from source using Cargo:

```bash
# Build the optimized release binary
cargo build --release

# And find it in the target/release/ directory
./target/release/rcv --help
```

## Running

If you are developing or simply testing a file without wanting to permanently build a system-wide executable, use `cargo run`.

By default, executing RCV without arguments will look for `resume.rcv` in the current working directory:
```bash
cargo run
```

To specify a custom `.rcv` file, pass the filename as the first positional argument. **Note:** when using `cargo run`, you need to separate arguments with `--`:
```bash
cargo run -- my-resume.rcv
```

## Outputs

RCV automatically handles the generation of two assets every time it is run:

1. **Standard Output (Stdout):**
   The compiled resume is printed to your terminal mapped directly as structural Markdown. This is very useful if you want to pipe the text into another tool.
   
   ```bash
   cargo run > out.md
   ```

2. **PDF File:**
   Alongside the `.rcv` input file, RCV places a matching `.pdf` file. For instance, running `cargo run -- jane_doe.rcv` will silently write `jane_doe.pdf` in the same directory.
