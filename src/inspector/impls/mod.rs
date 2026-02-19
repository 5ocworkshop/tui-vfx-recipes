// <FILE>src/inspector/impls/mod.rs</FILE> - <DESC>Built-in inspector implementations</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Pipeline stage inspection implementation</WCTX>
// <CLOG>Add StageInspector for comprehensive pipeline stage capture</CLOG>

mod cls_diff_inspector;
mod cls_stage_inspector;
mod cls_trace_inspector;
mod cls_validation_inspector;

pub use cls_diff_inspector::{CellChange, DiffInspector};
pub use cls_stage_inspector::{
    FilterResult, MaskResult, SamplerResult, ShaderResult, StageInspector,
};
pub use cls_trace_inspector::{TraceInspector, TraceVerbosity};
pub use cls_validation_inspector::ValidationInspector;

// <FILE>src/inspector/impls/mod.rs</FILE> - <DESC>Built-in inspector implementations</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
