# RCV - Resume CV Generator

A simple resume generator written in Rust that converts declarative `.rcv` files into beautiful PDFs.

## Features

- Simple, human-readable resume format
- Automatic PDF generation
- Markdown output support
- LaTeX-style PDF output using bundled Computer Modern fonts (CMU Serif, SIL OFL — see `fonts/LICENSE-OFL.txt`)

## Installation

```bash
cargo build --release
```

## Usage

```bash
# Use default resume.rcv file
cargo run

# Specify a custom resume file
cargo run -- my-resume.rcv

# Choose the PDF output path
cargo run -- my-resume.rcv -o out/cv.pdf

# Markdown only (clean stdout, safe to pipe)
cargo run -- my-resume.rcv --no-pdf > resume.md
```

Progress and warning messages (unknown directives, typos, missing fields) are printed to stderr, so stdout always contains only the Markdown output.

## Resume Format

Create a `.rcv` file using the following format:

```
@name: Your Name
@email: your.email@example.com
@phone: +1-234-567-8900
@website: https://yourwebsite.com

@links:
LinkedIn: https://linkedin.com/in/yourname
GitHub: https://github.com/yourname

@summary:
Your professional summary goes here.
You can use multiple lines.

@skills:
Languages: Rust, Python, JavaScript
Technologies: React, Git, Docker, Kubernetes

@experience:
title: Software Engineer
company: Company Name
date: 2020 - Present
description: Location
- Key achievement or responsibility
- Another achievement with metrics

@project:
name: my-tool
description: One-line description of what it does.
tech: Rust, SQLite
link: https://github.com/yourname/my-tool

@education:
school: University Name
degree: Bachelor of Science in Computer Science
year: 2016 - 2020
location: City, Country
```

Skill categories are free-form: any `Label: item, item` line inside `@skills:` becomes its own category. Repeat `@experience:`, `@project:`, and `@education:` blocks for multiple entries.

## Example Run

Below is a demonstration of the RCV CLI converting a `.rcv` file to a finished format:

```bash
$ cargo run -- resume.rcv
Parsing resume from resume.rcv...
# John Doe

**Email:** john.doe@example.com | **Phone:** +1-234-567-8900 | **Web:** https://johndoe.com | [GitHub](https://github.com/johndoe)

## Summary
A highly motivated Software Engineer with a passion for Rust and declarative formatting.

## Education
**State University**, BSc Computer Science (2016 - 2020)

## Experience
### Software Engineer @ Example Corp (2020) - Present

_Remote_

- Built an internal PDF generation service that reduced rendering time by 80%.
- Migrated legacy microservices from Java to Axum (Rust).

## Projects
- **[my-tool](https://github.com/johndoe/my-tool)** _(Rust)_: Minimal project scaffolding tool.

## Skills
#### Languages
Rust, Python, JavaScript

Loading fonts...
Rendering PDF to resume.pdf...
Successfully generated resume.pdf
```

(The progress lines are printed to stderr; only the Markdown itself goes to stdout.)
*Screenshot: The `cargo run` text output, representing a standard resume build pipeline.*

## Documentation

For a comprehensive breakdown of how RCV operates under the hood, visit the specific guides in our `docs/` repository:
- [Format Guide](docs/format.md): Syntax references for `.rcv` files.
- [Usage Guide](docs/usage.md): Instructions on CLI functionality and options.
- [Architecture](docs/architecture.md): A look into the pipeline, DSL builders, and parser implementations.

## File Structure

- `src/main.rs` - Entry point and CLI handling
- `src/parser.rs` - `.rcv` file parser
- `src/resume.rs` - Resume data structures and builders
- `src/pdf_renderer.rs` - PDF generation

## Output

The tool generates two outputs:
1. Markdown output to stdout
2. PDF file (e.g., `resume.pdf`)

## License

MIT
