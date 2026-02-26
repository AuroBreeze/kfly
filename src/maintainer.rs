use regex::Regex;
use colored::*;

#[derive(Debug)]
pub struct Maintainer {
    pub name: String,
    pub email: String,
    pub role: String,
}

pub fn parse_maintainers(raw_output: &str) -> Vec<Maintainer> {
    let re = Regex::new(r"^(?P<name>.*?) <(?P<email>.*?)> \((?P<role>.*?)\)$").unwrap();
    let mut list = Vec::new();
    for line in raw_output.lines() {
        if let Some(caps) = re.captures(line) {
            list.push(Maintainer {
                name: caps["name"].to_string(),
                email: caps["email"].to_string(),
                role: caps["role"].to_string(),
            });
        }
    }

    println!("{}", format!("✅ Patch {} maintainers:", list.len()).green().bold());
    for m in &list {
        println!(
            "   - {:^20} <{:^20}> [{}]",
            m.name.yellow(),
            m.email.bright_blue(),
            m.role.dimmed()
        );
    }
    list
}
