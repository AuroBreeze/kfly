mod args;
mod config;
mod mail;
mod maintainer;
mod workflow;

use clap::Parser;
use colored::*;
use std::fs;

use crate::args::Args;
use crate::config::Config;
use crate::workflow::run_workflow;

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
}
