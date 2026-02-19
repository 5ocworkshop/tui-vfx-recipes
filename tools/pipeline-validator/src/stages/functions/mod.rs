// <FILE>tools/pipeline-validator/src/stages/functions/mod.rs</FILE> - <DESC>Stage validation functions module</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Pipeline performance debugging</WCTX>
// <CLOG>Add fnc_benchmark_stages for performance timing</CLOG>

pub mod fnc_benchmark_stages;
pub mod fnc_count_buffer_cells;
pub mod fnc_sample_buffer_cells;
pub mod fnc_validate_output;
pub mod fnc_validate_profile;
pub mod fnc_validate_render;
pub mod fnc_validate_shader;
pub mod fnc_validate_stages;

pub use fnc_benchmark_stages::benchmark_stages;
pub use fnc_count_buffer_cells::count_buffer_cells;
pub use fnc_sample_buffer_cells::{sample_buffer_cells, sample_buffer_cells_at};
pub use fnc_validate_output::validate_output;
pub use fnc_validate_profile::validate_profile;
pub use fnc_validate_render::validate_render;
pub use fnc_validate_shader::validate_shader;
pub use fnc_validate_stages::validate_stages;

// <FILE>tools/pipeline-validator/src/stages/functions/mod.rs</FILE> - <DESC>Stage validation functions module</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
