// <FILE>src/inspector/mod.rs</FILE> - <DESC>Pipeline inspection infrastructure</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools implementation</WCTX>
// <CLOG>Initial creation of inspector module</CLOG>

mod cls_inspector_context;
pub mod impls;
mod traits;

pub use cls_inspector_context::InspectorContext;
pub use traits::PipelineInspector;

// <FILE>src/inspector/mod.rs</FILE> - <DESC>Pipeline inspection infrastructure</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
