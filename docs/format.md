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

### Summary

The `@summary:` block allows for multi-line text blocks. It continues capturing text until the next `@` directive is reached.

```rcv
@summary:
A highly motivated Software Engineer...
I specialize in Rust.
```

### Skills

The `@skills:` directive starts a section where you can categorize skills using `languages:`, `frameworks:`, and `tools:`. Use comma-separated values.

```rcv
@skills:
languages: Rust, Python, Go
frameworks: React, Rocket
tools: Git, Docker, Kubernetes
```

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
- `date:` (Required) - Accepts formats like `2020 - 2023` or `2022 - Present`.
- `description:` (Optional) - Often used for location or a brief context.
- `- <text>` (Optional list items) - Add bullet points detailing accomplishments.

### Education

The `@education:` directive behaves identically to experience but targets school histories.

```rcv
@education:
school: State University
degree: BSc Computer Science
year: 2015 - 2019
```

**Sub-fields:**
- `school:` (Required)
- `degree:` (Required)
- `year:` (Required)
