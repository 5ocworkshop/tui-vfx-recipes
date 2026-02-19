// <FILE>src/prelude.rs</FILE> - <DESC>Convenience re-exports for common types</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Provide single-import access to all commonly needed types</WCTX>
// <CLOG>Initial creation - comprehensive prelude for tui-vfx-recipes users</CLOG>

//! # Prelude
//!
//! Convenience module that re-exports commonly used types from tui-vfx-recipes
//! and its dependencies.
//!
//! ## Usage
//!
//! ```rust
//! use tui_vfx_recipes::prelude::*;
//! ```
//!
//! This gives you access to:
//! - Recipe loading and management types
//! - Preview and animation management
//! - Grid, Cell, Color, and styling types
//! - Anchor and geometry types
//! - Compositor pipeline types (masks, filters, samplers)
//! - Theme and appearance configuration

// =============================================================================
// Recipe System
// =============================================================================

pub use crate::recipe::{Recipe, RecipeError, load, parse};
pub use crate::registry::RecipeRegistry;

// =============================================================================
// Preview & Animation Management
// =============================================================================

pub use crate::manager::AnimationManager;
pub use crate::preview::{
    PreviewItem, PreviewManager, preview_for_recipe_id, preview_from_recipe_config,
    render_preview_item,
};
pub use crate::state::{AnimationPhase, LifecycleState};
pub use crate::traits::Animated;

// =============================================================================
// Theme & Appearance
// =============================================================================

pub use crate::theme::{
    AppearanceConfig, BordersConfig, HasAppearance, PaddingConfig, Theme, TitleConfig,
    TitlePosition,
};

// =============================================================================
// Animation Types
// =============================================================================

pub use crate::types::{
    Animation, AnimationProfile, AutoDismiss, OverflowPolicy, SlideBorderTrimPolicy,
    SlideExitDirection, StackingPolicy,
};

// =============================================================================
// Rendering
// =============================================================================

pub use crate::rendering::{
    RatatuiBufferAdapter, RatatuiBufferSnapshot, RenderPlanItem, render_animated,
    render_animated_with_appearance, render_animated_with_theme, render_manager_items,
};

// =============================================================================
// Interactions
// =============================================================================

pub use crate::interactions::{GeometryOverrides, StateCompositionMode, StateStyleConfig};

// =============================================================================
// Inspector
// =============================================================================

pub use crate::inspector::{InspectorContext, PipelineInspector};

// =============================================================================
// Foundation Types (from tui-vfx-types)
// =============================================================================

pub use tui_vfx_types::{
    // Geometry
    Anchor,
    // Grid and cells
    Cell,
    // Colors and styling
    Color,
    Grid,
    GridExt,
    Modifiers,
    OwnedGrid,
    Point,
    Rect,
    Size,
    Style,
};

// =============================================================================
// Geometry & Positioning (from tui-vfx-geometry)
// =============================================================================

pub use tui_vfx_geometry::{
    anchors::anchored_rect,
    types::{SignedRect, SlideDirection, SnappingStrategy},
};

// =============================================================================
// Compositor Pipeline (from tui-vfx-compositor)
// =============================================================================

pub use tui_vfx_compositor::{
    // Context
    context::cls_compositor_ctx::CompositorCtx,
    // Pipeline API
    pipeline::{CompositionOptions, render_pipeline},
    // Effect specs
    types::{FilterSpec, MaskCombineMode, MaskSpec, SamplerSpec},
};

// =============================================================================
// Style Effects (from tui-vfx-style)
// =============================================================================

pub use tui_vfx_style::models::{StyleEffect, StyleRegion};

// =============================================================================
// Content Effects (from tui-vfx-content)
// =============================================================================

pub use tui_vfx_content::types::ContentEffect;

// =============================================================================
// Ratatui Re-exports (commonly needed alongside our types)
// =============================================================================

pub use ratatui::{
    buffer::Buffer,
    layout::Rect as RatatuiRect,
    style::{Color as RatatuiColor, Modifier, Style as RatatuiStyle},
};

// <FILE>src/prelude.rs</FILE> - <DESC>Convenience re-exports for common types</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
