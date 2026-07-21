use crate::resume::{
    EducationBuilder, ExperienceBuilder, Link, ProjectBuilder, Resume, SkillsBuilder,
};
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;

/// The result of parsing a .rcv file: the resume plus any warnings about
/// input the parser did not understand and skipped.
#[derive(Debug)]
pub struct ParseOutcome {
    pub resume: Resume,
    pub warnings: Vec<String>,
}

/// Parses a .rcv file into a Resume struct.
/// The format expects directives starting with `@`.
/// Example:
/// @name: John Doe
/// @experience:
/// title: ...
pub fn parse_file(path: impl AsRef<Path>) -> Result<ParseOutcome> {
    let content = fs::read_to_string(path).context("Failed to read resume file")?;
    parse_content(&content)
}

#[derive(Debug)]
enum Directive {
    Name,
    Email,
    Phone,
    Website,
    Links,
    Summary,
    Skills,
    Experience,
    Project,
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
        } else if line.starts_with("links:") {
            Some((Self::Links, ""))
        } else if line.starts_with("skills:") {
            Some((Self::Skills, ""))
        } else if line.starts_with("experience:") {
            Some((Self::Experience, ""))
        } else if line.starts_with("project:") || line.starts_with("projects:") {
            Some((Self::Project, ""))
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

/// Helper to parse date ranges like "2020 - 2023" or "2020".
/// A spaced " - " separator wins over a bare hyphen so dates such as
/// "2020-09 - 2021-05" split at the range separator, not inside a date.
fn parse_date_range(date_str: &str) -> (String, Option<String>) {
    date_str
        .split_once(" - ")
        .or_else(|| date_str.split_once('-'))
        .map(|(start, end)| (start.trim().to_string(), Some(end.trim().to_string())))
        .unwrap_or_else(|| (date_str.to_string(), None))
}

fn parse_content(content: &str) -> Result<ParseOutcome> {
    enum State {
        Root,
        Links,
        Experience(ExperienceBuilder),
        Project(ProjectBuilder),
        Education(EducationBuilder),
        Skills(SkillsBuilder),
    }

    impl State {
        fn section_name(&self) -> &'static str {
            match self {
                State::Root => "root",
                State::Links => "@links",
                State::Experience(_) => "@experience",
                State::Project(_) => "@project",
                State::Education(_) => "@education",
                State::Skills(_) => "@skills",
            }
        }

        fn flush(
            self,
            experiences: &mut Vec<crate::resume::Experience>,
            projects: &mut Vec<crate::resume::Project>,
            educations: &mut Vec<crate::resume::Education>,
            skills: &mut Vec<crate::resume::Skills>,
        ) {
            match self {
                State::Skills(b) => skills.push(b.finish()),
                State::Experience(b) => experiences.push(b.finish()),
                State::Project(b) => projects.push(b.finish()),
                State::Education(b) => educations.push(b.finish()),
                State::Root | State::Links => {}
            }
        }
    }

    let mut state = State::Root;
    let mut warnings = Vec::new();
    let mut name = String::new();
    let mut email = String::new();
    let mut phone = None;
    let mut website = None;
    let mut links: Vec<Link> = Vec::new();
    let mut summary = String::new();
    let mut skills = Vec::new();
    let mut experiences = Vec::new();
    let mut projects = Vec::new();
    let mut educations = Vec::new();

    let mut lines = content
        .lines()
        .enumerate()
        .map(|(i, line)| (i + 1, line.trim()))
        .peekable();

    while let Some((line_no, line)) = lines.next() {
        if line.is_empty() {
            continue;
        }

        if let Some((directive, rest)) = Directive::from_line(line) {
            state.flush(
                &mut experiences,
                &mut projects,
                &mut educations,
                &mut skills,
            );

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
                    while let Some(&(_, next_line)) = lines.peek() {
                        if next_line.starts_with('@') {
                            break;
                        }
                        if let Some((_, line)) = lines.next()
                            && !line.is_empty()
                        {
                            summary_lines.push(line);
                        }
                    }
                    summary = summary_lines.join("\n");
                }
                Directive::Links => state = State::Links,
                Directive::Skills => state = State::Skills(SkillsBuilder::default()),
                Directive::Experience => state = State::Experience(ExperienceBuilder::default()),
                Directive::Project => state = State::Project(ProjectBuilder::default()),
                Directive::Education => state = State::Education(EducationBuilder::default()),
            }
        } else if line.starts_with('@') {
            // A directive-looking line that matched nothing is almost
            // certainly a typo; reset to Root so following lines are not
            // silently absorbed into the previous section.
            warnings.push(format!("line {}: unknown directive '{}'", line_no, line));
            std::mem::replace(&mut state, State::Root).flush(
                &mut experiences,
                &mut projects,
                &mut educations,
                &mut skills,
            );
        } else {
            let recognized = match &mut state {
                State::Root => false,
                State::Links => match line.split_once(':') {
                    Some((label, url)) if !label.trim().is_empty() && !url.trim().is_empty() => {
                        links.push(Link {
                            label: label.trim().to_string(),
                            url: url.trim().to_string(),
                        });
                        true
                    }
                    _ => false,
                },
                State::Experience(builder) => {
                    if let Some(val) = line.strip_prefix("title:") {
                        *builder = std::mem::take(builder).title(val.trim());
                        true
                    } else if let Some(val) = line.strip_prefix("company:") {
                        *builder = std::mem::take(builder).company(val.trim());
                        true
                    } else if let Some(val) = line.strip_prefix("date:") {
                        let (start, end) = parse_date_range(val.trim());
                        *builder = std::mem::take(builder).start(&start);
                        if let Some(end_date) = end {
                            *builder = std::mem::take(builder).end(&end_date);
                        }
                        true
                    } else if let Some(val) = line.strip_prefix("description:") {
                        *builder = std::mem::take(builder).description(val.trim());
                        true
                    } else if let Some(val) = line.strip_prefix('-') {
                        *builder = std::mem::take(builder).highlight(val.trim());
                        true
                    } else {
                        false
                    }
                }
                State::Project(builder) => {
                    if let Some(val) = line.strip_prefix("name:") {
                        *builder = std::mem::take(builder).name(val.trim());
                        true
                    } else if let Some(val) = line.strip_prefix("description:") {
                        *builder = std::mem::take(builder).description(val.trim());
                        true
                    } else if let Some(val) = line.strip_prefix("tech:") {
                        *builder = std::mem::take(builder).tech(parse_csv(val));
                        true
                    } else if let Some(val) = line.strip_prefix("link:") {
                        *builder = std::mem::take(builder).link(val.trim());
                        true
                    } else {
                        false
                    }
                }
                State::Education(builder) => {
                    if let Some(val) = line.strip_prefix("school:") {
                        *builder = std::mem::take(builder).school(val.trim());
                        true
                    } else if let Some(val) = line.strip_prefix("degree:") {
                        *builder = std::mem::take(builder).degree(val.trim());
                        true
                    } else if let Some(val) = line.strip_prefix("year:") {
                        *builder = std::mem::take(builder).year(val.trim());
                        true
                    } else if let Some(val) = line.strip_prefix("location:") {
                        *builder = std::mem::take(builder).location(val.trim());
                        true
                    } else {
                        false
                    }
                }
                // Any "Label: item, item, ..." line becomes a skill category,
                // so users can pick their own groupings (Languages, Technologies, ...).
                State::Skills(builder) => match line.split_once(':') {
                    Some((label, items)) if !label.trim().is_empty() => {
                        *builder = std::mem::take(builder).category(label.trim(), parse_csv(items));
                        true
                    }
                    _ => false,
                },
            };

            if !recognized {
                warnings.push(match state {
                    State::Root => format!(
                        "line {}: text outside of any section ignored: '{}'",
                        line_no, line
                    ),
                    _ => format!(
                        "line {}: unrecognized line in {} section ignored: '{}'",
                        line_no,
                        state.section_name(),
                        line
                    ),
                });
            }
        }
    }

    state.flush(
        &mut experiences,
        &mut projects,
        &mut educations,
        &mut skills,
    );

    if name.is_empty() {
        bail!("missing required '@name:' directive");
    }
    if email.is_empty() {
        warnings.push("no '@email:' directive found; contact line will be empty".to_string());
    }

    let mut builder = Resume::build().name(&name).email(&email);

    if let Some(p) = phone {
        builder = builder.phone(&p);
    }
    if let Some(w) = website {
        builder = builder.website(&w);
    }
    for link in links {
        builder = builder.link(&link.label, &link.url);
    }
    if !summary.is_empty() {
        builder = builder.summary(&summary);
    }

    for s in skills {
        builder = builder.merge_skills(s);
    }
    for exp in experiences {
        builder = builder.push_experience(exp);
    }
    for project in projects {
        builder = builder.push_project(project);
    }
    for edu in educations {
        builder = builder.push_education(edu);
    }

    Ok(ParseOutcome {
        resume: builder.finish(),
        warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_content, parse_csv, parse_date_range};

    #[test]
    fn parse_csv_trims_and_discards_empty_values() {
        assert_eq!(
            parse_csv("Rust,  Python ,, Go "),
            vec!["Rust", "Python", "Go"]
        );
    }

    #[test]
    fn parse_date_range_supports_single_value_and_ranges() {
        assert_eq!(
            parse_date_range("2020 - Present"),
            ("2020".to_string(), Some("Present".to_string()))
        );
        assert_eq!(parse_date_range("2020"), ("2020".to_string(), None));
    }

    #[test]
    fn parse_date_range_prefers_spaced_separator_over_inner_hyphens() {
        assert_eq!(
            parse_date_range("2020-09 - 2021-05"),
            ("2020-09".to_string(), Some("2021-05".to_string()))
        );
        assert_eq!(
            parse_date_range("2020-2023"),
            ("2020".to_string(), Some("2023".to_string()))
        );
    }

    #[test]
    fn parses_complete_resume_content() {
        let outcome = parse_content(
            r#"
@name: Jane Doe
@email: jane@example.com
@phone: +49-555-0100
@website: https://example.com

@links:
LinkedIn: https://linkedin.com/in/janedoe
GitHub: https://github.com/janedoe

@summary:
Builder-focused Rust engineer.
Writes maintainable tooling.

@skills:
Languages: Rust, Python
Technologies: Cargo, Git

@project:
name: speck
description: Lightweight React state library.
tech: TypeScript
link: https://github.com/janedoe/speck

@experience:
title: Staff Engineer
company: Example Co
date: 2022 - Present
description: Berlin, Germany
- Shipped internal developer tooling
- Reduced release friction

@education:
school: Example University
degree: BSc Computer Science
year: 2018
location: Leicester, UK
"#,
        )
        .expect("content should parse");

        let resume = &outcome.resume;
        assert!(outcome.warnings.is_empty(), "{:?}", outcome.warnings);
        assert_eq!(resume.name, "Jane Doe");
        assert_eq!(resume.email, "jane@example.com");
        assert_eq!(resume.phone.as_deref(), Some("+49-555-0100"));
        assert_eq!(resume.website.as_deref(), Some("https://example.com"));
        assert_eq!(resume.links.len(), 2);
        assert_eq!(resume.links[0].label, "LinkedIn");
        assert_eq!(resume.links[0].url, "https://linkedin.com/in/janedoe");
        assert_eq!(resume.links[1].label, "GitHub");
        assert_eq!(
            resume.summary.as_deref(),
            Some("Builder-focused Rust engineer.\nWrites maintainable tooling.")
        );
        assert_eq!(resume.skills.categories.len(), 2);
        assert_eq!(resume.skills.categories[0].label, "Languages");
        assert_eq!(resume.skills.categories[0].items, vec!["Rust", "Python"]);
        assert_eq!(resume.skills.categories[1].label, "Technologies");
        assert_eq!(resume.skills.categories[1].items, vec!["Cargo", "Git"]);
        assert_eq!(resume.projects.len(), 1);
        assert_eq!(resume.projects[0].name, "speck");
        assert_eq!(
            resume.projects[0].description,
            "Lightweight React state library."
        );
        assert_eq!(resume.projects[0].tech, vec!["TypeScript"]);
        assert_eq!(
            resume.projects[0].link.as_deref(),
            Some("https://github.com/janedoe/speck")
        );
        assert_eq!(resume.experience.len(), 1);
        assert_eq!(resume.experience[0].title, "Staff Engineer");
        assert_eq!(resume.experience[0].company, "Example Co");
        assert_eq!(resume.experience[0].start_date, "2022");
        assert_eq!(resume.experience[0].end_date.as_deref(), Some("Present"));
        assert_eq!(
            resume.experience[0].highlights,
            vec![
                "Shipped internal developer tooling",
                "Reduced release friction"
            ]
        );
        assert_eq!(resume.education.len(), 1);
        assert_eq!(resume.education[0].school, "Example University");
        assert_eq!(
            resume.education[0].location.as_deref(),
            Some("Leicester, UK")
        );
    }

    #[test]
    fn summary_stops_at_next_directive() {
        let outcome = parse_content(
            r#"
@name: Jane Doe
@email: jane@example.com
@summary:
First line
Second line
@skills:
Languages: Rust
"#,
        )
        .expect("content should parse");

        assert_eq!(
            outcome.resume.summary.as_deref(),
            Some("First line\nSecond line")
        );
        assert_eq!(outcome.resume.skills.categories[0].items, vec!["Rust"]);
    }

    #[test]
    fn skills_line_without_colon_warns() {
        let outcome = parse_content(
            r#"
@name: Jane Doe
@email: jane@example.com
@skills:
just some words
"#,
        )
        .expect("content should parse");

        assert_eq!(outcome.warnings.len(), 1);
        assert!(
            outcome.warnings[0].contains("line 5: unrecognized line in @skills section"),
            "{:?}",
            outcome.warnings
        );
    }

    #[test]
    fn unknown_directive_warns_and_resets_section() {
        let outcome = parse_content(
            r#"
@name: Jane Doe
@email: jane@example.com
@experience:
title: Engineer
company: Example Co
@experiance:
title: Should not merge into previous section
"#,
        )
        .expect("content should parse");

        assert_eq!(outcome.resume.experience.len(), 1);
        assert_eq!(outcome.resume.experience[0].title, "Engineer");
        assert_eq!(outcome.warnings.len(), 2);
        assert!(
            outcome.warnings[0].contains("line 7: unknown directive '@experiance:'"),
            "{:?}",
            outcome.warnings
        );
        assert!(
            outcome.warnings[1].contains("line 8: text outside of any section"),
            "{:?}",
            outcome.warnings
        );
    }

    #[test]
    fn unrecognized_section_key_warns() {
        let outcome = parse_content(
            r#"
@name: Jane Doe
@email: jane@example.com
@education:
school: Example University
degre: BSc Computer Science
"#,
        )
        .expect("content should parse");

        assert_eq!(outcome.warnings.len(), 1);
        assert!(
            outcome.warnings[0].contains("line 6: unrecognized line in @education section"),
            "{:?}",
            outcome.warnings
        );
    }

    #[test]
    fn missing_name_is_an_error() {
        let err = parse_content("@email: jane@example.com").expect_err("should fail");
        assert!(err.to_string().contains("@name"));
    }

    #[test]
    fn missing_email_only_warns() {
        let outcome = parse_content("@name: Jane Doe").expect("content should parse");
        assert!(outcome.warnings.iter().any(|w| w.contains("@email")));
    }
}
