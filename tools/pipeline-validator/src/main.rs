// <FILE>tools/pipeline-validator/src/main.rs</FILE> - <DESC>Pipeline validator CLI entry point</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Pipeline performance debugging</WCTX>
// <CLOG>Add --bench flag for performance timing analysis</CLOG>

mod cli;
mod rules;
mod stages;
mod types;

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use clap::Parser;
use colored::Colorize;

use cli::{Args, OutputFormat, Stage};
use stages::StageResult;

fn main() -> ExitCode {
    let args = Args::parse();

    match run(&args) {
        Ok(results) => {
            report_results(&results, &args);

            if results.all_passed() {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            ExitCode::from(2)
        }
    }
}

/// Validation results for all recipes.
#[derive(Debug)]
struct ValidationResults {
    recipe_results: Vec<RecipeResult>,
}

impl ValidationResults {
    fn all_passed(&self) -> bool {
        self.recipe_results.iter().all(|r| r.passed)
    }
}

/// Result for a single recipe.
#[derive(Debug)]
struct RecipeResult {
    path: String,
    stages: Vec<StageResult>,
    passed: bool,
}

/// Run validation on all paths.
fn run(args: &Args) -> Result<ValidationResults, String> {
    let mut results = ValidationResults {
        recipe_results: Vec::new(),
    };

    for path in &args.paths {
        if path.is_dir() {
            // Validate all .json files in directory
            let entries =
                fs::read_dir(path).map_err(|e| format!("Cannot read directory: {}", e))?;

            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.extension().map(|e| e == "json").unwrap_or(false) {
                    results
                        .recipe_results
                        .push(validate_recipe(&entry_path, args)?);
                }
            }
        } else {
            results.recipe_results.push(validate_recipe(path, args)?);
        }
    }

    Ok(results)
}

/// Validate a single recipe through all applicable stages.
fn validate_recipe(path: &Path, args: &Args) -> Result<RecipeResult, String> {
    let path_str = path.display().to_string();
    let mut stage_results = Vec::new();
    let stop_at = args.stage;

    // Stage 1: Parse
    let parse_result = stages::parse::validate(path, args)?;
    let parse_passed = parse_result.passed;
    stage_results.push(parse_result);

    if !parse_passed || stop_at == Some(Stage::Parse) {
        return Ok(RecipeResult {
            path: path_str,
            stages: stage_results,
            passed: parse_passed,
        });
    }

    // Load the parsed config for subsequent stages
    // Determine project_root for template resolution
    let project_root = path
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists())
        .unwrap_or_else(|| Path::new("."));

    let recipe = tui_vfx_recipes::recipe::load(path, project_root)
        .map_err(|e| format!("Recipe loading failed: {}", e))?;
    let config = recipe.config().clone();

    // Stage 2: Rules validation (if enabled)
    if args.rules {
        let rules = if let Some(ref rules_path) = args.rules_file {
            rules::load_rules_from_file(rules_path)?
        } else {
            rules::load_default_rules()?
        };

        let rules_result = stages::rules::validate(&config, &rules, args);
        let rules_passed = rules_result.passed;
        stage_results.push(rules_result);

        if !rules_passed {
            return Ok(RecipeResult {
                path: path_str,
                stages: stage_results,
                passed: false,
            });
        }
    }

    // Stage 3: Profile validation
    if stop_at != Some(Stage::Parse) {
        let profile_result = stages::validate_profile(&config, args);
        let profile_passed = profile_result.passed;
        stage_results.push(profile_result);

        if !profile_passed || stop_at == Some(Stage::Profile) {
            return Ok(RecipeResult {
                path: path_str,
                stages: stage_results,
                passed: profile_passed,
            });
        }
    }

    // Stage 4: Render validation
    if stop_at != Some(Stage::Parse) && stop_at != Some(Stage::Profile) {
        let render_result = stages::validate_render(&config, args);
        let render_passed = render_result.passed;
        stage_results.push(render_result);

        if !render_passed || stop_at == Some(Stage::Render) {
            return Ok(RecipeResult {
                path: path_str,
                stages: stage_results,
                passed: render_passed,
            });
        }
    }

    // Stage 5: Shader validation
    if stop_at != Some(Stage::Parse)
        && stop_at != Some(Stage::Profile)
        && stop_at != Some(Stage::Render)
    {
        let shader_result = stages::validate_shader(&config, args);
        let shader_passed = shader_result.passed;
        stage_results.push(shader_result);

        if !shader_passed || stop_at == Some(Stage::Shader) {
            return Ok(RecipeResult {
                path: path_str,
                stages: stage_results,
                passed: shader_passed,
            });
        }
    }

    // Stage 6: Output validation (actual rendering)
    if stop_at.is_none() || stop_at == Some(Stage::Output) {
        let output_result = stages::validate_output(&config, args);
        let output_passed = output_result.passed;
        stage_results.push(output_result);

        if !output_passed {
            return Ok(RecipeResult {
                path: path_str,
                stages: stage_results,
                passed: false,
            });
        }
    }

    // Stage 7: Pipeline stages inspection (when --stages flag is set)
    if args.stages {
        let stages_result = stages::validate_stages(&config, args);
        let stages_passed = stages_result.passed;
        stage_results.push(stages_result);

        if !stages_passed {
            return Ok(RecipeResult {
                path: path_str,
                stages: stage_results,
                passed: false,
            });
        }
    }

    // Stage 8: Performance benchmarking (when --bench flag is set)
    if args.bench {
        let bench_result = stages::benchmark_stages(&config, args);
        stage_results.push(bench_result);
    }

    let all_passed = stage_results.iter().all(|s| s.passed);

    Ok(RecipeResult {
        path: path_str,
        stages: stage_results,
        passed: all_passed,
    })
}

/// Report results to stdout.
fn report_results(results: &ValidationResults, args: &Args) {
    match args.format {
        OutputFormat::Text => report_text(results, args),
        OutputFormat::Json => report_json(results),
    }
}

/// Human-readable text output.
fn report_text(results: &ValidationResults, args: &Args) {
    for recipe in &results.recipe_results {
        let filename = Path::new(&recipe.path)
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_else(|| recipe.path.clone());

        if recipe.passed {
            println!("{} {}", "[PASS]".green().bold(), filename);
        } else {
            println!("{} {}", "[FAIL]".red().bold(), filename);
        }

        for stage in &recipe.stages {
            let stage_name = stage.stage.to_uppercase();

            if stage.passed {
                println!("  {} {}", format!("[{}]", stage_name).cyan(), "OK".green());
            } else {
                println!(
                    "  {} {}",
                    format!("[{}]", stage_name).cyan(),
                    "FAILED".red()
                );
            }

            // Show messages based on verbosity
            if args.verbose >= 1 || !stage.passed {
                for msg in &stage.messages {
                    let prefix = if stage.passed {
                        "    +".green()
                    } else {
                        "    !".red()
                    };
                    println!("{} {}", prefix, msg);
                }
            }

            // Show details in verbose mode
            if args.verbose >= 2 {
                for detail in &stage.details {
                    println!("      {}", detail.dimmed());
                }
            }
        }

        println!();
    }

    // Summary
    let total = results.recipe_results.len();
    let passed = results.recipe_results.iter().filter(|r| r.passed).count();
    let failed = total - passed;

    println!("{}", "Summary:".bold());
    println!("  Total: {}", total);
    println!("  {}: {}", "Passed".green(), passed);
    if failed > 0 {
        println!("  {}: {}", "Failed".red(), failed);
    }
}

/// JSON output for CI integration.
fn report_json(results: &ValidationResults) {
    #[derive(serde::Serialize)]
    struct JsonOutput {
        recipes: Vec<JsonRecipe>,
        summary: JsonSummary,
    }

    #[derive(serde::Serialize)]
    struct JsonRecipe {
        path: String,
        status: String,
        stages: Vec<StageResult>,
    }

    #[derive(serde::Serialize)]
    struct JsonSummary {
        total: usize,
        passed: usize,
        failed: usize,
    }

    let recipes: Vec<JsonRecipe> = results
        .recipe_results
        .iter()
        .map(|r| JsonRecipe {
            path: r.path.clone(),
            status: if r.passed {
                "PASS".to_string()
            } else {
                "FAIL".to_string()
            },
            stages: r.stages.clone(),
        })
        .collect();

    let total = results.recipe_results.len();
    let passed = results.recipe_results.iter().filter(|r| r.passed).count();

    let output = JsonOutput {
        recipes,
        summary: JsonSummary {
            total,
            passed,
            failed: total - passed,
        },
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&output).unwrap_or_default()
    );
}

// <FILE>tools/pipeline-validator/src/main.rs</FILE> - <DESC>Pipeline validator CLI entry point</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
