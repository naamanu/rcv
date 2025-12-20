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
    pub summary: Option<String>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub skills: Vec<String>,
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

/// Represents an educational background.
#[derive(Debug, Default, Clone)]
pub struct Education {
    pub school: String,
    pub degree: String,
    pub year: String,
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

    /// Sets the professional summary or objective.
    pub fn summary(mut self, text: &str) -> Self {
        self.resume.summary = Some(text.to_string());
        self
    }

    /// Adds a skill to the resume.
    pub fn skill(mut self, skill: &str) -> Self {
        self.resume.skills.push(skill.to_string());
        self
    }

    /// Adds multiple skills at once.
    pub fn skills(mut self, skills: &[&str]) -> Self {
        for s in skills {
            self.resume.skills.push(s.to_string());
        }
        self
    }

    /// Adds an experience section using a closure to build the Experience object.
    /// This allows for a nested DSL structure.
    pub fn experience<F>(mut self, build: F) -> Self
    where
        F: FnOnce(ExperienceBuilder) -> ExperienceBuilder,
    {
        let builder = ExperienceBuilder::default();
        let exp = build(builder).finish();
        self.resume.experience.push(exp);
        self
    }

    /// Adds an education section using a closure.
    pub fn education<F>(mut self, build: F) -> Self
    where
        F: FnOnce(EducationBuilder) -> EducationBuilder,
    {
        let builder = EducationBuilder::default();
        let edu = build(builder).finish();
        self.resume.education.push(edu);
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

    pub fn current(mut self) -> Self {
        self.experience.end_date = Some("Present".to_string());
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

    pub fn finish(self) -> Education {
        self.education
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
        writeln!(f, "\n")?;

        // Summary
        if let Some(summary) = &self.summary {
            writeln!(f, "## Summary")?;
            writeln!(f, "{}\n", summary)?;
        }

        // Skills
        if !self.skills.is_empty() {
            writeln!(f, "## Skills")?;
            writeln!(f, "{}\n", self.skills.join(", "))?;
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

        // Education
        if !self.education.is_empty() {
            writeln!(f, "## Education")?;
            for edu in &self.education {
                writeln!(f, "**{}**, {} ({})", edu.school, edu.degree, edu.year)?;
            }
        }

        Ok(())
    }
}
