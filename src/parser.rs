use crate::resume::{EducationBuilder, ExperienceBuilder, Resume, ResumeBuilder, SkillsBuilder};
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
    let _builder = ResumeBuilder::default();

    // Parse line by line, but @directives start a block (like @experience).
    // A simple state machine tracks what we are currently building.
    enum State {
        Root,
        Experience(ExperienceBuilder),
        Education(EducationBuilder),
        Skills(SkillsBuilder),
    }

    let mut state = State::Root;

    // Helper to flush current state
    let flush_state = |state: &mut State,
                       experiences: &mut Vec<crate::resume::Experience>,
                       educations: &mut Vec<crate::resume::Education>,
                       skills: &mut Vec<crate::resume::Skills>| {
        match std::mem::replace(state, State::Root) {
            State::Skills(b) => skills.push(b.finish()),
            State::Experience(b) => experiences.push(b.finish()),
            State::Education(b) => educations.push(b.finish()),
            State::Root => {}
        }
    };

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

        if line.starts_with("@") {
            // New directive starting, flush previous block
            flush_state(&mut state, &mut experiences, &mut educations, &mut skills);
        }

        if line.starts_with("@skills:") {
            state = State::Skills(SkillsBuilder::default());
            i += 1;
        } else if line.starts_with("@experience:") {
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
                    i += 1;
                }
                State::Skills(skills_builder) => {
                    if let Some(val) = line.strip_prefix("languages:") {
                        // Parse comma separated list
                        let list: Vec<String> = val
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        *skills_builder = std::mem::take(skills_builder).languages(list);
                    } else if let Some(val) = line.strip_prefix("frameworks:") {
                        let list: Vec<String> = val
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        *skills_builder = std::mem::take(skills_builder).frameworks(list);
                    } else if let Some(val) = line.strip_prefix("tools:") {
                        let list: Vec<String> = val
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        *skills_builder = std::mem::take(skills_builder).tools(list);
                    }
                    i += 1;
                }
            }
        }
    }

    // Final flush
    flush_state(&mut state, &mut experiences, &mut educations, &mut skills);

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

    for s in skills {
        builder = builder.merge_skills(s);
    }

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
