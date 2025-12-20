mod resume;

use resume::Resume;

fn main() {
    // This is our "DSL" in action.
    // It reads almost like a configuration file, but it's pure Rust.
    let my_resume = Resume::build()
        .name("Nana Adjei Manu")
        .email("nana@example.com")
        .phone("+1-555-0102")
        .website("https://nanamanu.com")
        .summary("Senior Software Engineer with a passion for Rust, Distributed Systems, and AI. Proven track record of building scalable applications.")
        
        .skills(&["Rust", "Go", "Python", "Kubernetes", "AWS", "React"])
        
        .experience(|e| e
            .title("Senior Software Engineer")
            .company("Tech Giants Inc.")
            .start("2023-01")
            .current()
            .description("Leading the backend infrastructure team.")
            .highlight("Improved API latency by 40% using Rust.")
            .highlight("Mentored 3 junior engineers to promotion.")
        )
        .experience(|e| e
            .title("Software Engineer")
            .company("Startup Hustle")
            .start("2020-05")
            .end("2022-12")
            .description("Full stack development for a rapid-growth startup.")
            .highlight("Built the initial MVP in 3 months.")
            .highlight("Scaled database from 1k to 1M users.")
        )
        
        .education(|e| e
            .school("University of Technology")
            .degree("B.S. Computer Science")
            .year("2020")
        )
        
        .finish();

    // The Display trait is implemented to output Markdown.
    println!("{}", my_resume);
}
