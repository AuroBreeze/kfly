use colored::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::maintainer::Maintainer;
use crate::config::Task;
use crate::workflow::ExecutionContext;

#[derive(Debug)]
pub struct Mails {
    pub to_emails: HashSet<String>,
    pub cc_emails: HashSet<String>,
}

impl Mails {
    pub fn new() -> Self {
        Self {
            to_emails: HashSet::new(),
            cc_emails: HashSet::new(),
        }
    }

    pub fn add_email(&mut self, maintainers: &Vec<Maintainer>) {
        for m in maintainers {
            if m.role.contains("maintainer") {
                self.to_emails.insert(m.email.clone());
            } else {
                self.cc_emails.insert(m.email.clone());
            }
        }
    }

    pub fn send_email(&self, task: &Task, context: &ExecutionContext ,patch: &Path, kernel_root: &PathBuf, is_test: bool, dry_run: bool) {
        // let mut cmd = Command::new("git");
        // cmd.arg("-C").arg(kernel_root).arg("send-email");
        // cmd.arg("--suppress-cc=all");
        let processed_args: Vec<String> = task
            .args
            .iter()
            .map(|arg| arg.replace("{patch}", &context.patch_path))
            .collect();

        let parts: Vec<&str> = task.command.split_whitespace().collect();
        let mut cmd = Command::new(parts[0]);

        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }

        cmd.arg("-C").arg(kernel_root);

        for arg in processed_args {
            cmd.arg(arg);
        }

        println!("{}", "📧 kfly send_mail handling......".cyan().bold());
        println!("\n{}", "--- [ 🚀 kfly Execution Plan ] ---".bold().cyan());

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
            if context.maintainers.is_empty() {
                println!("  {:>10}  {}", "To:".bold().cyan(), "None".yellow());
            }

            for email in &self.to_emails {
                println!("  {:>10}  {}", "To:".bold().cyan(), email.yellow());
                cmd.arg(format!("--to={}", email));
            }
            for email in &self.cc_emails {
                println!("  {:>10}  {}", "Cc:".bold().blue(), email.dimmed());
                cmd.arg(format!("--cc={}", email));
            }
        }

        // cmd.arg(patch);
        println!("  {:>10}  {}", "Patch:".bold().cyan(), patch.display());

        let preview = format!(
            "git -C {} send-email --to=... {}",
            kernel_root.display(),
            patch.display()
        );
        println!(
            "  {:>10}  {}",
            "Command:".bold().bright_black(),
            preview.italic().dimmed()
        );

        println!("{}", "---------------------------------".dimmed());

        if dry_run {
            println!(
                "{}",
                "🛡️  kfly dry-run mode: skipping execution".cyan().bold()
            );
            return;
        }

        println!("{}", "🚀 kfly: Launching send-email...".cyan().bold());
        match cmd.status() {
            Ok(status) if status.success() => {
                println!("\n{} {}", "✨".green(), "Success!".green().bold())
            }
            Ok(status) => eprintln!("\n{} {} {}", "❌".red(), "Failed:".red(), status),
            Err(e) => eprintln!("\n{} {} {}", "❌".red(), "Error:".red().bold(), e),
        }
    }
}
