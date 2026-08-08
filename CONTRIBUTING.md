# Contributing to rcv

Thanks for your interest in contributing.

## Prerequisites

- A stable Rust toolchain (`rustup` recommended), with `rustfmt` and `clippy`
- Git

## Getting started

```bash
git clone https://github.com/your-username/rcv.git
cd rcv
cargo build
cargo run -- resume.rcv        # renders resume.pdf
cargo run -- resume.rcv --markdown   # markdown to stdout
```

## Before you open a PR

CI runs exactly three checks, so run them locally first:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Clippy warnings are denied in CI, so a warning is a failure.

## Project structure

```
src/main.rs          # Entry point and CLI handling
src/parser.rs        # .rcv file parser
src/resume.rs        # Resume data structures and builders
src/pdf_renderer.rs  # PDF generation
tests/cli.rs         # End-to-end CLI tests
docs/                # Format, usage, and architecture guides
```

See [`docs/architecture.md`](docs/architecture.md) for how the pipeline, DSL builders,
and parser fit together.

## Making changes

1. Branch off `main`: `git checkout -b fix/your-change`
2. If you change the `.rcv` format, update [`docs/format.md`](docs/format.md) in the
   same PR — the format guide is the reference users rely on.
3. If you change PDF output, mention the visual difference in the PR description.
4. Open a PR against `main`, describing the change and linking any related issue.

## Reporting issues

Please include:

- Your OS and `rustc --version`
- The `.rcv` input that triggered it (a minimal snippet is ideal)
- Expected vs actual output

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
