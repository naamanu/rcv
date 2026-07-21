mod parser;
mod pdf_renderer;
mod resume;

use anyhow::{Context, Result, bail};
use std::env;
use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "\
Usage: rcv [OPTIONS] [FILE]

Converts a .rcv resume file to Markdown (stdout) and PDF.

Arguments:
  FILE                 input .rcv file (default: resume.rcv)

Options:
  -o, --output <FILE>  PDF output path (default: input file with .pdf extension)
      --no-pdf         print Markdown only, skip PDF generation
  -h, --help           print this help";

struct Args {
    input: String,
    output: Option<String>,
    no_pdf: bool,
}

fn parse_args(argv: impl Iterator<Item = String>) -> Result<Option<Args>> {
    let mut input = None;
    let mut output = None;
    let mut no_pdf = false;

    let mut argv = argv.peekable();
    while let Some(arg) = argv.next() {
        match arg.as_str() {
            "-h" | "--help" => return Ok(None),
            "-o" | "--output" => {
                output = Some(argv.next().context("'--output' requires a file path")?);
            }
            "--no-pdf" => no_pdf = true,
            _ if arg.starts_with('-') => bail!("unknown option '{}'\n\n{}", arg, USAGE),
            _ => {
                if input.replace(arg).is_some() {
                    bail!("only one input file may be given\n\n{}", USAGE);
                }
            }
        }
    }

    Ok(Some(Args {
        input: input.unwrap_or_else(|| "resume.rcv".to_string()),
        output,
        no_pdf,
    }))
}

fn run(args: Args) -> Result<()> {
    eprintln!("Parsing resume from {}...", args.input);

    let outcome = parser::parse_file(&args.input)
        .with_context(|| format!("Failed to parse resume file '{}'", args.input))?;

    for warning in &outcome.warnings {
        eprintln!("warning: {}", warning);
    }

    // The Display trait is implemented to output Markdown.
    println!("{}", outcome.resume);

    if args.no_pdf {
        return Ok(());
    }

    let pdf_filename = args.output.unwrap_or_else(|| {
        Path::new(&args.input)
            .with_extension("pdf")
            .display()
            .to_string()
    });

    pdf_renderer::export_to_pdf(&outcome.resume, &pdf_filename)
        .with_context(|| format!("Failed to generate PDF '{}'", pdf_filename))?;

    eprintln!("Successfully generated {}", pdf_filename);

    Ok(())
}

fn main() -> Result<ExitCode> {
    match parse_args(env::args().skip(1))? {
        Some(args) => run(args).map(|_| ExitCode::SUCCESS),
        None => {
            println!("{}", USAGE);
            Ok(ExitCode::SUCCESS)
        }
    }
}
