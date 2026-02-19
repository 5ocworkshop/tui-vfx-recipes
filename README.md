# tui-vfx-recipes

A collection of hundreds of JSON animation and notification recipes for the
[tui-vfx](https://crates.io/crates/tui-vfx) terminal visual effects framework,
bundled with an interactive recipe browser.

This crate is a companion to **tui-vfx**, which is the main project containing
full documentation, the rendering engine, and the compositor. This crate provides
the recipes themselves -- ready-made effect configurations you can browse, use
directly, or use as starting points for your own.

## What's included

- **400+ JSON recipes** covering animations, notifications, transitions, progress
  indicators, borders, physics-based motion, and more
- **Interactive demo browser** for previewing every recipe in your terminal
- Recipe loading, parsing, and validation
- Template inheritance and resolution
- A recipe registry for managing collections

## Recipe browser

The fastest way to explore is the built-in demo:

```bash
cargo run --example demo --release
```

Use the arrow keys to browse recipes, then press Space or Enter to preview the
selected recipe in the lower-right pane.

## Usage

```rust,ignore
use tui_vfx_recipes::prelude::*;

// Load a recipe from JSON
let recipe = load(Path::new("recipes/my_effect.json"), Path::new("."))?;

// Create a preview manager
let mut manager = PreviewManager::new();
let item = preview_from_recipe_config(recipe.config());
manager.add(item, Instant::now());

// Render in your ratatui loop
manager.tick(now);
manager.render(frame_area, &mut buffer, now);
```

## Documentation

For full documentation on the tui-vfx rendering engine, effect types, signal
system, and compositor, see the main project:

- [tui-vfx on crates.io](https://crates.io/crates/tui-vfx)
- [tui-vfx on GitHub](https://github.com/5ocworkshop/tui-vfx)

## License

MIT
