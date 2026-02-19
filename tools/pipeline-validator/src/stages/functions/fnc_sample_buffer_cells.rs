// <FILE>tools/pipeline-validator/src/stages/functions/fnc_sample_buffer_cells.rs</FILE> - <DESC>Sample buffer cells at key positions</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Extracted from output.rs for OFPF compliance</CLOG>

use ratatui::buffer::Buffer;

use crate::stages::StageResult;

/// Sample cells at key positions and add to result.
pub fn sample_buffer_cells(
    buffer: &Buffer,
    width: u16,
    height: u16,
    result: StageResult,
) -> StageResult {
    sample_buffer_cells_at(buffer, 0, 0, width, height, result)
}

/// Sample cells at key positions with an offset and add to result.
pub fn sample_buffer_cells_at(
    buffer: &Buffer,
    offset_x: u16,
    offset_y: u16,
    width: u16,
    height: u16,
    mut result: StageResult,
) -> StageResult {
    result = result.with_detail("Sample cells:".to_string());

    let samples = [
        (offset_x, offset_y, "top-left"),
        (offset_x + width.saturating_sub(1), offset_y, "top-right"),
        (offset_x, offset_y + height.saturating_sub(1), "bottom-left"),
        (
            offset_x + width.saturating_sub(1),
            offset_y + height.saturating_sub(1),
            "bottom-right",
        ),
        (offset_x + width / 2, offset_y + height / 2, "center"),
        // Content area samples (inside border)
        (offset_x + 5, offset_y + 1, "content-start"),
        (offset_x + 10, offset_y + 1, "content-mid1"),
        (offset_x + 20, offset_y + 1, "content-mid2"),
    ];

    for (x, y, name) in samples {
        if let Some(cell) = buffer.cell((x, y)) {
            result = result.with_detail(format!(
                "  [{}] ({},{}): '{}' fg={:?} bg={:?}",
                name,
                x,
                y,
                cell.symbol(),
                cell.fg,
                cell.bg
            ));
        }
    }

    // Dump first few non-empty cells to see where content actually is
    let mut non_empty_samples = Vec::new();
    for y in offset_y..(offset_y + height) {
        for x in offset_x..(offset_x + width) {
            if let Some(cell) = buffer.cell((x, y)) {
                let sym = cell.symbol();
                if !sym.trim().is_empty() && non_empty_samples.len() < 10 {
                    non_empty_samples.push(format!(
                        "({},{}): '{}' fg={:?} bg={:?}",
                        x, y, sym, cell.fg, cell.bg
                    ));
                }
            }
        }
    }
    if !non_empty_samples.is_empty() {
        result = result.with_detail("First non-empty cells:".to_string());
        for sample in non_empty_samples {
            result = result.with_detail(format!("  {}", sample));
        }
    }

    result
}

// <FILE>tools/pipeline-validator/src/stages/functions/fnc_sample_buffer_cells.rs</FILE> - <DESC>Sample buffer cells at key positions</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
