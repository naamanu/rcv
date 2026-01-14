# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

```bash
cargo build --release          # Build optimized binary
cargo run                      # Run with default resume.rcv
cargo run -- my-resume.rcv     # Run with custom resume file
```

No tests exist yet. Standard Rust tooling (rustfmt, clippy) works with defaults.

## Architecture

RCV converts declarative `.rcv` files into PDF resumes. The data flows through three stages:

```
.rcv file → parser.rs → Resume struct → pdf_renderer.rs → .pdf file
                              ↓
                        Markdown to stdout
```

### Modules

- **main.rs** - CLI entry point; parses arguments, orchestrates parsing and rendering
- **parser.rs** - State machine parser for .rcv format; handles directives like `@name`, `@experience`, `@skills`
- **resume.rs** - Domain model (`Resume`, `Experience`, `Education`, `Skills`) with builder pattern for fluent construction; implements `Display` for Markdown output
- **pdf_renderer.rs** - Uses `genpdf` with system Arial fonts via `font-kit` to generate styled PDFs

### Parser State Machine

The parser uses four states to handle nested content:
- `Root` - Top-level directives (@name, @email, @summary, etc.)
- `Experience` - Parsing job entries (title, company, date, description, bullet highlights)
- `Education` - Parsing education entries (school, degree, year)
- `Skills` - Parsing skill categories (languages, frameworks, tools)

### .rcv Format Reference

```
@name: Your Name
@email: email@example.com
@phone: +1-234-567-8900
@website: https://yoursite.com

@summary:
Multi-line summary text.

@skills:
languages: Rust, Python, JavaScript
frameworks: React, Flask
tools: Git, Docker

@experience:
title: Software Engineer
company: Company Name
date: 2020 - Present
description: Location
- Bullet point achievement
- Another achievement

@education:
school: University Name
degree: BSc Computer Science
year: 2016 - 2020
```

Multiple `@experience` and `@education` blocks are supported.
