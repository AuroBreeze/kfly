use clap::Parser;
use colored::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Patch path
    #[arg(short, long)]
    patch: String,

    /// mode switch
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    #[arg(short, long, default_value_t = true)]
    test: bool,

    /// kernel path
    #[arg(short, long, default_value = ".")]
    kernel_root: PathBuf,
}

#[derive(Debug)]
struct Maintainer {
    name: String,
    email: String,
    role: String,
}

#[derive(Debug)]
struct Mails {
    to_emails: HashSet<String>,
    cc_emails: HashSet<String>,
}

impl Mails {
    fn new() -> Mails {
        Mails {
            to_emails: HashSet::new(),
            cc_emails: HashSet::new(),
        }
    }

    fn add_email(&mut self, maintainers: Vec<Maintainer>) {
        for m in &maintainers {
            if m.role.contains("maintainer") {
                self.to_emails.insert(m.email.clone());
            } else {
                self.cc_emails.insert(m.email.clone());
            }
        }
        // dbg!(&self);
        // println!("{:#?}", self);
    }

    fn send_email(&self, patch: &str, _kernel_root: &PathBuf, is_test: bool) {
        let mut cmd = Command::new("git");

        // Setup base command: Change directory to kernel root
        cmd.arg("-C").arg(_kernel_root).arg("send-email");
        // Disable automatic Cc extraction to keep control in kfly
        cmd.arg("--suppress-cc=all");

        // --- UI Header ---
        println!("\n{}", "--- [ 🚀 kfly Execution Plan ] ---".bold().cyan());

        // 1. Handle Recipients logic
        if is_test {
            let self_email = "AuroBreeze@outlook.com";
            println!(
                "  {:>10}  {}",
                "Mode:".bold().magenta(),
                "TEST (Self-only)".on_magenta().white()
            );
            println!("  {:>10}  {}", "To:".bold().cyan(), self_email.yellow());
            cmd.arg(format!("--to={}", self_email));
        } else {
            println!(
                "  {:>10}  {}",
                "Mode:".bold().green(),
                "PRODUCTION".on_green().white()
            );

            for email in &self.to_emails {
                println!("  {:>10}  {}", "To:".bold().cyan(), email.yellow());
                cmd.arg(format!("--to={}", email));
            }

            for email in &self.cc_emails {
                println!("  {:>10}  {}", "Cc:".bold().blue(), email.dimmed());
                cmd.arg(format!("--cc={}", email));
            }
        }

        // 2. Add the patch path
        cmd.arg(patch);

        // --- Metadata & Preview ---
        println!("  {:>10}  {}", "Patch:".bold().cyan(), patch.underline());

        // Clean Command Preview (Simplified Shell-style string)
        let preview = format!(
            "git -C {} send-email --to=... {}",
            _kernel_root.display(),
            patch
        );
        println!(
            "  {:>10}  {}",
            "Command:".bold().bright_black(),
            preview.italic().dimmed()
        );

        println!("{}", "---------------------------------".dimmed());
        println!("{}", "🚀 kfly: Launching send-email...".cyan().bold());

        // 3. Execution and Result Handling
        match cmd.status() {
            Ok(status) => {
                if status.success() {
                    println!(
                        "\n{} {}",
                        "✨".green(),
                        "Success! Patch delivered.".green().bold()
                    );
                } else {
                    eprintln!(
                        "\n{} {} {}",
                        "❌".red(),
                        "Failed: git-send-email exited with".red(),
                        status
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "\n{} {} {}",
                    "❌".red(),
                    "Critical Error:".red().bold(),
                    e.to_string().red()
                );
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let absolute_patch = fs::canonicalize(&args.patch).expect("❌ kfly get absolute patch failed");
    let patch_str = absolute_patch.to_str().unwrap();

    println!(
        "{} {}",
        "🚀 kfly handling:".cyan().bold(),
        args.patch.yellow()
    );

    if args.dry_run {
        println!("{}", "🚀 kfly dry-run mode".cyan().bold());
    }

    println!("{}", "🔍 kfly get_maintainer handling......".cyan().bold());
    let output = Command::new("perl")
        .arg(args.kernel_root.join("scripts/get_maintainer.pl"))
        .arg(&patch_str)
        .current_dir(&args.kernel_root)
        .output();

    let mut mails = Mails::new();

    match output {
        Ok(output) => {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout).to_string();
                let maintainers = parse_maintainers(&result);
                mails.add_email(maintainers);
                println!("{}", "🚀 kfly get_maintainer success".cyan().bold());
                mails.send_email(&patch_str, &args.kernel_root, true);
            } else {
                println!("{}", "❌ kfly get_maintainer failed".red().bold());
            }
        }
        Err(e) => {
            println!(
                "{} {}",
                "❌ kfly get_maintainer failed:".red().bold(),
                e.to_string().red()
            );
        }
    }
}

fn parse_maintainers(raw_output: &str) -> Vec<Maintainer> {
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

    let count_msg = format!("✅ Patch {} maintainers:", list.len());
    println!("{}", count_msg.green().bold());

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

#[allow(dead_code)]
fn run_ls() -> Result<String, String> {
    let output_result = Command::new("ls").arg("-lh").output();

    match output_result {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(stdout)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                Err(stderr)
            }
        }

        Err(e) => Err(e.to_string()),
    }
}
