mod parser;
mod pdf_renderer;
mod resume;

use std::env;

fn main() {
    // Check if a file argument is provided, otherwise default to "resume.rcv"
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "resume.rcv"
    };

    println!("Parsing resume from {}...", filename);

    let my_resume = match parser::parse_file(filename) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error parsing file '{}': {:#}", filename, e);
            return;
        }
    };

    // The Display trait is implemented to output Markdown.
    println!("{}", my_resume);

    // Generate PDF
    let pdf_filename = format!("{}.pdf", filename.trim_end_matches(".rcv"));
    if let Err(e) = pdf_renderer::export_to_pdf(&my_resume, &pdf_filename) {
        eprintln!("Error generating PDF: {:#}", e);
    } else {
        println!("Successfully generated {}", pdf_filename);
    }
}
