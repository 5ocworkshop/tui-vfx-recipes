// <FILE>tools/pipeline-validator/src/rules/functions/mod.rs</FILE> - <DESC>Rule evaluation functions module</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Pipeline debugging tools - color contrast validation</WCTX>
// <CLOG>Add fnc_color_utils for color contrast validation functions</CLOG>

pub mod fnc_color_utils;
pub mod fnc_compare_values;
pub mod fnc_evaluate_condition;
pub mod fnc_evaluate_field;
pub mod fnc_evaluate_function;
pub mod fnc_evaluate_value;
pub mod fnc_get_json_field;
pub mod fnc_interpolate_message;
pub mod fnc_split_on_operator;

// Re-export main public functions
#[allow(unused_imports)]
pub use fnc_color_utils::{color_distance, is_dark, luminance, max_luminance, parse_color};
pub use fnc_evaluate_condition::evaluate_condition;
pub use fnc_interpolate_message::interpolate_message;

// <FILE>tools/pipeline-validator/src/rules/functions/mod.rs</FILE> - <DESC>Rule evaluation functions module</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
