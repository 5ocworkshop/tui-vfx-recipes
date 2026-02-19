// <FILE>tools/pipeline-validator/src/stages/functions/fnc_count_buffer_cells.rs</FILE> - <DESC>Count non-empty cells in buffer</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Extracted from output.rs for OFPF compliance</CLOG>

use ratatui::buffer::Buffer;

/// Count cells that have non-empty content.
pub fn count_buffer_cells(buffer: &Buffer) -> usize {
    let mut count = 0;
    for y in buffer.area.top()..buffer.area.bottom() {
        for x in buffer.area.left()..buffer.area.right() {
            if let Some(cell) = buffer.cell((x, y)) {
                if !cell.symbol().trim().is_empty() {
                    count += 1;
                }
            }
        }
    }
    count
}

// <FILE>tools/pipeline-validator/src/stages/functions/fnc_count_buffer_cells.rs</FILE> - <DESC>Count non-empty cells in buffer</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
