use crate::resume::Resume;
use anyhow::{Context, Result};
use font_kit::family_name::FamilyName;
use font_kit::properties::{Properties, Style as FontStyle, Weight};
use font_kit::source::SystemSource;
use genpdf::elements;
use genpdf::{Element, SimplePageDecorator, style};

fn load_font_data(family_name: &str, weight: Weight, style: FontStyle) -> Result<genpdf::fonts::FontData> {
    let source = SystemSource::new();
    let mut props = Properties::new();
    props.weight(weight);
    props.style(style);

    let family = FamilyName::Title(family_name.to_string());

    let handle = source
        .select_best_match(&[family, FamilyName::SansSerif], &props)
        .context("Failed to find a matching system font (Arial or SansSerif)")?;

    let font = handle.load()?;
    let data = font
        .copy_font_data()
        .context("Font data is not available")?;

    genpdf::fonts::FontData::new((*data).clone(), None)
        .map_err(|e| anyhow::anyhow!("Failed to parse font data: {}", e))
}

pub fn export_to_pdf(resume: &Resume, output_file: &str) -> Result<()> {
    println!("Loading system fonts...");

    let font_family = genpdf::fonts::FontFamily {
        regular: load_font_data("Arial", Weight::NORMAL, FontStyle::Normal)?,
        bold: load_font_data("Arial", Weight::BOLD, FontStyle::Normal)?,
        italic: load_font_data("Arial", Weight::NORMAL, FontStyle::Italic)?,
        bold_italic: load_font_data("Arial", Weight::BOLD, FontStyle::Italic)?,
    };

    // 2. Create the document
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(format!("Resume - {}", resume.name));

    // Customize the page layout with narrower margins for more content width
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(20);
    doc.set_page_decorator(decorator);

    // --- Header ---
    // Name
    let mut title_para = elements::Paragraph::new(&resume.name);
    title_para.set_alignment(genpdf::Alignment::Center);
    doc.push(title_para.styled(style::Style::new().bold().with_font_size(24)));

    // Contact Info
    let contact_text = [
        Some(&resume.email),
        resume.phone.as_ref(),
        resume.website.as_ref(),
    ]
    .iter()
    .flatten()
    .map(|s| s.as_str())
    .collect::<Vec<_>>()
    .join(" | ");
    let mut contact_para = elements::Paragraph::new(contact_text);
    contact_para.set_alignment(genpdf::Alignment::Center);
    doc.push(contact_para.styled(style::Style::new().with_font_size(10)));

    // --- Summary ---
    if let Some(summary) = &resume.summary {
        doc.push(section_header("Summary"));
        doc.push(
            elements::Paragraph::new(summary)
                .styled(style::Style::new().with_font_size(10)),
        );
    }

    // --- Skills ---
    if !resume.skills.is_empty() {
        doc.push(section_header("Skills"));

        let mut skill_parts = Vec::new();
        if !resume.skills.languages.is_empty() {
            skill_parts.push(format!("Languages: {}", resume.skills.languages.join(", ")));
        }
        if !resume.skills.frameworks.is_empty() {
            skill_parts.push(format!("Frameworks: {}", resume.skills.frameworks.join(", ")));
        }
        if !resume.skills.tools.is_empty() {
            skill_parts.push(format!("Tools: {}", resume.skills.tools.join(", ")));
        }

        if !skill_parts.is_empty() {
            doc.push(
                elements::Paragraph::new(skill_parts.join(" | "))
                    .styled(style::Style::new().with_font_size(10)),
            );
        }
    }

    // --- Experience ---
    if !resume.experience.is_empty() {
        doc.push(section_header("Experience"));

        for exp in &resume.experience {
            // Title and company
            let title_text = format!("{}, {}", exp.title, exp.company);
            doc.push(
                elements::Paragraph::new(title_text)
                    .styled(style::Style::new().bold().with_font_size(10)),
            );

            // Date and location on same line
            let date_text = match &exp.end_date {
                Some(end) => format!("{} - {}", exp.start_date, end),
                None => exp.start_date.clone(),
            };
            let meta_text = match &exp.description {
                Some(location) => format!("{} | {}", location, date_text),
                None => date_text,
            };
            doc.push(
                elements::Paragraph::new(meta_text).styled(
                    style::Style::new()
                        .italic()
                        .with_color(style::Color::Rgb(80, 80, 80))
                        .with_font_size(10),
                ),
            );

            // Bullet points
            if !exp.highlights.is_empty() {
                let mut list = elements::UnorderedList::new();
                for highlight in &exp.highlights {
                    list.push(
                        elements::Paragraph::new(highlight)
                            .styled(style::Style::new().with_font_size(10)),
                    );
                }
                doc.push(list);
            }

            doc.push(elements::Break::new(0.5));
        }
    }

    // --- Education ---
    if !resume.education.is_empty() {
        doc.push(section_header("Education"));
        for edu in &resume.education {
            // Degree
            doc.push(
                elements::Paragraph::new(&edu.degree)
                    .styled(style::Style::new().bold().with_font_size(10)),
            );
            // School and year
            let meta_text = format!("{} | {}", edu.school, edu.year);
            doc.push(
                elements::Paragraph::new(meta_text).styled(
                    style::Style::new()
                        .italic()
                        .with_color(style::Color::Rgb(80, 80, 80))
                        .with_font_size(10),
                ),
            );
            doc.push(elements::Break::new(0.5));
        }
    }

    // --- Notable Projects ---
    if !resume.projects.is_empty() {
        doc.push(section_header("Notable Projects"));
        for proj in &resume.projects {
            // Project name (bold)
            doc.push(
                elements::Paragraph::new(&proj.name)
                    .styled(style::Style::new().bold().with_font_size(10)),
            );
            // Description
            doc.push(
                elements::Paragraph::new(&proj.description)
                    .styled(style::Style::new().with_font_size(10)),
            );
            doc.push(elements::Break::new(0.5));
        }
    }

    // Render
    println!("Rendering PDF to {}...", output_file);
    doc.render_to_file(output_file)
        .context("Failed to render PDF")?;

    Ok(())
}

fn section_header(text: &str) -> elements::LinearLayout {
    let mut layout = elements::LinearLayout::vertical();
    layout.push(elements::Break::new(1.0));
    layout.push(
        elements::Paragraph::new(text.to_uppercase())
            .styled(style::Style::new().bold().with_font_size(11)),
    );
    layout.push(elements::Break::new(0.5));
    layout
}
