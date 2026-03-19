# RCV - Resume CV Generator

A simple resume generator written in Rust that converts declarative `.rcv` files into beautiful PDFs.

## Features

- Simple, human-readable resume format
- Automatic PDF generation
- Markdown output support
- System font integration

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
```

## Resume Format

Create a `.rcv` file using the following format:

```
@name: Your Name
@email: your.email@example.com
@phone: +1-234-567-8900
@website: https://yourwebsite.com

@summary:
Your professional summary goes here.
You can use multiple lines.

@skills:
languages: Rust, Python, JavaScript
frameworks: React, Next.js, Flask
tools: Git, Docker, Kubernetes

@experience:
title: Software Engineer
company: Company Name
date: 2020 - Present
description: Location
- Key achievement or responsibility
- Another achievement with metrics

@education:
school: University Name
degree: Bachelor of Science in Computer Science
year: 2016 - 2020
```

## Example Run

Below is a demonstration of the RCV CLI converting a `.rcv` file to a finished format:

```bash
$ cargo run -- resume.rcv
Parsing resume from resume.rcv...
# John Doe

**Email:** john.doe@example.com | **Phone:** +1-234-567-8900 | **Web:** https://johndoe.com

## Summary
A highly motivated Software Engineer with a passion for Rust and declarative formatting.

## Skills
#### Languages
Rust, Python, JavaScript

## Experience
### Software Engineer @ Example Corp (2020) - Present

_Remote_

- Built an internal PDF generation service that reduced rendering time by 80%.
- Migrated legacy microservices from Java to Axum (Rust).

## Education
**State University**, BSc Computer Science (2016 - 2020)

Loading system fonts...
Rendering PDF to resume.pdf...
Successfully generated resume.pdf
```
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
