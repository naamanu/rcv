# Architecture Overview

The RCV tool is intentionally small, delegating distinct responsibilities to functional modules inside the `src/` directory.

## Core Modules

### `src/main.rs`
The entry point handling CLI arguments (`std::env::args`). It coordinates the pipeline: calling the parser, printing standard Markdown output via `Display`, and finally triggering the PDF generation routine.

### `src/parser.rs`
Contains the `.rcv` parsing routine (`parse_file`).
- Treats the input file sequentially via custom line-by-line streaming.
- State-machine based: Uses an internal enum `State` to manage nested parsing contexts (like building up a multi-line `Summary` or continuing an `Experience` list block).
- Commits completed blocks via `.flush()` automatically switching paths based on the encountered `@` directive.

### `src/resume.rs`
The data structures and Domain Specific Language builders.
- Contains the root `Resume` struct and inner structures (`Experience`, `Education`, `Skills`).
- Utilizes the **Builder Pattern** heavily (`ResumeBuilder`, `ExperienceBuilder`), making programatic or parsed construction clean and fault tolerant.
- Encompasses the `Display` trait implementation for `Resume` which defines how the data structure outputs as structured Markdown.

### `src/pdf_renderer.rs`
Converts a constructed `Resume` representation into a final PDF layout. Relies heavily on external crates to calculate typesetting and export the binary file format directly.
