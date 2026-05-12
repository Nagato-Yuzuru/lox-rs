use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

#[derive(Parser)]
#[command(version, about = "A Lox interpreter, in Rust.")]
struct Cli {
    /// Lox script to execute. Omit to start the REPL.
    script: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.script.map_or_else(run_repl, |path| run_file(&path))
}

fn run_file(path: &Path) -> Result<()> {
    let source = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    lox::run(&source).map_err(Into::into)
}

fn run_repl() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        match rl.readline("lox> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                if let Err(e) = lox::run(&line) {
                    eprintln!("{e}");
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
