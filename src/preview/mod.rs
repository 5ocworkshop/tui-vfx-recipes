// <FILE>src/preview/mod.rs</FILE> - <DESC>Preview module for animation playback</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Pipeline stage inspection implementation</WCTX>
// <CLOG>Export render_preview_item_inspected for compositor inspection</CLOG>

//! Preview module for animation playback.
//!
//! This module provides a standardized preview pipeline that can be used
//! by all tools and examples to ensure accurate and consistent animation
//! playback behavior.
//!
//! # Components
//!
//! - [`PreviewItem`] - An item that implements `Animated` for use with `AnimationManager`
//! - [`PreviewManager`] - Wraps `AnimationManager<PreviewItem>` with rendering capabilities
//! - [`preview_from_recipe_config`] - Builds a `PreviewItem` from a V2 recipe config
//! - [`render_preview_item`] - Renders a preview item to a buffer
//!
//! # Usage
//!
//! ```ignore
//! use tui_vfx_recipes::preview::{PreviewManager, preview_from_recipe_config};
//!
//! let mut manager = PreviewManager::new();
//! let item = preview_from_recipe_config(&config);
//! manager.add(item, Instant::now());
//!
//! // In render loop:
//! manager.tick(now);
//! manager.render(frame_area, buffer, now);
//! ```

pub mod cls_preview_item;
pub mod cls_preview_manager;
pub mod fnc_append_cursor_if_visible;
pub mod fnc_preview_from_config;
pub mod fnc_render_preview_item;

pub use cls_preview_item::PreviewItem;
pub use cls_preview_manager::PreviewManager;
pub use fnc_append_cursor_if_visible::append_cursor_if_visible;
pub use fnc_preview_from_config::{preview_for_recipe_id, preview_from_recipe_config};
pub use fnc_render_preview_item::{render_preview_item, render_preview_item_inspected};

// <FILE>src/preview/mod.rs</FILE> - <DESC>Preview module for animation playback</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
