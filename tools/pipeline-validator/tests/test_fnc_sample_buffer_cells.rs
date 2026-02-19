// <FILE>tools/pipeline-validator/tests/stages/functions/test_fnc_sample_buffer_cells.rs</FILE> - <DESC>Tests for buffer cell sampling</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline validator OFPF restructuring</WCTX>
// <CLOG>Initial creation with TDD tests</CLOG>

use pipeline_validator::stages::StageResult;
use pipeline_validator::stages::functions::fnc_sample_buffer_cells::sample_buffer_cells;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

#[test]
fn test_sample_adds_header() {
    let area = Rect::new(0, 0, 10, 5);
    let buffer = Buffer::empty(area);
    let result = StageResult::pass("test");

    let result = sample_buffer_cells(&buffer, 10, 5, result);

    let has_header = result.details.iter().any(|d| d.contains("Sample cells:"));
    assert!(
        has_header,
        "Should add sample cells header: {:?}",
        result.details
    );
}

#[test]
fn test_sample_includes_corners() {
    let area = Rect::new(0, 0, 10, 5);
    let buffer = Buffer::empty(area);
    let result = StageResult::pass("test");

    let result = sample_buffer_cells(&buffer, 10, 5, result);

    let has_top_left = result.details.iter().any(|d| d.contains("top-left"));
    let has_top_right = result.details.iter().any(|d| d.contains("top-right"));
    let has_bottom_left = result.details.iter().any(|d| d.contains("bottom-left"));
    let has_bottom_right = result.details.iter().any(|d| d.contains("bottom-right"));
    let has_center = result.details.iter().any(|d| d.contains("center"));

    assert!(has_top_left, "Should sample top-left");
    assert!(has_top_right, "Should sample top-right");
    assert!(has_bottom_left, "Should sample bottom-left");
    assert!(has_bottom_right, "Should sample bottom-right");
    assert!(has_center, "Should sample center");
}

#[test]
fn test_sample_reports_cell_properties() {
    let area = Rect::new(0, 0, 10, 5);
    let mut buffer = Buffer::empty(area);
    buffer.cell_mut((0, 0)).unwrap().set_symbol("X");
    let result = StageResult::pass("test");

    let result = sample_buffer_cells(&buffer, 10, 5, result);

    let has_symbol = result.details.iter().any(|d| d.contains("'X'"));
    assert!(
        has_symbol,
        "Should report cell symbol: {:?}",
        result.details
    );
}

// <FILE>tools/pipeline-validator/tests/stages/functions/test_fnc_sample_buffer_cells.rs</FILE> - <DESC>Tests for buffer cell sampling</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
