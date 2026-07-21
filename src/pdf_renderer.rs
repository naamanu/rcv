use crate::resume::{Education, Experience, Project, Resume};
use anyhow::{Context as AnyhowContext, Result};
use genpdf::error::Error as GenpdfError;
use genpdf::{
    Context, Element, Position, RenderResult, SimplePageDecorator, Size, elements, render, style,
};

const BODY_SIZE: u8 = 10;

// CMU Serif (Computer Modern Unicode), the LaTeX look, bundled so output is
// identical on every machine. SIL OFL licensed; see fonts/LICENSE-OFL.txt.
// Only TTFs work here: genpdf's rusttype backend cannot parse CFF-based OTFs.
const FONT_REGULAR: &[u8] = include_bytes!("../fonts/cmunrm.ttf");
const FONT_BOLD: &[u8] = include_bytes!("../fonts/cmunbx.ttf");
const FONT_ITALIC: &[u8] = include_bytes!("../fonts/cmunti.ttf");
const FONT_BOLD_ITALIC: &[u8] = include_bytes!("../fonts/cmunbi.ttf");

fn load_font_data(data: &[u8]) -> Result<genpdf::fonts::FontData> {
    genpdf::fonts::FontData::new(data.to_vec(), None)
        .map_err(|e| anyhow::anyhow!("Failed to parse bundled font data: {}", e))
}

/// A thin horizontal rule spanning the full width, used under section titles
/// to mimic the LaTeX \titlerule.
struct HLine;

impl Element for HLine {
    fn render(
        &mut self,
        _context: &Context,
        area: render::Area<'_>,
        _style: style::Style,
    ) -> Result<RenderResult, GenpdfError> {
        let width = area.size().width;
        area.draw_line(
            vec![Position::new(0, 0.4), Position::new(width, 0.4)],
            style::Style::new(),
        );
        Ok(RenderResult {
            size: Size::new(width, 1.0),
            has_more: false,
        })
    }
}

pub fn export_to_pdf(resume: &Resume, output_file: &str) -> Result<()> {
    eprintln!("Loading fonts...");

    let font_family = genpdf::fonts::FontFamily {
        regular: load_font_data(FONT_REGULAR)?,
        bold: load_font_data(FONT_BOLD)?,
        italic: load_font_data(FONT_ITALIC)?,
        bold_italic: load_font_data(FONT_BOLD_ITALIC)?,
    };

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(format!("Resume - {}", resume.name));
    // CMU Serif carries generous line metrics; tighten toward LaTeX's baseline grid.
    doc.set_line_spacing(0.85);

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(12);
    doc.set_page_decorator(decorator);

    push_header(&mut doc, resume);

    // --- Summary ---
    if let Some(summary) = &resume.summary {
        push_section_header(&mut doc, "Summary");
        doc.push(
            elements::Paragraph::new(summary).styled(style::Style::new().with_font_size(BODY_SIZE)),
        );
    }

    // --- Education ---
    if !resume.education.is_empty() {
        push_section_header(&mut doc, "Education");
        for edu in &resume.education {
            push_education(&mut doc, edu);
        }
    }

    // --- Experience ---
    if !resume.experience.is_empty() {
        push_section_header(&mut doc, "Experience");
        for exp in &resume.experience {
            push_experience(&mut doc, exp);
        }
    }

    // --- Projects ---
    if !resume.projects.is_empty() {
        push_section_header(&mut doc, "Projects");
        for project in &resume.projects {
            push_project(&mut doc, project);
        }
    }

    // --- Skills ---
    if !resume.skills.is_empty() {
        push_section_header(&mut doc, "Skills");
        for category in &resume.skills.categories {
            let mut para = elements::Paragraph::new("");
            para.push_styled(
                format!("{}: ", category.label),
                style::Style::new().bold().with_font_size(BODY_SIZE),
            );
            para.push_styled(
                category.items.join(", "),
                style::Style::new().with_font_size(BODY_SIZE),
            );
            doc.push(para);
        }
    }

    eprintln!("Rendering PDF to {}...", output_file);
    doc.render_to_file(output_file)
        .context("Failed to render PDF")?;

    Ok(())
}

/// Header in the LaTeX template's layout: bold name left with email right,
/// then website left with the remaining links right.
fn push_header(doc: &mut genpdf::Document, resume: &Resume) {
    let mut table = elements::TableLayout::new(vec![1, 1]);

    let name = elements::Paragraph::new(&resume.name)
        .styled(style::Style::new().bold().with_font_size(16));
    let contact = elements::Paragraph::new(build_contact_text(resume))
        .aligned(genpdf::Alignment::Right)
        .styled(style::Style::new().with_font_size(BODY_SIZE));
    table
        .row()
        .element(name)
        .element(contact)
        .push()
        .expect("header row should have two cells");

    let website = resume.website.as_deref().unwrap_or("");
    let links = resume
        .links
        .iter()
        .map(|link| shorten_url(&link.url))
        .collect::<Vec<_>>()
        .join(" — ");
    if !website.is_empty() || !links.is_empty() {
        let left = elements::Paragraph::new(shorten_url(website))
            .styled(style::Style::new().with_font_size(BODY_SIZE));
        let right = elements::Paragraph::new(links)
            .aligned(genpdf::Alignment::Right)
            .styled(style::Style::new().with_font_size(BODY_SIZE));
        table
            .row()
            .element(left)
            .element(right)
            .push()
            .expect("header row should have two cells");
    }

    doc.push(table);
}

/// Section title over a horizontal rule, like the LaTeX \section formatting.
fn push_section_header(doc: &mut genpdf::Document, text: &str) {
    doc.push(elements::Break::new(0.5));
    doc.push(elements::Paragraph::new(text).styled(style::Style::new().bold().with_font_size(12)));
    doc.push(HLine);
    doc.push(elements::Break::new(0.1));
}

/// A two-line entry heading: bold primary left with plain right column,
/// then an italic secondary line, matching \resumeSubheading.
fn push_subheading(doc: &mut genpdf::Document, top: (&str, &str), bottom: (&str, &str)) {
    let mut table = elements::TableLayout::new(vec![1, 1]);

    let top_style = style::Style::new().bold().with_font_size(11);
    let bottom_style = style::Style::new().italic().with_font_size(BODY_SIZE);

    table
        .row()
        .element(elements::Paragraph::new(top.0).styled(top_style))
        .element(
            elements::Paragraph::new(top.1)
                .aligned(genpdf::Alignment::Right)
                .styled(style::Style::new().with_font_size(BODY_SIZE)),
        )
        .push()
        .expect("subheading row should have two cells");
    table
        .row()
        .element(elements::Paragraph::new(bottom.0).styled(bottom_style))
        .element(
            elements::Paragraph::new(bottom.1)
                .aligned(genpdf::Alignment::Right)
                .styled(bottom_style),
        )
        .push()
        .expect("subheading row should have two cells");

    doc.push(table);
}

fn push_experience(doc: &mut genpdf::Document, exp: &Experience) {
    push_subheading(
        doc,
        (&exp.company, exp.description.as_deref().unwrap_or("")),
        (&exp.title, &format_experience_dates(exp)),
    );

    if !exp.highlights.is_empty() {
        let mut list = elements::UnorderedList::with_bullet("◦");
        for highlight in &exp.highlights {
            list.push(
                elements::Paragraph::new(highlight)
                    .styled(style::Style::new().with_font_size(BODY_SIZE)),
            );
        }
        doc.push(list);
    }
    doc.push(elements::Break::new(0.3));
}

fn push_education(doc: &mut genpdf::Document, edu: &Education) {
    push_subheading(
        doc,
        (&edu.school, edu.location.as_deref().unwrap_or("")),
        (&edu.degree, &edu.year),
    );
    doc.push(elements::Break::new(0.1));
}

fn push_project(doc: &mut genpdf::Document, project: &Project) {
    // The link column must be wide enough for an unbreakable URL;
    // genpdf drops words that do not fit a column.
    let mut table = elements::TableLayout::new(vec![7, 3]);

    let mut para = elements::Paragraph::new("");
    para.push_styled(
        format!("{}: ", project.name),
        style::Style::new().bold().with_font_size(BODY_SIZE),
    );
    para.push_styled(
        project.description.clone(),
        style::Style::new().with_font_size(BODY_SIZE),
    );
    if !project.tech.is_empty() {
        para.push_styled(
            format!(" ({})", project.tech.join(", ")),
            style::Style::new().bold().with_font_size(BODY_SIZE),
        );
    }

    let link =
        elements::Paragraph::new(project.link.as_deref().map(shorten_url).unwrap_or_default())
            .aligned(genpdf::Alignment::Right)
            .styled(
                style::Style::new()
                    .with_color(style::Color::Rgb(100, 100, 100))
                    .with_font_size(8),
            );

    table
        .row()
        .element(para)
        .element(link)
        .push()
        .expect("project row should have two cells");
    doc.push(table);
    doc.push(elements::Break::new(0.2));
}

fn build_contact_text(resume: &Resume) -> String {
    [Some(resume.email.as_str()), resume.phone.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" | ")
}

/// Trims scheme and trailing slash so links stay compact in print,
/// e.g. "https://github.com/naamanu/" -> "github.com/naamanu".
fn shorten_url(url: &str) -> String {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_string()
}

fn format_experience_dates(experience: &Experience) -> String {
    match experience.end_date.as_deref() {
        Some(end) => format!("{} - {}", experience.start_date, end),
        None => experience.start_date.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::{build_contact_text, format_experience_dates, shorten_url};
    use crate::resume::Resume;

    #[test]
    fn build_contact_text_skips_missing_optional_fields() {
        let resume = Resume {
            email: "jane@example.com".to_string(),
            phone: Some("+49-555-0100".to_string()),
            ..Default::default()
        };

        assert_eq!(
            build_contact_text(&resume),
            "jane@example.com | +49-555-0100"
        );

        let email_only = Resume {
            email: "jane@example.com".to_string(),
            ..Default::default()
        };
        assert_eq!(build_contact_text(&email_only), "jane@example.com");
    }

    #[test]
    fn shorten_url_strips_scheme_and_trailing_slash() {
        assert_eq!(
            shorten_url("https://github.com/naamanu/"),
            "github.com/naamanu"
        );
        assert_eq!(shorten_url("http://example.com"), "example.com");
        assert_eq!(shorten_url("example.com"), "example.com");
    }

    #[test]
    fn format_experience_dates_uses_end_date_when_present() {
        let experience = crate::resume::Experience {
            start_date: "2022".to_string(),
            end_date: Some("Present".to_string()),
            ..Default::default()
        };

        assert_eq!(format_experience_dates(&experience), "2022 - Present");
    }

    #[test]
    fn format_experience_dates_returns_start_date_when_open_ended() {
        let experience = crate::resume::Experience {
            start_date: "2022".to_string(),
            ..Default::default()
        };

        assert_eq!(format_experience_dates(&experience), "2022");
    }
}
