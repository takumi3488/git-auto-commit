use chrono::Local;
use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[clap(short, long)]
    push: bool,
}
fn main() {
    Command::new("git").args(&["add", "-A"]).output().unwrap();
    Command::new("git")
        .args(&[
            "commit",
            "-m",
            &Local::now()
                .format("'Updated at: %Y/%m/%d %H:%M:%S'")
                .to_string(),
        ])
        .output()
        .unwrap();
    if Cli::parse().push {
        Command::new("git")
            .args(&["push", "origin", "main"])
            .output()
            .unwrap();
    }
}
