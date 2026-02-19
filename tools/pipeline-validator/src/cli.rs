// <FILE>tools/pipeline-validator/src/cli.rs</FILE> - <DESC>CLI argument parsing with clap</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Pipeline performance debugging</WCTX>
// <CLOG>Add --bench flag for pipeline timing analysis</CLOG>

use clap::Parser;
use std::path::PathBuf;

/// Pipeline validator CLI tool for validating tui-vfx-recipes recipes.
#[derive(Parser, Debug)]
#[command(name = "pipeline-validator")]
#[command(version, about = "Validate tui-vfx-recipes recipes and pipeline")]
pub struct Args {
    /// Recipe file(s) or directory to validate
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,

    /// Increase verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Output format: text, json
    #[arg(long, default_value = "text")]
    pub format: OutputFormat,

    /// Stop at specific stage: parse, profile, render, shader, output
    #[arg(long)]
    pub stage: Option<Stage>,

    /// Test specific phase: entering, dwelling, exiting
    #[arg(long)]
    pub phase: Option<Phase>,

    /// Sample at specific t values (0.0-1.0), comma-separated
    #[arg(long, num_args = 1.., value_delimiter = ',')]
    pub sample_t: Option<Vec<f64>>,

    /// Dump intermediate state at specified stage
    #[arg(long)]
    pub dump: bool,

    /// Enable full pipeline tracing
    #[arg(long)]
    pub trace: bool,

    /// Enable visibility rules validation
    #[arg(long)]
    pub rules: bool,

    /// Path to custom rules file (default: built-in rules)
    #[arg(long, value_name = "FILE")]
    pub rules_file: Option<PathBuf>,

    /// Treat warnings as errors
    #[arg(long)]
    pub strict: bool,

    /// Show only errors, hide warnings
    #[arg(long)]
    pub errors_only: bool,

    /// Enable pipeline stage inspection (sampler, mask, shader, filter)
    #[arg(long)]
    pub stages: bool,

    /// Run performance benchmarking (measures render pipeline timing)
    #[arg(long)]
    pub bench: bool,

    /// Number of iterations for benchmarking (default: 100)
    #[arg(long, default_value = "100")]
    pub iterations: usize,
}

impl Args {
    /// Get sample points, defaulting to 0.0, 0.5, 1.0
    pub fn sample_points(&self) -> Vec<f64> {
        self.sample_t.clone().unwrap_or_else(|| vec![0.0, 0.5, 1.0])
    }
}

/// Output format for validation results.
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

/// Validation stages in the pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Stage {
    Parse,
    Profile,
    Render,
    Shader,
    Output,
}

/// Animation phases for targeted testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Phase {
    Entering,
    Dwelling,
    Exiting,
}

// <FILE>tools/pipeline-validator/src/cli.rs</FILE> - <DESC>CLI argument parsing with clap</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
