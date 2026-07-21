# RCV Format Guide

The `.rcv` file is a plain, human-readable text file that uses simple `@` directives to describe your resume sections.

## General Rules

1. Each section starts with a directive containing an `@` character (e.g., `@name:` or `@experience:`).
2. Directives can be single-line properties (like `@name:`) or multi-line blocks (like `@summary:`).
3. Whitespace around colons and commas is automatically trimmed.

## Directives

### Personal Information

The basic contact and metadata fields. These are single-line properties.

```rcv
@name: Your Name
@email: your.email@example.com
@phone: +1-234-567-8900
@website: https://yourwebsite.com
```

`@name:` is required; everything else is optional (a missing `@email:` produces a warning).

### Links

The `@links:` directive starts a section of labelled links, one per line, rendered in the resume header.

```rcv
@links:
LinkedIn: https://linkedin.com/in/yourname
GitHub: https://github.com/yourname
```

### Summary

The `@summary:` block allows for multi-line text blocks. It continues capturing text until the next `@` directive is reached.

```rcv
@summary:
A highly motivated Software Engineer...
I specialize in Rust.
```

### Skills

The `@skills:` directive starts a section of free-form categories. Any `Label: item, item` line becomes its own category, so you can pick groupings like `Languages`, `Technologies`, or `Frameworks`. Use comma-separated values; category labels are rendered as written.

```rcv
@skills:
Languages: Rust, Python, Go
Technologies: React, Git, Docker, Kubernetes
```

Multiple `@skills:` blocks are merged; items in a repeated category label (case-insensitive) are appended to the existing category.

### Experience

The `@experience:` directive begins a new job entry. You can define multiple `@experience:` blocks per resume. It expects specific sub-fields to be listed below it.

```rcv
@experience:
title: Senior Developer
company: Example Corp
date: 2020 - Present    # Can also just be a single year, e.g., 2020
description: Remote, UK # Optional location/details
- Led a team of 4 engineers
- Reduced build times by 40%
```

**Sub-fields:**
- `title:` (Required)
- `company:` (Required)
- `date:` (Required) - Accepts formats like `2020 - 2023` or `2022 - Present`. The range splits at a spaced ` - ` first, so dates such as `2020-09 - 2021-05` also work.
- `description:` (Optional) - Often used for location or a brief context.
- `- <text>` (Optional list items) - Add bullet points detailing accomplishments.

### Projects

The `@project:` directive begins a new project entry (the spelling `@projects:` is also accepted). Define one block per project.

```rcv
@project:
name: my-tool
description: Minimal project scaffolding tool with build configs.
tech: Rust, SQLite
link: https://github.com/yourname/my-tool
```

**Sub-fields:**
- `name:` (Required)
- `description:` (Required)
- `tech:` (Optional) - Comma-separated technologies, rendered in bold after the description.
- `link:` (Optional) - Rendered as a shortened URL next to the project.

### Education

The `@education:` directive behaves identically to experience but targets school histories.

```rcv
@education:
school: State University
degree: BSc Computer Science
year: 2015 - 2019
location: Leicester, UK
```

**Sub-fields:**
- `school:` (Required)
- `degree:` (Required)
- `year:` (Required) - Taken verbatim, so ranges like `Jan. 2024 - July 2025` are fine.
- `location:` (Optional) - Shown right-aligned in the PDF, like the experience location.
