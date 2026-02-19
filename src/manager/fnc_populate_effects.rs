// <FILE>src/manager/fnc_populate_effects.rs</FILE> - <DESC>Populate phase-based effects on RenderPlanItem</DESC>
// <VERS>VERSION: 2.0.1</VERS>
// <WCTX>Clippy cleanup for populate_effects tests</WCTX>
// <CLOG>Inline dummy render plan creation to avoid let-and-return warning</CLOG>

use crate::rendering::RenderPlanItem;
use crate::state::AnimationPhase;
use crate::traits::Animated;
use tui_vfx_compositor::types::MaskCombineMode;

/// Populate effects (masks, filters, samplers) on a RenderPlanItem based on animation phase.
///
/// # Arguments
/// * `plan_item` - The render plan item to populate
/// * `item` - The animated item providing effect specifications
/// * `phase` - Current animation phase (Entering/Dwelling/Exiting/Finished)
/// * `mask_combine_mode` - How multiple masks should be combined
pub fn populate_effects<'a, T: Animated>(
    plan_item: &mut RenderPlanItem<'a>,
    item: &'a T,
    phase: AnimationPhase,
    mask_combine_mode: MaskCombineMode,
) {
    plan_item.mask_combine_mode = mask_combine_mode;

    match phase {
        AnimationPhase::Entering => {
            plan_item.masks = item.enter_masks();
            plan_item.filters = item.enter_filters();
            if let Some(sampler) = item.enter_sampler() {
                plan_item.sampler_spec = Some(sampler.clone());
            }
        }
        AnimationPhase::Dwelling => {
            plan_item.masks = item.dwell_masks();
            plan_item.filters = item.dwell_filters();
            if let Some(sampler) = item.dwell_sampler() {
                plan_item.sampler_spec = Some(sampler.clone());
            }
        }
        AnimationPhase::Exiting => {
            plan_item.masks = item.exit_masks();
            plan_item.filters = item.exit_filters();
            if let Some(sampler) = item.exit_sampler() {
                plan_item.sampler_spec = Some(sampler.clone());
            }
        }
        AnimationPhase::Finished => {
            // No effects during finished phase
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compat::ratatui_rect_to_vfx;
    use crate::rendering::RenderPlanItem;
    use crate::state::AnimationPhase;
    use crate::types::{
        Animation, AnimationProfile, AutoDismiss, SlideBorderTrimPolicy, SlideExitDirection,
    };
    use ratatui::layout::Rect;
    use std::borrow::Cow;
    use std::sync::LazyLock;
    use tui_vfx_compositor::types::{FilterSpec, MaskSpec};
    use tui_vfx_geometry::transitions::SlidePath;
    use tui_vfx_geometry::types::{Anchor, SignedRect, SlideDirection};

    fn dummy_plan_item() -> RenderPlanItem<'static> {
        RenderPlanItem::new(
            1,
            Anchor::TopLeft,
            0.0,
            0.0,
            0,
            0,
            0,
            0,
            AnimationPhase::Entering,
            Animation::Slide,
            Rect::new(0, 0, 10, 10),
            Rect::new(0, 0, 10, 10),
            SignedRect::new(0, 0, 10, 10),
            0.5,
        )
    }

    // Minimal mock implementing all required Animated trait methods
    struct MockAnimated;

    static MOCK_PROFILE: LazyLock<AnimationProfile> = LazyLock::new(AnimationProfile::default);

    impl Animated for MockAnimated {
        fn anchor(&self) -> Anchor {
            Anchor::TopLeft
        }
        fn profile(&self) -> &AnimationProfile {
            &MOCK_PROFILE
        }
        fn animation(&self) -> Animation {
            Animation::Slide
        }
        fn exit_animation(&self) -> Option<Animation> {
            None
        }
        fn auto_dismiss(&self) -> AutoDismiss {
            AutoDismiss::default()
        }
        fn width(&self) -> u16 {
            10
        }
        fn height(&self) -> u16 {
            10
        }
        fn exterior_margin(&self) -> u16 {
            0
        }
        fn slide_direction(&self) -> SlideDirection {
            SlideDirection::Default
        }
        fn slide_exit_direction(&self) -> SlideExitDirection {
            SlideExitDirection::SameAsEnter
        }
        fn slide_border_trim(&self) -> SlideBorderTrimPolicy {
            SlideBorderTrimPolicy::default()
        }
        fn slide_path(&self, _frame: Rect, dwell: Rect) -> SlidePath {
            SlidePath::toast(SignedRect::from(ratatui_rect_to_vfx(dwell)))
        }
    }

    #[test]
    fn test_entering_phase_sets_masks_and_filters() {
        let mut item = dummy_plan_item();
        let mock = MockAnimated;

        populate_effects(
            &mut item,
            &mock,
            AnimationPhase::Entering,
            MaskCombineMode::default(),
        );

        // Default implementation returns empty vecs (no masks configured)
        assert!(item.masks.is_empty());
        assert!(item.filters.is_empty());
    }

    #[test]
    fn test_finished_phase_no_effects() {
        let mut item = dummy_plan_item();
        let mock = MockAnimated;

        populate_effects(
            &mut item,
            &mock,
            AnimationPhase::Finished,
            MaskCombineMode::default(),
        );

        // Finished phase doesn't modify effects
        assert!(item.masks.is_empty());
        assert!(item.filters.is_empty());
    }

    #[test]
    fn test_mask_combine_mode_is_set() {
        let mut item = dummy_plan_item();
        let mock = MockAnimated;

        populate_effects(
            &mut item,
            &mock,
            AnimationPhase::Dwelling,
            MaskCombineMode::Any,
        );

        assert_eq!(item.mask_combine_mode, MaskCombineMode::Any);
    }

    #[test]
    fn uses_borrowed_slices_for_multi_effects() {
        let masks = [MaskSpec::Diamond { soft_edge: false }];
        let filters = [FilterSpec::Dim {
            factor: mixed_signals::prelude::SignalOrFloat::Static(0.5),
            apply_to: tui_vfx_compositor::types::ApplyTo::Both,
        }];

        struct BorrowingAnimated<'a> {
            masks: &'a [MaskSpec],
            filters: &'a [FilterSpec],
        }

        static PROFILE: LazyLock<AnimationProfile> = LazyLock::new(AnimationProfile::default);

        impl Animated for BorrowingAnimated<'_> {
            fn anchor(&self) -> Anchor {
                Anchor::TopLeft
            }
            fn profile(&self) -> &AnimationProfile {
                &PROFILE
            }
            fn animation(&self) -> Animation {
                Animation::Slide
            }
            fn exit_animation(&self) -> Option<Animation> {
                None
            }
            fn auto_dismiss(&self) -> AutoDismiss {
                AutoDismiss::default()
            }
            fn width(&self) -> u16 {
                10
            }
            fn height(&self) -> u16 {
                10
            }
            fn exterior_margin(&self) -> u16 {
                0
            }
            fn slide_direction(&self) -> SlideDirection {
                SlideDirection::Default
            }
            fn slide_exit_direction(&self) -> SlideExitDirection {
                SlideExitDirection::SameAsEnter
            }
            fn slide_border_trim(&self) -> SlideBorderTrimPolicy {
                SlideBorderTrimPolicy::default()
            }
            fn slide_path(&self, _frame: Rect, dwell: Rect) -> SlidePath {
                SlidePath::toast(SignedRect::from(ratatui_rect_to_vfx(dwell)))
            }
            fn enter_masks(&self) -> Cow<'_, [MaskSpec]> {
                Cow::Borrowed(self.masks)
            }
            fn enter_filters(&self) -> Cow<'_, [FilterSpec]> {
                Cow::Borrowed(self.filters)
            }
        }

        let mut item = dummy_plan_item();
        let animated = BorrowingAnimated {
            masks: &masks,
            filters: &filters,
        };

        populate_effects(
            &mut item,
            &animated,
            AnimationPhase::Entering,
            MaskCombineMode::All,
        );

        assert!(matches!(item.masks, Cow::Borrowed(_)));
        assert!(matches!(item.filters, Cow::Borrowed(_)));
    }
}

// <FILE>src/manager/fnc_populate_effects.rs</FILE> - <DESC>Populate phase-based effects on RenderPlanItem</DESC>
// <VERS>END OF VERSION: 2.0.1</VERS>
