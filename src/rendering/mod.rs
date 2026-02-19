// <FILE>src/rendering/mod.rs</FILE> - <DESC>Rendering module root</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Compositor integration - add ratatui Buffer adapter for Grid trait</WCTX>
// <CLOG>Add RatatuiBufferAdapter and RatatuiBufferSnapshot for compositor pipeline</CLOG>

pub mod cls_ratatui_buffer_adapter;
pub mod fnc_render_animated;
pub mod fnc_render_animated_with_theme;
pub mod fnc_render_manager_items;
pub mod types;

pub use cls_ratatui_buffer_adapter::{RatatuiBufferAdapter, RatatuiBufferSnapshot};
pub use fnc_render_animated::render_animated;
pub use fnc_render_animated_with_theme::{
    render_animated_with_appearance, render_animated_with_appearance_inspected,
    render_animated_with_inspector, render_animated_with_theme,
};
pub use fnc_render_manager_items::render_manager_items;
pub use types::RenderPlanItem;

// <FILE>src/rendering/mod.rs</FILE> - <DESC>Rendering module root</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
