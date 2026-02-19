// <FILE>src/manager/mod.rs</FILE> - <DESC>AnimationManager and layout orchestration</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>WG9: Layout Module Extraction (OFPF Compliance)</WCTX>
// <CLOG>Renamed fnc_render_plan to orc_render_plan; added extracted function modules</CLOG>

use crate::rendering::RenderPlanItem;
use crate::state::LifecycleState;
use crate::traits::Animated;
use crate::types::{OverflowPolicy, StackingPolicy};
use ratatui::layout::Rect;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tui_vfx_geometry::types::Anchor;
pub mod fnc_calculate_motion_rect;
pub mod fnc_compute_dwell_rect;
pub mod fnc_populate_effects;
pub mod fnc_resolve_anchor;
pub mod orc_render_plan;
#[derive(Debug)]
pub struct AnimationManager<T: Animated> {
    states: HashMap<u64, LifecycleState<T>>,
    by_anchor: HashMap<Anchor, Vec<u64>>,
    next_id: u64,
    pub max_concurrent_per_anchor: usize,
    pub overflow: OverflowPolicy,
    pub default_display_time: Duration,
    pub stacking_policy: StackingPolicy,
}
impl<T: Animated> Default for AnimationManager<T> {
    fn default() -> Self {
        Self {
            states: HashMap::new(),
            by_anchor: HashMap::new(),
            next_id: 0,
            max_concurrent_per_anchor: 5,
            overflow: OverflowPolicy::default(),
            default_display_time: Duration::from_secs(4),
            stacking_policy: StackingPolicy::default(),
        }
    }
}
impl<T: Animated> AnimationManager<T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the global stacking policy for all anchors (builder API)
    pub fn with_stacking_policy(mut self, policy: StackingPolicy) -> Self {
        self.stacking_policy = policy;
        self
    }

    /// Set stacking policy (mutable API)
    pub fn set_stacking_policy(&mut self, policy: StackingPolicy) {
        self.stacking_policy = policy;
    }
    pub fn add(&mut self, item: T, now: Instant) -> Option<u64> {
        let anchor = item.anchor();
        if !self.enforce_limit(anchor) {
            return None;
        }
        let id = self.allocate_id()?;
        let state = LifecycleState::new(id, item, now);
        self.states.insert(id, state);
        self.by_anchor.entry(anchor).or_default().push(id);
        Some(id)
    }
    pub fn remove(&mut self, id: u64) -> bool {
        if let Some(state) = self.states.remove(&id) {
            if let Some(ids) = self.by_anchor.get_mut(&state.item.anchor()) {
                ids.retain(|&existing| existing != id);
            }
            true
        } else {
            false
        }
    }
    pub fn clear(&mut self) {
        self.states.clear();
        self.by_anchor.clear();
    }
    pub fn dismiss(&mut self, id: u64, now: Instant) -> bool {
        if let Some(state) = self.states.get_mut(&id) {
            state.dismiss(now);
            true
        } else {
            false
        }
    }
    pub fn tick(&mut self, now: Instant) {
        let ids: Vec<u64> = self.states.keys().copied().collect();
        for id in ids {
            if let Some(s) = self.states.get_mut(&id) {
                s.tick(now, self.default_display_time);
            }
        }
        let finished: Vec<u64> = self
            .states
            .iter()
            .filter_map(|(id, s)| s.is_finished().then_some(*id))
            .collect();
        for id in finished {
            self.remove(id);
        }
    }
    pub fn render_plan(&self, frame_area: Rect, now: Instant) -> Vec<RenderPlanItem<'_>> {
        orc_render_plan::render_plan(
            &self.states,
            &self.by_anchor,
            frame_area,
            now,
            None,
            self.stacking_policy,
        )
    }

    /// Render plan at a specific t value (for scrub/preview mode).
    /// When override_t is Some, bypasses time-based progress calculation.
    pub fn render_plan_at_t(
        &self,
        frame_area: Rect,
        now: Instant,
        override_t: Option<f64>,
    ) -> Vec<RenderPlanItem<'_>> {
        orc_render_plan::render_plan(
            &self.states,
            &self.by_anchor,
            frame_area,
            now,
            override_t,
            self.stacking_policy,
        )
    }
    pub fn states(&self) -> impl Iterator<Item = &LifecycleState<T>> {
        self.states.values()
    }
    pub fn get_state(&self, id: u64) -> Option<&LifecycleState<T>> {
        self.states.get(&id)
    }
    pub fn by_anchor(&self) -> &HashMap<Anchor, Vec<u64>> {
        &self.by_anchor
    }
    fn enforce_limit(&mut self, anchor: Anchor) -> bool {
        let ids = self.by_anchor.get(&anchor).map(|v| v.len()).unwrap_or(0);
        if ids < self.max_concurrent_per_anchor {
            return true;
        }
        match self.overflow {
            OverflowPolicy::Reject => false,
            OverflowPolicy::DiscardOldest => {
                if let Some(id) = self.find_oldest_at_anchor(anchor) {
                    self.remove(id);
                }
                true
            }
            OverflowPolicy::DiscardNewest => {
                if let Some(id) = self.find_newest_at_anchor(anchor) {
                    self.remove(id);
                }
                true
            }
        }
    }
    fn find_oldest_at_anchor(&self, anchor: Anchor) -> Option<u64> {
        let ids = self.by_anchor.get(&anchor)?;
        ids.iter()
            .filter_map(|id| self.states.get(id).map(|s| (*id, s.created_at())))
            .min_by_key(|(_, t)| *t)
            .map(|(id, _)| id)
    }
    fn find_newest_at_anchor(&self, anchor: Anchor) -> Option<u64> {
        let ids = self.by_anchor.get(&anchor)?;
        ids.iter()
            .filter_map(|id| self.states.get(id).map(|s| (*id, s.created_at())))
            .max_by_key(|(_, t)| *t)
            .map(|(id, _)| id)
    }

    fn allocate_id(&mut self) -> Option<u64> {
        let start = self.next_id;
        loop {
            let id = self.next_id;
            self.next_id = self.next_id.wrapping_add(1);
            if !self.states.contains_key(&id) {
                return Some(id);
            }
            if self.next_id == start {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AnimationManager;
    use crate::preview::PreviewItem;
    use std::time::Instant;

    #[test]
    fn add_skips_existing_ids_after_wrap() {
        let mut manager = AnimationManager::new();
        let now = Instant::now();
        let first = manager.add(PreviewItem::new("first"), now).unwrap();

        manager.next_id = first;
        let second = manager.add(PreviewItem::new("second"), now).unwrap();

        assert_ne!(first, second);
        assert!(manager.states.contains_key(&first));
        assert!(manager.states.contains_key(&second));
    }
}

// <FILE>src/manager/mod.rs</FILE> - <DESC>AnimationManager and layout orchestration</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
