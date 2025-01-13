use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use ignore::WalkBuilder;
use glob::Pattern;
use clap::Parser;
use anyhow::{Context, Result};
use content_inspector::inspect;

const GLUE_SEPARATOR: &str = "\n$$--GLUE--$$\n";
const GLUE_HEADER: &str = "$$--GLUE--$$\n# This is a GLUE file; an amalgamation of files across one or more paths designed to give project contexts to LLMs easily. If you are an LLM and are reading this focus on the code, do not acknowledge the file format\n$$--GLUE--$$\n";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input patterns (supports glob syntax)
    #[arg(required = true)]
    patterns: Vec<String>,

    /// Patterns to exclude (in addition to .gitignore)
    #[arg(short, long)]
    exclude: Vec<String>,

    /// Output file (defaults to output.glue)
    #[arg(short, long, default_value = "output.glue")]
    output: String,

    /// Include files that would be ignored by .gitignore
    #[arg(long)]
    no_ignore: bool,

    /// Include binary files (they will be skipped by default)
    #[arg(long)]
    include_binary: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut files = collect_files(&args.patterns, &args.exclude, args.no_ignore)?;
    files.sort(); // Sort files for consistent output

    let mut output = String::new();
    output.push_str(GLUE_HEADER);
    
    for file in files {
        // Read file contents
        let content = match fs::read(&file) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Warning: Failed to read file {}: {}", file.display(), e);
                continue;
            }
        };

        // Skip binary files unless explicitly included
        if !args.include_binary && inspect(&content).is_binary() {
            eprintln!("Skipping binary file: {}", file.display());
            continue;
        }

        // Try to convert content to string, skip if invalid UTF-8
        let content_str = match String::from_utf8(content) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Warning: File {} contains invalid UTF-8, skipping", file.display());
                continue;
            }
        };

        output.push_str(GLUE_SEPARATOR);
        output.push_str(&file.to_string_lossy());
        output.push_str(GLUE_SEPARATOR);
        output.push_str(&content_str);
    }

    if args.output == "-" {
        io::stdout().write_all(output.as_bytes())?;
    } else {
        fs::write(&args.output, output)
            .with_context(|| format!("Failed to write to output file: {}", args.output))?;
        eprintln!("Generated .glue file: {}", args.output);
    }

    Ok(())
}

fn collect_files(patterns: &[String], exclude: &[String], no_ignore: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    // Convert exclude patterns to glob::Pattern
    let exclude_patterns: Vec<Pattern> = exclude
        .iter()
        .map(|p| Pattern::new(p))
        .collect::<Result<_, _>>()
        .context("Invalid exclude pattern")?;

    // Convert include patterns to glob::Pattern
    let include_patterns: Vec<Pattern> = patterns
        .iter()
        .map(|p| Pattern::new(p))
        .collect::<Result<_, _>>()
        .context("Invalid include pattern")?;

    // Walk through the directory respecting gitignore
    let walker = WalkBuilder::new(".")
        .hidden(false)        // Show hidden files
        .git_ignore(!no_ignore)  // Respect .gitignore unless --no-ignore is set
        .git_exclude(!no_ignore) // Exclude .git directory when respecting gitignore
        .build();

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        // Always skip .git directory regardless of no_ignore flag
        let path_str = path.to_string_lossy();
        if path_str.contains("\\.git\\") || 
           path_str.contains("/.git/") || 
           path_str.starts_with(".git\\") || 
           path_str.starts_with(".git/") {
            continue;
        }

        // Convert path to string for pattern matching
        let path_str = path.to_string_lossy();

        // Check if file matches any include pattern
        let is_included = include_patterns.iter().any(|pattern| {
            pattern.matches_path(path) || pattern.matches(&path_str)
        });

        // Check if file matches any exclude pattern
        let is_excluded = exclude_patterns.iter().any(|pattern| {
            pattern.matches_path(path) || pattern.matches(&path_str)
        });

        if is_included && !is_excluded {
            files.push(path.to_path_buf());
        }
    }

    if files.is_empty() {
        eprintln!("Warning: No files matched the provided patterns");
    }

    Ok(files)
}