mod args;
mod maintainer;
mod mail;

use clap::Parser;
use colored::*;
use std::fs;
use std::process::Command;

use crate::args::Args;
use crate::maintainer::parse_maintainers;
use crate::mail::Mails;

fn main() {
    let args = Args::parse();
    
    // Path handling
    let absolute_patch = fs::canonicalize(&args.patch)
        .expect("❌ kfly: Failed to get absolute patch path");

    println!("{} {}", "🚀 kfly handling:".cyan().bold(), args.patch.yellow());

    // Execute get_maintainer.pl
    println!("{}", "🔍 kfly get_maintainer handling......".cyan().bold());
    let output = Command::new("perl")
        .arg(args.kernel_root.join("scripts/get_maintainer.pl"))
        .arg(&absolute_patch)
        .current_dir(&args.kernel_root)
        .output();

    let mut mails = Mails::new();

    match output {
        Ok(out) if out.status.success() => {
            let result = String::from_utf8_lossy(&out.stdout).to_string();
            let maintainers = parse_maintainers(&result);
            
            mails.add_email(maintainers);
            println!("{}", "🚀 kfly get_maintainer success".cyan().bold());
            
            // Note: I set is_test and dry_run based on args
            mails.send_email(&absolute_patch, &args.kernel_root, args.test, args.dry_run);
        }
        Ok(_) => println!("{}", "❌ kfly get_maintainer failed".red().bold()),
        Err(e) => println!("{} {}", "❌ kfly execution error:".red().bold(), e),
    }
}
