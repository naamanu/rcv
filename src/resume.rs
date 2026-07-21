/// The main module for our Resume DSL.
/// This module contains the data structures and the builder pattern implementation
/// that creates the "Domain Specific Language" feel for creating resumes in Rust.
use std::fmt;

/// Represents the entire resume.
#[derive(Debug, Default, Clone)]
pub struct Resume {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub links: Vec<Link>,
    pub summary: Option<String>,
    pub experience: Vec<Experience>,
    pub projects: Vec<Project>,
    pub education: Vec<Education>,
    pub skills: Skills,
}

/// A labelled external link, e.g. "LinkedIn: https://linkedin.com/in/...".
#[derive(Debug, Default, Clone)]
pub struct Link {
    pub label: String,
    pub url: String,
}

/// Represents a set of skills as ordered, user-defined categories,
/// e.g. "Languages: Rust, Go" or "Technologies: AWS, React".
#[derive(Debug, Default, Clone)]
pub struct Skills {
    pub categories: Vec<SkillCategory>,
}

/// A single skill category with its comma-separated items.
#[derive(Debug, Default, Clone)]
pub struct SkillCategory {
    pub label: String,
    pub items: Vec<String>,
}

impl Skills {
    pub fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }

    /// Merges another skill set, appending items into a same-named category
    /// when one exists (case-insensitive) and adding new categories otherwise.
    pub fn merge(&mut self, other: Skills) {
        for category in other.categories {
            match self
                .categories
                .iter_mut()
                .find(|c| c.label.eq_ignore_ascii_case(&category.label))
            {
                Some(existing) => existing.items.extend(category.items),
                None => self.categories.push(category),
            }
        }
    }
}

/// Represents a single job experience.
#[derive(Debug, Default, Clone)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub description: Option<String>,
    pub highlights: Vec<String>,
}

/// Represents a personal or open-source project.
#[derive(Debug, Default, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub tech: Vec<String>,
    pub link: Option<String>,
}

/// Represents an educational background.
#[derive(Debug, Default, Clone)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub year: String,
    pub location: Option<String>,
}

// --- Builder Implementation (The DSL) ---

impl Resume {
    /// Starts the building process.
    pub fn build() -> ResumeBuilder {
        ResumeBuilder::default()
    }
}

/// A builder struct for the Resume.
#[derive(Default)]
pub struct ResumeBuilder {
    resume: Resume,
}

impl ResumeBuilder {
    /// Sets the name of the candidate.
    pub fn name(mut self, name: &str) -> Self {
        self.resume.name = name.to_string();
        self
    }

    /// Sets the email of the candidate.
    pub fn email(mut self, email: &str) -> Self {
        self.resume.email = email.to_string();
        self
    }

    /// Sets the phone number (optional).
    pub fn phone(mut self, phone: &str) -> Self {
        self.resume.phone = Some(phone.to_string());
        self
    }

    /// Sets a personal website or portfolio URL (optional).
    pub fn website(mut self, site: &str) -> Self {
        self.resume.website = Some(site.to_string());
        self
    }

    /// Adds a labelled link such as LinkedIn or GitHub.
    pub fn link(mut self, label: &str, url: &str) -> Self {
        self.resume.links.push(Link {
            label: label.to_string(),
            url: url.to_string(),
        });
        self
    }

    /// Sets the professional summary or objective.
    pub fn summary(mut self, text: &str) -> Self {
        self.resume.summary = Some(text.to_string());
        self
    }

    /// Adds an experience section using a closure to build the Experience object.
    /// This allows for a nested DSL structure.
    /// Not called by the CLI itself, but kept as part of the Rust-facing DSL.
    #[allow(dead_code)]
    pub fn experience<F>(self, build: F) -> Self
    where
        F: FnOnce(ExperienceBuilder) -> ExperienceBuilder,
    {
        self.push_experience(build(ExperienceBuilder::default()).finish())
    }

    /// Adds an education section using a closure.
    /// Not called by the CLI itself, but kept as part of the Rust-facing DSL.
    #[allow(dead_code)]
    pub fn education<F>(self, build: F) -> Self
    where
        F: FnOnce(EducationBuilder) -> EducationBuilder,
    {
        self.push_education(build(EducationBuilder::default()).finish())
    }

    /// Adds an already-built Experience, e.g. one produced by the parser.
    pub fn push_experience(mut self, exp: Experience) -> Self {
        self.resume.experience.push(exp);
        self
    }

    /// Adds an already-built Project, e.g. one produced by the parser.
    pub fn push_project(mut self, project: Project) -> Self {
        self.resume.projects.push(project);
        self
    }

    /// Adds an already-built Education, e.g. one produced by the parser.
    pub fn push_education(mut self, edu: Education) -> Self {
        self.resume.education.push(edu);
        self
    }

    pub fn merge_skills(mut self, skills: Skills) -> Self {
        self.resume.skills.merge(skills);
        self
    }

    /// Finalizes the build and returns the Resume struct.
    pub fn finish(self) -> Resume {
        self.resume
    }
}

/// A builder struct for Experience.
#[derive(Default)]
pub struct ExperienceBuilder {
    experience: Experience,
}

impl ExperienceBuilder {
    pub fn title(mut self, title: &str) -> Self {
        self.experience.title = title.to_string();
        self
    }

    pub fn company(mut self, company: &str) -> Self {
        self.experience.company = company.to_string();
        self
    }

    pub fn start(mut self, date: &str) -> Self {
        self.experience.start_date = date.to_string();
        self
    }

    pub fn end(mut self, date: &str) -> Self {
        self.experience.end_date = Some(date.to_string());
        self
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.experience.description = Some(desc.to_string());
        self
    }

    pub fn highlight(mut self, text: &str) -> Self {
        self.experience.highlights.push(text.to_string());
        self
    }

    pub fn finish(self) -> Experience {
        self.experience
    }
}

/// A builder struct for Project.
#[derive(Default)]
pub struct ProjectBuilder {
    project: Project,
}

impl ProjectBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.project.name = name.to_string();
        self
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.project.description = desc.to_string();
        self
    }

    pub fn tech(mut self, tech: Vec<String>) -> Self {
        self.project.tech = tech;
        self
    }

    pub fn link(mut self, url: &str) -> Self {
        self.project.link = Some(url.to_string());
        self
    }

    pub fn finish(self) -> Project {
        self.project
    }
}

/// A builder struct for Education.
#[derive(Default)]
pub struct EducationBuilder {
    education: Education,
}

impl EducationBuilder {
    pub fn school(mut self, school: &str) -> Self {
        self.education.school = school.to_string();
        self
    }

    pub fn degree(mut self, degree: &str) -> Self {
        self.education.degree = degree.to_string();
        self
    }

    pub fn year(mut self, year: &str) -> Self {
        self.education.year = year.to_string();
        self
    }

    pub fn location(mut self, location: &str) -> Self {
        self.education.location = Some(location.to_string());
        self
    }

    pub fn finish(self) -> Education {
        self.education
    }
}

/// A builder struct for Skills
#[derive(Default)]
pub struct SkillsBuilder {
    skills: Skills,
}

impl SkillsBuilder {
    pub fn category(mut self, label: &str, items: Vec<String>) -> Self {
        self.skills.categories.push(SkillCategory {
            label: label.to_string(),
            items,
        });
        self
    }

    pub fn finish(self) -> Skills {
        self.skills
    }
}

// --- Output Formatting ---

impl fmt::Display for Resume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# {}\n", self.name)?;

        // Contact Info
        write!(f, "**Email:** {}", self.email)?;
        if let Some(phone) = &self.phone {
            write!(f, " | **Phone:** {}", phone)?;
        }
        if let Some(web) = &self.website {
            write!(f, " | **Web:** {}", web)?;
        }
        for link in &self.links {
            write!(f, " | [{}]({})", link.label, link.url)?;
        }
        writeln!(f, "\n")?;

        // Summary
        if let Some(summary) = &self.summary {
            writeln!(f, "## Summary")?;
            writeln!(f, "{}\n", summary)?;
        }

        // Education
        if !self.education.is_empty() {
            writeln!(f, "## Education")?;
            for edu in &self.education {
                write!(f, "**{}**, {} ({})", edu.school, edu.degree, edu.year)?;
                match &edu.location {
                    Some(location) => writeln!(f, " — {}", location)?,
                    None => writeln!(f)?,
                }
            }
            writeln!(f)?;
        }

        // Experience
        if !self.experience.is_empty() {
            writeln!(f, "## Experience")?;
            for exp in &self.experience {
                write!(f, "### {} @ {}", exp.title, exp.company)?;
                write!(f, " ({})", exp.start_date)?;
                if let Some(end) = &exp.end_date {
                    writeln!(f, " - {}", end)?;
                } else {
                    writeln!(f)?;
                }

                if let Some(desc) = &exp.description {
                    writeln!(f, "\n_{}_\n", desc)?;
                }

                for highlight in &exp.highlights {
                    writeln!(f, "- {}", highlight)?;
                }
                writeln!(f)?;
            }
        }

        // Projects
        if !self.projects.is_empty() {
            writeln!(f, "## Projects")?;
            for project in &self.projects {
                match &project.link {
                    Some(link) => write!(f, "- **[{}]({})**", project.name, link)?,
                    None => write!(f, "- **{}**", project.name)?,
                }
                if !project.tech.is_empty() {
                    write!(f, " _({})_", project.tech.join(", "))?;
                }
                writeln!(f, ": {}", project.description)?;
            }
            writeln!(f)?;
        }

        // Skills
        if !self.skills.is_empty() {
            writeln!(f, "## Skills")?;
            for category in &self.skills.categories {
                writeln!(f, "#### {}", category.label)?;
                writeln!(f, "{}\n", category.items.join(", "))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{EducationBuilder, ExperienceBuilder, ProjectBuilder, Resume, SkillsBuilder};

    #[test]
    fn display_renders_markdown_sections_for_populated_resume() {
        let resume = Resume::build()
            .name("Jane Doe")
            .email("jane@example.com")
            .phone("+49-555-0100")
            .website("https://example.com")
            .link("GitHub", "https://github.com/janedoe")
            .summary("Builds reliable developer tools.")
            .merge_skills(
                SkillsBuilder::default()
                    .category("Languages", vec!["Rust".to_string()])
                    .category("Technologies", vec!["AWS".to_string()])
                    .finish(),
            )
            .experience(|b| {
                b.title("Staff Engineer")
                    .company("Example Co")
                    .start("2022")
                    .end("Present")
                    .description("Berlin, Germany")
                    .highlight("Reduced release friction")
            })
            .push_project(
                ProjectBuilder::default()
                    .name("speck")
                    .description("Lightweight React state library.")
                    .tech(vec!["TypeScript".to_string()])
                    .link("https://github.com/janedoe/speck")
                    .finish(),
            )
            .education(|b| {
                b.school("Example University")
                    .degree("BSc Computer Science")
                    .year("2018")
                    .location("Leicester, UK")
            })
            .finish();

        let rendered = resume.to_string();

        assert!(rendered.contains("# Jane Doe"));
        assert!(rendered.contains(
            "**Email:** jane@example.com | **Phone:** +49-555-0100 | **Web:** https://example.com | [GitHub](https://github.com/janedoe)"
        ));
        assert!(rendered.contains("## Summary"));
        assert!(rendered.contains("#### Languages"));
        assert!(rendered.contains("#### Technologies"));
        assert!(rendered.contains("### Staff Engineer @ Example Co (2022) - Present"));
        assert!(rendered.contains("- Reduced release friction"));
        assert!(rendered.contains(
            "- **[speck](https://github.com/janedoe/speck)** _(TypeScript)_: Lightweight React state library."
        ));
        assert!(
            rendered
                .contains("**Example University**, BSc Computer Science (2018) — Leicester, UK")
        );
    }

    #[test]
    fn skills_merge_extends_matching_categories_case_insensitively() {
        let mut skills = SkillsBuilder::default()
            .category("Languages", vec!["Rust".to_string()])
            .finish();
        skills.merge(
            SkillsBuilder::default()
                .category("languages", vec!["Go".to_string()])
                .category("Technologies", vec!["AWS".to_string()])
                .finish(),
        );

        assert_eq!(skills.categories.len(), 2);
        assert_eq!(skills.categories[0].items, vec!["Rust", "Go"]);
        assert_eq!(skills.categories[1].label, "Technologies");
    }

    #[test]
    fn education_builder_defaults_location_to_none() {
        let edu = EducationBuilder::default().school("X").finish();
        assert!(edu.location.is_none());
        let _ = ExperienceBuilder::default();
    }
}
