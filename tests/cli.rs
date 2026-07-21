use std::fs;
use std::path::PathBuf;
use std::process::Command;

const SAMPLE: &str = "\
@name: Jane Doe
@email: jane@example.com

@summary:
Builds reliable developer tools.

@experience:
title: Staff Engineer
company: Example Co
date: 2022 - Present
- Reduced release friction
";

fn scratch_dir(test_name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(test_name);
    fs::create_dir_all(&dir).expect("scratch dir should be creatable");
    dir
}

#[test]
fn generates_markdown_and_pdf_end_to_end() {
    let dir = scratch_dir("generates_markdown_and_pdf_end_to_end");
    let input = dir.join("sample.rcv");
    let pdf = dir.join("out.pdf");
    fs::write(&input, SAMPLE).expect("sample input should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_rcv"))
        .arg(&input)
        .arg("--output")
        .arg(&pdf)
        .output()
        .expect("binary should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // stdout must be pure Markdown, with status messages kept on stderr.
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");
    assert!(stdout.starts_with("# Jane Doe"), "stdout: {}", stdout);
    assert!(stdout.contains("### Staff Engineer @ Example Co (2022) - Present"));

    let pdf_bytes = fs::read(&pdf).expect("PDF should have been written");
    assert!(pdf_bytes.starts_with(b"%PDF"), "output is not a PDF");
}

#[test]
fn no_pdf_flag_skips_pdf_generation() {
    let dir = scratch_dir("no_pdf_flag_skips_pdf_generation");
    let input = dir.join("sample.rcv");
    fs::write(&input, SAMPLE).expect("sample input should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_rcv"))
        .arg(&input)
        .arg("--no-pdf")
        .output()
        .expect("binary should run");

    assert!(output.status.success());
    assert!(!input.with_extension("pdf").exists());
}

#[test]
fn missing_name_fails_with_helpful_error() {
    let dir = scratch_dir("missing_name_fails_with_helpful_error");
    let input = dir.join("broken.rcv");
    fs::write(&input, "@email: jane@example.com\n").expect("input should be writable");

    let output = Command::new(env!("CARGO_BIN_EXE_rcv"))
        .arg(&input)
        .arg("--no-pdf")
        .output()
        .expect("binary should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("@name"), "stderr: {}", stderr);
}

#[test]
fn help_flag_prints_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_rcv"))
        .arg("--help")
        .output()
        .expect("binary should run");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage: rcv"), "stdout: {}", stdout);
}
