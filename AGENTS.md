# Repository Guidelines

## Project Structure & Module Organization
`src/main.rs` is the CLI entry point: it reads a `.rcv` file, prints Markdown to stdout, and writes a PDF beside the input file. Keep parsing logic in `src/parser.rs`, resume data structures and builders in `src/resume.rs`, and PDF rendering in `src/pdf_renderer.rs`. Use `resume.rcv` as the local sample input; generated PDFs such as `resume.pdf` are build artifacts and should not drive tests.

## Build, Test, and Development Commands
Use Cargo for all routine work:

- `cargo run` parses `resume.rcv` and generates `resume.pdf`.
- `cargo run -- my-resume.rcv` renders a custom input file.
- `cargo build --release` builds the optimized CLI binary.
- `cargo fmt` applies standard Rust formatting.
- `cargo clippy --all-targets --all-features` checks for lint issues before review.
- `cargo test` runs unit and integration tests.

## Coding Style & Naming Conventions
Target Rust 2024 and let `rustfmt` define layout. Follow standard Rust naming: `snake_case` for files, modules, and functions; `UpperCamelCase` for structs and enums. Prefer small, single-purpose functions, builder-style APIs for resume data, and `anyhow::Context` when returning errors from I/O or rendering paths. Keep comments brief and only where the control flow is not obvious.

## Testing Guidelines
There is no dedicated `tests/` directory yet, so add unit tests close to the module they verify with `#[cfg(test)]`. Put broader CLI or file-system behavior in `tests/` when needed. Name tests after observable behavior, for example `parses_multiline_summary` or `renders_pdf_for_custom_input`. Cover parser directives, Markdown output, and PDF generation paths when touching those areas.

## Commit & Pull Request Guidelines
Recent history follows concise conventional subjects such as `docs: add project README` and `refactor(parser): improve idiomatic Rust patterns`. Keep commit messages imperative, scoped when useful, and focused on one change. Pull requests should explain the behavior change, list verification commands, and include a sample `.rcv` snippet or rendered output screenshot when modifying layout or PDF output.
