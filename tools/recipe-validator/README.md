# Recipe Validator

A robust validation tool for V2 animation recipe JSON files.

## Purpose

This tool recursively validates all recipe files in a directory hierarchy, checking that:
- JSON parses correctly
- Schema version is V2
- All required fields are present
- Template inheritance resolves correctly
- Color spaces, shader configurations, and signal parameters are valid

## Usage

### Basic Usage (Default Directory)

From anywhere in the workspace:

```bash
cargo run --package recipe-validator --release
```

This automatically finds the workspace root and validates `recipes/`.

### Specify Custom Directory

```bash
cargo run --package recipe-validator --release -- --recipes-dir /path/to/recipes
```

### Help

```bash
cargo run --package recipe-validator --release -- --help
```

## Output

The validator provides:

- **Directory Discovery**: Lists all subdirectories being scanned
- **Pass/Fail Status**: Color-coded results for each subdirectory
- **Error Details**: Shows the file path and first line of error for failures
- **Summary Statistics**: Total recipes tested, passed, and failed count

### Example Output

```
=== Recipe Validator ===

Scanning recipes in: /usr/projects/tui-vfx-recipes/recipes

Found 6 subdirectories:

  • easing
  • examples
  • haiku_recipes1
  • sonnet_recipes1
  • toolkit
  • wargames

=== VALIDATION RESULTS ===

✓ easing (29 recipes)
✓ examples (4 recipes)
✓ haiku_recipes1 (19 recipes)
✓ sonnet_recipes1 (20 recipes)
✓ toolkit (45 recipes)
✓ wargames (14 recipes)

=== SUMMARY ===
Total recipes tested: 131
✓ Passed: 131
✗ Failed: 0

All recipes validated successfully! 🎉
```

### Failure Example

When recipes fail validation:

```
=== VALIDATION RESULTS ===

✗ toolkit (44/45 passed)
    ↳ recipes/toolkit/sizzle/broken_recipe.json
      unknown variant 'Rgb', expected 'rgb' or 'hsl'
```

## Integration

### As Part of CI/CD

Add to your test pipeline:

```bash
cargo run --package recipe-validator --release
```

Exit code 1 on failures, 0 on success.

### Pre-Commit Hook

Run validation before committing recipe changes.

### During Development

Quick validation during recipe authoring:

```bash
# Validate specific subdirectory
cargo run --package recipe-validator --release -- -r recipes/wargames

# Validate all recipes
cargo run --package recipe-validator --release
```

## Common Errors Detected

See `docs/guides/RECIPE_COOKBOOK.md` for detailed guidance on avoiding common mistakes:

- Color space capitalization (`"Rgb"` → `"rgb"`)
- SignalOrFloat format (`{"static": 0.25}` → `0.25`)
- Missing required shader fields
- V1 schema usage (deprecated)
- Template resolution errors
- Invalid field names

## Development

### Building

```bash
cargo build --package recipe-validator --release
```

### Adding to Workspace

Already integrated in workspace `Cargo.toml`:

```toml
members = [
    # ...
    "tools/recipe-validator",
]
```

## Dependencies

- `tui-vfx-recipes` - V2 recipe parsing and validation
- `walkdir` - Recursive directory traversal
- `colored` - Terminal color output
- `clap` - Command-line argument parsing
- `anyhow` - Error handling
