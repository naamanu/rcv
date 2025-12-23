mod parser;
mod pdf_renderer;
mod resume;

use anyhow::{Context, Result};
use std::env;
use std::path::Path;

fn main() -> Result<()> {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| "resume.rcv".to_string());

    println!("Parsing resume from {}...", filename);

    let my_resume = parser::parse_file(&filename)
        .with_context(|| format!("Failed to parse resume file '{}'", filename))?;

    // The Display trait is implemented to output Markdown.
    println!("{}", my_resume);

    // Generate PDF
    let pdf_filename = Path::new(&filename)
        .with_extension("pdf")
        .display()
        .to_string();

    pdf_renderer::export_to_pdf(&my_resume, &pdf_filename)
        .with_context(|| format!("Failed to generate PDF '{}'", pdf_filename))?;

    println!("Successfully generated {}", pdf_filename);

    Ok(())
}
