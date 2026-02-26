mod args;
mod config;
mod mail;
mod maintainer;

use clap::Parser;
use colored::*;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, io};

use crate::args::Args;
use crate::config::Config;
use crate::mail::Mails;
use crate::maintainer::parse_maintainers;

fn main() {
    let args = Args::parse();

    println!(
        "{} {}",
        "🚀 kfly handling:".cyan().bold(),
        args.patch.yellow()
    );
    // Path handling
    let absolute_patch =
        fs::canonicalize(&args.patch).expect("❌ kfly: Failed to get absolute patch path");

    println!("{}", "🔍 kfly: Loading kfly.toml......".cyan().bold());
    let config = Config::load("./src/kfly.toml").expect("❌ kfly: Failed to load kfly.toml");
    println!("{}", "✅ kfly: kfly.toml loaded".green().bold());

    run_workflow(config, &absolute_patch);

    //
    // // Execute get_maintainer.pl
    // println!("{}", "🔍 kfly get_maintainer handling......".cyan().bold());
    // let output = Command::new("perl")
    //     .arg(args.kernel_root.join("scripts/get_maintainer.pl"))
    //     .arg(&absolute_patch)
    //     .current_dir(&args.kernel_root)
    //     .output();
    //
    // let mut mails = Mails::new();
    //
    // match output {
    //     Ok(out) if out.status.success() => {
    //         let result = String::from_utf8_lossy(&out.stdout).to_string();
    //         let maintainers = parse_maintainers(&result);
    //
    //         mails.add_email(maintainers);
    //         println!("{}", "🚀 kfly get_maintainer success".cyan().bold());
    //
    //         // Note: I set is_test and dry_run based on args
    //         mails.send_email(&absolute_patch, &args.kernel_root, args.test, args.dry_run);
    //     }
    //     Ok(_) => println!("{}", "❌ kfly get_maintainer failed".red().bold()),
    //     Err(e) => println!("{} {}", "❌ kfly execution error:".red().bold(), e),
    // }
}

fn run_workflow(config: Config, absolute_patch: &Path) {
    let kernel_root = &config.settings.kernel_root;
    let patch_str = absolute_patch.to_string_lossy();

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

        // special handling
        // if task.command == "git send-email" {
        // }

        let mut _cmd = if task.command.ends_with(".pl") {
            let mut c = Command::new("perl");
            c.arg(Path::new(&kernel_root).join(&task.command));
            c
        } else {
            Command::new(&task.command)
        };
        let processed_args: Vec<String> = task
            .args
            .iter()
            .map(|arg| arg.replace("{patch}", &patch_str))
            .collect();
        let parts: Vec<&str> = task.command.split_whitespace().collect();
        let mut cmd = Command::new(parts[0]);

        if parts.len() > 1 {
            cmd.args(&parts[1..]);
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
                    return;
                }
            }
        }
    }
}
