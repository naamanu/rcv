use crate::resume::{EducationBuilder, ExperienceBuilder, Resume, ResumeBuilder};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Parses a .rcv file into a Resume struct.
/// The format expects directives starting with `@`.
/// Example:
/// @name: John Doe
/// @experience:
/// title: ...
pub fn parse_file(path: impl AsRef<Path>) -> Result<Resume> {
    let content = fs::read_to_string(path).context("Failed to read resume file")?;
    parse_content(&content)
}

fn parse_content(content: &str) -> Result<Resume> {
    let mut builder = ResumeBuilder::default();

    // We parse line by line, but some directives start a block (like @experience).
    // This simple state machine tracks what we are currently building.
    enum State {
        Root,
        Experience(ExperienceBuilder),
        Education(EducationBuilder),
    }

    let mut state = State::Root;

    // Helper to finish the current block and add it to the resume builder
    // We use a macro or closure to avoid borrowing issues, or just handle it iteratively.
    // Since builders consume self, we need to be careful.
    // simpler approach: modify a mutable Resume object directly?
    // Our Builder consumes self, which is great for chaining but harder for state parsing loop.
    // Let's modify the Builder to allow holding intermediate state?
    // Or we just gather structs and add them at the end.
    // Actually, ResumeBuilder can hold `resume`. We can't access it easily if we consume it.
    // Let's rely on the internal `Resume` struct being accessible or just use the `field` methods?

    // Refactor: Let's assume we can access the underlying resume in the builder or just build side-lists.
    // Note: The public API of ResumeBuilder consumes self.
    // To keep it simple, let's collect the components separately and use the builder at the end or
    // fundamentally, we can change the parse strategy to collecting `sections`.

    // Let's parse into a temporary structure or direct fields.
    // Since we are inside the crate, we *could* modify Resume directly if fields are pub, which they are.
    // But let's stick to the builder usage to respect the "API".
    // We will accumulate experiences and educations in vectors and add them in batch if the builder supports it,
    // or just chain them at the end.

    let mut name = String::new();
    let mut email = String::new();
    let mut phone = None;
    let mut website = None;
    let mut summary = String::new();
    let mut skills = Vec::new();

    let mut experiences = Vec::new();
    let mut educations = Vec::new();

    // Loop through lines
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.starts_with("@experience:") {
            state = State::Experience(ExperienceBuilder::default());
            i += 1;
        } else if line.starts_with("@education:") {
            state = State::Education(EducationBuilder::default());
            i += 1;
        } else if let Some(stripped) = line.strip_prefix("@name:") {
            name = stripped.trim().to_string();
            state = State::Root;
            i += 1;
        } else if let Some(stripped) = line.strip_prefix("@email:") {
            email = stripped.trim().to_string();
            state = State::Root;
            i += 1;
        } else if let Some(stripped) = line.strip_prefix("@phone:") {
            phone = Some(stripped.trim().to_string());
            state = State::Root;
            i += 1;
        } else if let Some(stripped) = line.strip_prefix("@website:") {
            website = Some(stripped.trim().to_string());
            state = State::Root;
            i += 1;
        } else if let Some(stripped) = line.strip_prefix("@summary:") {
            // Read until next directive
            state = State::Root;
            i += 1;
            let mut summary_lines = Vec::new();
            if !stripped.trim().is_empty() {
                summary_lines.push(stripped.trim());
            }
            while i < lines.len() && !lines[i].trim().starts_with("@") {
                let l = lines[i].trim();
                if !l.is_empty() {
                    summary_lines.push(l);
                }
                i += 1;
            }
            summary = summary_lines.join("\n");
        } else if let Some(stripped) = line.strip_prefix("@skills:") {
            state = State::Root;
            // Can be inline or multiline?
            // Assume multiline until next directive or empty line?
            // The example shows comma separated on next line.
            i += 1;
            let mut skill_text = stripped.trim().to_string();
            while i < lines.len() && !lines[i].trim().starts_with("@") {
                let l = lines[i].trim();
                if !l.is_empty() {
                    if !skill_text.is_empty() {
                        skill_text.push_str(", ");
                    }
                    skill_text.push_str(l);
                }
                i += 1;
            }

            for s in skill_text.split(',') {
                let s = s.trim();
                if !s.is_empty() {
                    skills.push(s.to_string());
                }
            }
        } else {
            // Inside a block?
            match &mut state {
                State::Root => {
                    // Unknown root line, ignore or log
                    i += 1;
                }
                State::Experience(exp_builder) => {
                    // Parse experience key-values
                    if let Some(val) = line.strip_prefix("title:") {
                        *exp_builder = std::mem::take(exp_builder).title(val.trim());
                    } else if let Some(val) = line.strip_prefix("company:") {
                        *exp_builder = std::mem::take(exp_builder).company(val.trim());
                    } else if let Some(val) = line.strip_prefix("date:") {
                        // Split start/end if possible "Start - End"
                        let date_str = val.trim();
                        if let Some((start, end)) = date_str.split_once("-") {
                            *exp_builder = std::mem::take(exp_builder)
                                .start(start.trim())
                                .end(end.trim());
                        } else {
                            *exp_builder = std::mem::take(exp_builder).start(date_str);
                        }
                    } else if let Some(val) = line.strip_prefix("description:") {
                        *exp_builder = std::mem::take(exp_builder).description(val.trim());
                    } else if let Some(val) = line.strip_prefix("-") {
                        *exp_builder = std::mem::take(exp_builder).highlight(val.trim());
                    } else {
                        // assume part of previous description? or just ignore
                    }

                    // Check lookahead to see if block ends (next line is directive)
                    if i + 1 >= lines.len() || lines[i + 1].trim().starts_with("@") {
                        // End of block
                        let finished_exp = std::mem::take(exp_builder).finish();
                        experiences.push(finished_exp);
                        state = State::Root;
                    }
                    i += 1;
                }
                State::Education(edu_builder) => {
                    if let Some(val) = line.strip_prefix("school:") {
                        *edu_builder = std::mem::take(edu_builder).school(val.trim());
                    } else if let Some(val) = line.strip_prefix("degree:") {
                        *edu_builder = std::mem::take(edu_builder).degree(val.trim());
                    } else if let Some(val) = line.strip_prefix("year:") {
                        *edu_builder = std::mem::take(edu_builder).year(val.trim());
                    }

                    if i + 1 >= lines.len() || lines[i + 1].trim().starts_with("@") {
                        let finished_edu = std::mem::take(edu_builder).finish();
                        educations.push(finished_edu);
                        state = State::Root;
                    }
                    i += 1;
                }
            }
        }
    }

    // Construct final resume
    let mut builder = Resume::build().name(&name).email(&email);

    if let Some(p) = phone {
        builder = builder.phone(&p);
    }
    if let Some(w) = website {
        builder = builder.website(&w);
    }
    if !summary.is_empty() {
        builder = builder.summary(&summary);
    }

    builder = builder.skills(&skills.iter().map(|s| s.as_str()).collect::<Vec<_>>());

    // We need to extend the builder to accept pre-built objects or reconstruct them.
    // The current builder uses closures.
    // Limitation of the current builder API: It expects closures for experience/education.
    // Let's modify the builder in `resume.rs` to allow adding structs directly, or access fields.
    // Actually, we can just use the existing closure API by moving the data in.

    for exp in experiences {
        builder = builder.experience(move |mut b| {
            b = b
                .title(&exp.title)
                .company(&exp.company)
                .start(&exp.start_date);
            if let Some(end) = &exp.end_date {
                b = b.end(end);
            }
            if let Some(desc) = &exp.description {
                b = b.description(desc);
            }
            for h in &exp.highlights {
                b = b.highlight(h);
            }
            b
        });
    }

    for edu in educations {
        builder = builder.education(move |mut b| {
            b = b.school(&edu.school).degree(&edu.degree).year(&edu.year);
            b
        });
    }

    Ok(builder.finish())
}
