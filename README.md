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
