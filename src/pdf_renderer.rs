use crate::resume::Resume;
use anyhow::Context;
use font_kit::family_name::FamilyName;
use font_kit::properties::{Properties, Style as FontStyle, Weight};
use font_kit::source::SystemSource;
use genpdf::elements;
use genpdf::{Element, SimplePageDecorator, style};

fn load_font_data(
    family_name: &str,
    bold: bool,
    italic: bool,
) -> anyhow::Result<genpdf::fonts::FontData> {
    let source = SystemSource::new();
    let mut props = Properties::new();
    props.weight(if bold { Weight::BOLD } else { Weight::NORMAL });
    props.style(if italic {
        FontStyle::Italic
    } else {
        FontStyle::Normal
    });

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

pub fn export_to_pdf(resume: &Resume, output_file: &str) -> anyhow::Result<()> {
    // 1. Load fonts from system
    println!("Loading system fonts...");
    let regular = load_font_data("Arial", false, false)?;
    let bold = load_font_data("Arial", true, false)?;
    let italic = load_font_data("Arial", false, true)?;
    let bold_italic = load_font_data("Arial", true, true)?;

    let font_family = genpdf::fonts::FontFamily {
        regular,
        bold,
        italic,
        bold_italic,
    };

    // 2. Create the document
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(format!("Resume - {}", resume.name));

    // Customize the page layout
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(12);
    doc.set_page_decorator(decorator);

    // --- Header ---
    // Name
    let mut title_para = elements::Paragraph::new(&resume.name);
    title_para.set_alignment(genpdf::Alignment::Center);
    doc.push(title_para.styled(style::Style::new().bold().with_font_size(24)));

    // Contact Info
    let mut contact_parts = vec![resume.email.clone()];
    if let Some(phone) = &resume.phone {
        contact_parts.push(phone.clone());
    }
    if let Some(website) = &resume.website {
        contact_parts.push(website.clone());
    }

    let contact_text = contact_parts.join(" | ");
    let mut contact_para = elements::Paragraph::new(contact_text);
    contact_para.set_alignment(genpdf::Alignment::Center);
    doc.push(contact_para.styled(style::Style::new().with_font_size(10)));

    doc.push(elements::Break::new(2.0));

    // --- Summary ---
    if let Some(summary) = &resume.summary {
        doc.push(section_header("Summary"));
        doc.push(elements::Paragraph::new(summary));
        doc.push(elements::Break::new(1.0));
    }

    // --- Skills ---
    if !resume.skills.is_empty() {
        doc.push(section_header("Skills"));
        let skills_text = resume.skills.join(", ");
        doc.push(elements::Paragraph::new(skills_text));
        doc.push(elements::Break::new(1.0));
    }

    // --- Experience ---
    if !resume.experience.is_empty() {
        doc.push(section_header("Experience"));

        for exp in &resume.experience {
            let title_text = format!("{} @ {}", exp.title, exp.company);
            let title_para = elements::Paragraph::new(title_text);
            doc.push(title_para.styled(style::Style::new().bold().with_font_size(11)));

            let mut date_text = format!("{}", exp.start_date);
            if let Some(end) = &exp.end_date {
                date_text.push_str(&format!(" - {}", end));
            }
            let date_para = elements::Paragraph::new(date_text);
            doc.push(
                date_para.styled(
                    style::Style::new()
                        .italic()
                        .with_color(style::Color::Rgb(100, 100, 100))
                        .with_font_size(10),
                ),
            );

            if let Some(desc) = &exp.description {
                let desc_para = elements::Paragraph::new(desc);
                doc.push(desc_para.styled(style::Style::new().with_font_size(10)));
            }

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

            doc.push(elements::Break::new(1.0));
        }
    }

    // --- Education ---
    if !resume.education.is_empty() {
        doc.push(section_header("Education"));
        for edu in &resume.education {
            let text = format!("{}, {} ({})", edu.school, edu.degree, edu.year);
            doc.push(elements::Paragraph::new(text));
        }
    }

    // Render
    println!("Rendering PDF to {}...", output_file);
    doc.render_to_file(output_file)
        .context("Failed to render PDF")?;

    Ok(())
}

fn section_header(text: &str) -> impl Element {
    let p = elements::Paragraph::new(text);
    let mut layout = elements::LinearLayout::vertical();
    layout.push(elements::Break::new(0.5));
    layout.push(p.styled(style::Style::new().bold().with_font_size(14)));
    layout.push(elements::Break::new(0.5));
    layout
}
