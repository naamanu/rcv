use crate::resume::{EducationBuilder, ExperienceBuilder, Resume, SkillsBuilder};
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

#[derive(Debug)]
enum Directive {
    Name,
    Email,
    Phone,
    Website,
    Summary,
    Skills,
    Experience,
    Education,
}

impl Directive {
    fn from_line(line: &str) -> Option<(Self, &str)> {
        let line = line.strip_prefix('@')?;

        if let Some(rest) = line.strip_prefix("name:") {
            Some((Self::Name, rest.trim()))
        } else if let Some(rest) = line.strip_prefix("email:") {
            Some((Self::Email, rest.trim()))
        } else if let Some(rest) = line.strip_prefix("phone:") {
            Some((Self::Phone, rest.trim()))
        } else if let Some(rest) = line.strip_prefix("website:") {
            Some((Self::Website, rest.trim()))
        } else if let Some(rest) = line.strip_prefix("summary:") {
            Some((Self::Summary, rest.trim()))
        } else if line.starts_with("skills:") {
            Some((Self::Skills, ""))
        } else if line.starts_with("experience:") {
            Some((Self::Experience, ""))
        } else if line.starts_with("education:") {
            Some((Self::Education, ""))
        } else {
            None
        }
    }
}

/// Helper to parse comma-separated values into a Vec<String>
fn parse_csv(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

/// Helper to parse date ranges like "2020 - 2023" or "2020"
fn parse_date_range(date_str: &str) -> (String, Option<String>) {
    date_str
        .split_once('-')
        .map(|(start, end)| (start.trim().to_string(), Some(end.trim().to_string())))
        .unwrap_or_else(|| (date_str.to_string(), None))
}

fn parse_content(content: &str) -> Result<Resume> {
    enum State {
        Root,
        Experience(ExperienceBuilder),
        Education(EducationBuilder),
        Skills(SkillsBuilder),
    }

    impl State {
        fn flush(
            self,
            experiences: &mut Vec<crate::resume::Experience>,
            educations: &mut Vec<crate::resume::Education>,
            skills: &mut Vec<crate::resume::Skills>,
        ) {
            match self {
                State::Skills(b) => skills.push(b.finish()),
                State::Experience(b) => experiences.push(b.finish()),
                State::Education(b) => educations.push(b.finish()),
                State::Root => {}
            }
        }
    }

    let mut state = State::Root;
    let mut name = String::new();
    let mut email = String::new();
    let mut phone = None;
    let mut website = None;
    let mut summary = String::new();
    let mut skills = Vec::new();
    let mut experiences = Vec::new();
    let mut educations = Vec::new();

    let mut lines = content.lines().map(str::trim).peekable();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        if let Some((directive, rest)) = Directive::from_line(line) {
            state.flush(&mut experiences, &mut educations, &mut skills);

            match directive {
                Directive::Name => {
                    name = rest.to_string();
                    state = State::Root;
                }
                Directive::Email => {
                    email = rest.to_string();
                    state = State::Root;
                }
                Directive::Phone => {
                    phone = Some(rest.to_string());
                    state = State::Root;
                }
                Directive::Website => {
                    website = Some(rest.to_string());
                    state = State::Root;
                }
                Directive::Summary => {
                    state = State::Root;
                    let mut summary_lines = Vec::new();
                    if !rest.is_empty() {
                        summary_lines.push(rest);
                    }
                    while let Some(&next_line) = lines.peek() {
                        if next_line.starts_with('@') {
                            break;
                        }
                        if let Some(line) = lines.next() {
                            if !line.is_empty() {
                                summary_lines.push(line);
                            }
                        }
                    }
                    summary = summary_lines.join("\n");
                }
                Directive::Skills => state = State::Skills(SkillsBuilder::default()),
                Directive::Experience => state = State::Experience(ExperienceBuilder::default()),
                Directive::Education => state = State::Education(EducationBuilder::default()),
            }
        } else {
            match &mut state {
                State::Root => {}
                State::Experience(builder) => {
                    if let Some(val) = line.strip_prefix("title:") {
                        *builder = std::mem::take(builder).title(val.trim());
                    } else if let Some(val) = line.strip_prefix("company:") {
                        *builder = std::mem::take(builder).company(val.trim());
                    } else if let Some(val) = line.strip_prefix("date:") {
                        let (start, end) = parse_date_range(val.trim());
                        *builder = std::mem::take(builder).start(&start);
                        if let Some(end_date) = end {
                            *builder = std::mem::take(builder).end(&end_date);
                        }
                    } else if let Some(val) = line.strip_prefix("description:") {
                        *builder = std::mem::take(builder).description(val.trim());
                    } else if let Some(val) = line.strip_prefix('-') {
                        *builder = std::mem::take(builder).highlight(val.trim());
                    }
                }
                State::Education(builder) => {
                    if let Some(val) = line.strip_prefix("school:") {
                        *builder = std::mem::take(builder).school(val.trim());
                    } else if let Some(val) = line.strip_prefix("degree:") {
                        *builder = std::mem::take(builder).degree(val.trim());
                    } else if let Some(val) = line.strip_prefix("year:") {
                        *builder = std::mem::take(builder).year(val.trim());
                    }
                }
                State::Skills(builder) => {
                    if let Some(val) = line.strip_prefix("languages:") {
                        *builder = std::mem::take(builder).languages(parse_csv(val));
                    } else if let Some(val) = line.strip_prefix("frameworks:") {
                        *builder = std::mem::take(builder).frameworks(parse_csv(val));
                    } else if let Some(val) = line.strip_prefix("tools:") {
                        *builder = std::mem::take(builder).tools(parse_csv(val));
                    }
                }
            }
        }
    }

    state.flush(&mut experiences, &mut educations, &mut skills);

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
        builder = builder.education(move |b| {
            b.school(&edu.school).degree(&edu.degree).year(&edu.year)
        });
    }

    Ok(builder.finish())
}
