// <FILE>tools/pipeline-validator/tests/stages/functions/test_fnc_count_buffer_cells.rs</FILE> - <DESC>Tests for buffer cell counting</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Initial creation with TDD tests</CLOG>

use pipeline_validator::stages::count_buffer_cells;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

#[test]
fn test_count_empty_buffer() {
    let area = Rect::new(0, 0, 10, 5);
    let buffer = Buffer::empty(area);

    let count = count_buffer_cells(&buffer);

    assert_eq!(count, 0, "Empty buffer should have 0 non-empty cells");
}

#[test]
fn test_count_buffer_with_content() {
    let area = Rect::new(0, 0, 10, 5);
    let mut buffer = Buffer::empty(area);

    // Set some cells with content
    buffer.cell_mut((0, 0)).unwrap().set_symbol("A");
    buffer.cell_mut((1, 0)).unwrap().set_symbol("B");
    buffer.cell_mut((2, 0)).unwrap().set_symbol("C");

    let count = count_buffer_cells(&buffer);

    assert_eq!(count, 3, "Should count 3 non-empty cells");
}

#[test]
fn test_count_ignores_whitespace_only() {
    let area = Rect::new(0, 0, 10, 5);
    let mut buffer = Buffer::empty(area);

    // Set cells with whitespace
    buffer.cell_mut((0, 0)).unwrap().set_symbol(" ");
    buffer.cell_mut((1, 0)).unwrap().set_symbol("  ");
    buffer.cell_mut((2, 0)).unwrap().set_symbol("X");

    let count = count_buffer_cells(&buffer);

    assert_eq!(count, 1, "Should only count non-whitespace cells");
}

// <FILE>tools/pipeline-validator/tests/stages/functions/test_fnc_count_buffer_cells.rs</FILE> - <DESC>Tests for buffer cell counting</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
