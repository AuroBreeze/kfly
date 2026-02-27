use colored::*;

use std::io;
use std::io::Write;
use std::option::Option;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::{Config, Task};
use crate::mail::Mails;
use crate::maintainer::{parse_maintainers, Maintainer};

pub struct ExecutionContext {
    pub maintainers: Vec<Maintainer>,
    pub patch_path: String,
}

pub fn run_workflow(config: Config, absolute_patch: &Path) {
    let kernel_root = &config.settings.kernel_root;
    let patch_str = absolute_patch.to_string_lossy().to_string();

    let mut ctx = ExecutionContext {
        maintainers: Vec::new(),
        patch_path: patch_str.clone(),
    };

    for task in config.workflow {
        println!("🚀 Running task: {}", task.name.cyan());

        if task.interactive {
            println!(
                "{} {} (y/N)",
                "❓ Interactive mode".cyan(),
                task.name.yellow()
            );
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() != "y" && input.trim() != "Y" {
                println!(
                    "{} {} skipped",
                    "🚀 Interactive mode".cyan(),
                    task.name.yellow()
                );
                continue;
            }
        }

        match task.name.as_str() {
            "Get Maintainers" => {
                if let Some(m) = handle_get_maintainer(&task, kernel_root, &patch_str) {
                    ctx.maintainers = m;
                }
            }

            "Send Email" => {
                handle_send_mail(&task, kernel_root, &ctx);
            }

            _ => {
                if !execute_generic_task(&task, kernel_root, &patch_str) {
                    return;
                }
            }
        }
    }
}

fn execute_generic_task(task: &Task, kernel_root: &Path, patch_path: &str) -> bool{
    let processed_args: Vec<String> = task
        .args
        .iter()
        .map(|arg| arg.replace("{patch}", patch_path))
        .collect();
    let parts: Vec<&str> = task.command.split_whitespace().collect();
    let mut _cmd = Command::new(parts[0]);

    if parts.len() > 1 {
        _cmd.args(&parts[1..]);
    }

    _cmd.args(&processed_args);
    _cmd.current_dir(kernel_root);

    match _cmd.status() {
        Ok(status) if status.success() => {
            println!(
                "{} {} {}",
                "✨".green(),
                "Task success!".green().bold(),
                task.name.cyan()
            );
            true
        }

        _ => {
            println!(
                "{} {} {}",
                "❌".red(),
                "Task failed:".red().bold(),
                task.name.cyan()
            );

            if task.fail_fast {
                println!(
                    "{} {} {}",
                    "🛑".red(),
                    "Task failed:".red().bold(),
                    task.name.cyan()
                );
                false
            }else{
                true
            }
        }
    }
}

fn handle_get_maintainer(
    task: &Task,
    kernel_root: &Path,
    patch_path: &str,
) -> Option<Vec<Maintainer>> {
    // Execute get_maintainer.pl
    println!("{}", "🔍 kfly get_maintainer handling......".cyan().bold());
    let output = Command::new("perl")
        .arg(kernel_root.join(&task.command))
        .arg(patch_path)
        .current_dir(kernel_root)
        .output();

    let output = output.ok()?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        let maintainers = parse_maintainers(&result);
        // dbg!(&maintainers);
        println!("{}", "✅ kfly get_maintainer success".green().bold());

        Some(maintainers)
    } else {
        println!("{}", "❌ kfly get_maintainer failed".red().bold());
        None
    }
}

fn handle_send_mail(task: &Task, kernel_root: &Path, context: &ExecutionContext) {
    let mut mails = Mails::new();

    mails.add_email(&context.maintainers);
    let patch_buf = PathBuf::from(&context.patch_path);
    let kernel_path = kernel_root.to_path_buf();
    mails.send_email(task, context, &patch_buf, &kernel_path, true, false);

}
