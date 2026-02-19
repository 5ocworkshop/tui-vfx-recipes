// <FILE>src/types/mod.rs</FILE> - <DESC>Updated types module</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>WG7: Extensible Stacking Policy</WCTX>
// <CLOG>Added StackingPolicy enum for configurable layout modes</CLOG>

pub mod animation;
pub mod animation_profile;
pub mod auto_dismiss;
pub mod overflow_policy;
pub mod slide_border_trim_policy;
pub mod slide_exit_direction;
pub mod stacking_policy;
pub use animation::Animation;
pub use animation_profile::AnimationProfile;
pub use auto_dismiss::AutoDismiss;
pub use overflow_policy::OverflowPolicy;
pub use slide_border_trim_policy::SlideBorderTrimPolicy;
pub use slide_exit_direction::SlideExitDirection;
pub use stacking_policy::StackingPolicy;

// <FILE>src/types/mod.rs</FILE> - <DESC>Updated types module</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
