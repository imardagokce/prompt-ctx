mod cli;
mod ignore_handler;

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;
use tiktoken_rs::cl100k_base;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    let root = args.path.as_deref().unwrap_or_else(|| std::path::Path::new("."));
    let files = ignore_handler::find_text_files(root)?;
    let tokenizer = cl100k_base().context("failed to initialize cl100k tokenizer")?;

    let mut total_tokens = 0usize;
    let mut contents = String::new();
    for path in files {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {} as text", path.display()))?;
        total_tokens += tokenizer.encode_with_special_tokens(&content).len();
        contents.push_str(&format!("\n--- {} ---\n", path.display()));
        contents.push_str(&content);
        if !content.ends_with('\n') {
            contents.push('\n');
        }
    }

    println!("Total tokens: {total_tokens}");
    if let Some(limit) = args.max_tokens {
        println!("Token budget: {limit} ({} remaining)", limit.saturating_sub(total_tokens));
    }
    if let Some(output) = args.output {
        fs::write(&output, contents)
            .with_context(|| format!("failed to write {}", output.display()))?;
    }
    Ok(())
}
