// <FILE>src/preview/fnc_append_cursor_if_visible.rs</FILE> - <DESC>Append typewriter cursor if visible based on blink and visibility signals</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>feat-20251224-170136: Complete signal-driven content effects</WCTX>
// <CLOG>Initial creation - cursor visibility and blink logic with signal evaluation</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_content::types::TypewriterCursor;

/// Append cursor to text if it should be visible
///
/// Evaluates cursor visibility signals and blink timing to determine
/// whether to append the cursor character to the transformed text.
///
/// # Arguments
/// * `text` - The transformed text (after Typewriter effect applied)
/// * `cursor` - Cursor configuration with signal-driven parameters
/// * `progress` - Animation progress (0.0-1.0)
/// * `signal_ctx` - Signal context with timing and dimensions
///
/// # Returns
/// Text with cursor appended if visible, otherwise original text
pub fn append_cursor_if_visible(
    text: &str,
    cursor: &TypewriterCursor,
    progress: f64,
    signal_ctx: &SignalContext,
) -> String {
    // Empty cursor character means disabled
    if cursor.character.is_empty() {
        return text.to_string();
    }

    // Determine visibility based on progress and signals (fallback to visible on error)
    let visibility = if progress < 1.0 {
        cursor
            .show_while_typing
            .evaluate(progress, signal_ctx)
            .unwrap_or(1.0)
    } else {
        cursor
            .show_after_complete
            .evaluate(progress, signal_ctx)
            .unwrap_or(1.0)
    };

    if visibility <= 0.0 {
        return text.to_string(); // Invisible
    }

    // Calculate blink state (fallback to 0.0 = always visible on error)
    let blink_ms = f64::from(
        cursor
            .blink_interval
            .evaluate(progress, signal_ctx)
            .unwrap_or(0.0)
            .max(0.0),
    );

    let visible = if blink_ms <= 0.0 {
        true // Always visible if interval is 0 or negative
    } else {
        // Get elapsed time in milliseconds (absolute_t is Option<f64> in ms)
        let elapsed_ms = signal_ctx.absolute_t.unwrap_or(0.0);
        let cycle_ms = blink_ms * 2.0;
        let position_in_cycle = elapsed_ms % cycle_ms;
        position_in_cycle < blink_ms
    };

    if visible {
        // Alpha blending: threshold at 0.5 (< 0.5 = hidden, >= 0.5 = visible)
        let show_cursor = visibility >= 0.5;
        if show_cursor {
            format!("{}{}", text, cursor.character)
        } else {
            text.to_string()
        }
    } else {
        text.to_string()
    }
}

// <FILE>src/preview/fnc_append_cursor_if_visible.rs</FILE> - <DESC>Append typewriter cursor if visible based on blink and visibility signals</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
