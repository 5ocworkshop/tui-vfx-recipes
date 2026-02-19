//! # DEPRECATED
//!
//! This tool has been superseded by `pipeline-validator` which provides:
//! - All the same parse validation
//! - Visibility rules checking (color contrast, band coverage, etc.)
//! - Works from any directory (embedded rules)
//! - Custom rules support
//!
//! Use instead:
//! ```
//! pipeline-validator --rules /path/to/recipes/
//! ```
//!
//! See docs/tools/PIPELINE_VALIDATOR.md for full documentation.

use anyhow::Result;
use clap::Parser;
use colored::*;
use std::collections::HashMap;
use std::path::PathBuf;
use tui_vfx_recipes::recipe_schema::json_recipe_dyn_from_file;
use walkdir::WalkDir;

/// Recipe validator for V2 animation recipes
///
/// DEPRECATED: Use `pipeline-validator --rules` instead.
/// See docs/tools/PIPELINE_VALIDATOR.md
#[derive(Parser, Debug)]
#[command(name = "recipe-validator")]
#[command(about = "DEPRECATED: Use pipeline-validator instead. Validates V2 recipe JSON files.", long_about = None)]
struct Args {
    /// Path to the recipes directory (defaults to workspace_root/recipes)
    #[arg(short, long, value_name = "DIR")]
    recipes_dir: Option<PathBuf>,
}

fn find_workspace_root() -> Result<PathBuf> {
    let mut project_root = std::env::current_dir()?;
    loop {
        let cargo_toml = project_root.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    return Ok(project_root);
                }
            }
        }
        if !project_root.pop() {
            anyhow::bail!("Could not find workspace root (no Cargo.toml with [workspace] found)");
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Deprecation warning
    eprintln!(
        "{}",
        "⚠️  DEPRECATED: recipe-validator has been replaced by pipeline-validator"
            .yellow()
            .bold()
    );
    eprintln!(
        "{}",
        "   Use: pipeline-validator --rules /path/to/recipes/".yellow()
    );
    eprintln!("{}", "   See: docs/tools/PIPELINE_VALIDATOR.md\n".yellow());

    println!("{}", "\n=== Recipe Validator ===\n".bold().cyan());

    // Determine recipes directory and project root
    let (recipes_root, project_root) = if let Some(dir) = args.recipes_dir {
        // Custom recipes directory specified
        let proj_root = find_workspace_root().unwrap_or_else(|_| {
            dir.parent()
                .and_then(|p| p.parent())
                .unwrap_or(dir.as_path())
                .to_path_buf()
        });
        (dir, proj_root)
    } else {
        // Default: use workspace root
        let project_root = find_workspace_root()?;
        let recipes_root = project_root.join("recipes");
        (recipes_root, project_root)
    };

    if !recipes_root.exists() {
        eprintln!(
            "{}",
            format!("Error: recipes directory not found at {:?}", recipes_root)
                .red()
                .bold()
        );
        std::process::exit(1);
    }

    println!("Scanning recipes in: {}\n", recipes_root.display());

    // Find all subdirectories
    let mut directories = Vec::new();
    for entry in std::fs::read_dir(&recipes_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(name) = path.file_name() {
                directories.push(name.to_string_lossy().to_string());
            }
        }
    }
    directories.sort();

    println!("Found {} subdirectories:\n", directories.len());
    for dir in &directories {
        println!("  • {}", dir.cyan());
    }
    println!();

    // Validate recipes by directory
    let mut results_by_dir: HashMap<String, (Vec<PathBuf>, Vec<(PathBuf, String)>)> =
        HashMap::new();
    let mut total_passed = 0;
    let mut total_failed = 0;

    // First, validate root-level recipes (not in subdirectories)
    {
        let mut passed = Vec::new();
        let mut failed = Vec::new();

        for entry in std::fs::read_dir(&recipes_root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                match json_recipe_dyn_from_file(&path, &project_root) {
                    Ok(_) => {
                        passed.push(path.clone());
                    }
                    Err(e) => {
                        failed.push((path.clone(), e.to_string()));
                    }
                }
            }
        }

        if !passed.is_empty() || !failed.is_empty() {
            total_passed += passed.len();
            total_failed += failed.len();
            results_by_dir.insert("(root)".to_string(), (passed, failed));
            directories.insert(0, "(root)".to_string());
        }
    }

    for dir in directories.iter().filter(|d| *d != "(root)") {
        let dir_path = recipes_root.join(dir);
        let mut passed = Vec::new();
        let mut failed = Vec::new();

        // Walk the directory recursively
        for entry in WalkDir::new(&dir_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match json_recipe_dyn_from_file(path, &project_root) {
                    Ok(_) => {
                        passed.push(path.to_path_buf());
                    }
                    Err(e) => {
                        failed.push((path.to_path_buf(), e.to_string()));
                    }
                }
            }
        }

        total_passed += passed.len();
        total_failed += failed.len();
        results_by_dir.insert(dir.clone(), (passed, failed));
    }

    // Print results
    println!("{}", "=== VALIDATION RESULTS ===\n".bold().yellow());

    for dir in &directories {
        if let Some((passed, failed)) = results_by_dir.get(dir) {
            let total = passed.len() + failed.len();

            if failed.is_empty() {
                println!(
                    "{} {} ({} recipes)",
                    "✓".green().bold(),
                    dir.green().bold(),
                    total
                );
            } else {
                println!(
                    "{} {} ({}/{} passed)",
                    "✗".red().bold(),
                    dir.red().bold(),
                    passed.len(),
                    total
                );

                for (path, error) in failed {
                    let relative_path = path.strip_prefix(&project_root).unwrap_or(path);
                    println!("    {} {}", "↳".red(), relative_path.display());

                    // Print first line of error
                    let error_line = error.lines().next().unwrap_or("Unknown error");
                    println!("      {}", error_line.dimmed());
                }
            }
        }
    }

    println!();
    println!("{}", "=== SUMMARY ===".bold().yellow());
    println!("Total recipes tested: {}", total_passed + total_failed);
    println!("{} {}", "✓ Passed:".green().bold(), total_passed);

    if total_failed > 0 {
        println!("{} {}", "✗ Failed:".red().bold(), total_failed);
        println!();
        println!(
            "{}",
            "Run with RUST_BACKTRACE=1 for detailed error information.".dimmed()
        );
        std::process::exit(1);
    } else {
        println!("{} {}", "✗ Failed:".green(), total_failed);
        println!();
        println!(
            "{}",
            "All recipes validated successfully! 🎉".green().bold()
        );
        Ok(())
    }
}
