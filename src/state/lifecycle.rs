// <FILE>src/state/lifecycle.rs</FILE> - <DESC>Generic lifecycle state machine</DESC>
// <VERS>VERSION: 0.2.0 - 2025-12-23</VERS>
// <WCTX>WG8: Signal Time Synchronization</WCTX>
// <CLOG>Added loop_progress() method for cyclical loop time calculation</CLOG>

use crate::traits::Animated;
use crate::types::{Animation, AutoDismiss};
use std::time::{Duration, Instant};
use tui_vfx_core::TimeSpec;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPhase {
    Entering,
    Dwelling,
    Exiting,
    Finished,
}
#[derive(Debug, Clone)]
pub struct LifecycleState<T: Animated> {
    pub id: u64,
    pub item: T,
    pub phase: AnimationPhase,
    created_at: Instant,
    enter_start: Instant,
    dwell_start: Option<Instant>,
    exit_start: Option<Instant>,
}
impl<T: Animated> LifecycleState<T> {
    pub fn new(id: u64, item: T, now: Instant) -> Self {
        let (phase, dwell_start) = if item.animation() == Animation::None {
            (AnimationPhase::Dwelling, Some(now))
        } else {
            (AnimationPhase::Entering, None)
        };
        Self {
            id,
            item,
            phase,
            created_at: now,
            enter_start: now,
            dwell_start,
            exit_start: None,
        }
    }
    pub fn created_at(&self) -> Instant {
        self.created_at
    }
    pub fn is_finished(&self) -> bool {
        self.phase == AnimationPhase::Finished
    }
    fn enter_duration(&self) -> Duration {
        Duration::from_millis(self.item.profile().enter.duration_ms)
    }
    fn exit_duration(&self) -> Duration {
        Duration::from_millis(self.item.profile().exit.duration_ms)
    }
    fn effective_display_duration(&self, default_display: Duration) -> Option<Duration> {
        match self.item.auto_dismiss() {
            AutoDismiss::Manual => None,
            AutoDismiss::After(d) if d > Duration::ZERO => Some(d),
            AutoDismiss::After(_) => Some(default_display),
        }
    }
    pub fn dismiss(&mut self, now: Instant) {
        if self.phase == AnimationPhase::Finished {
            return;
        }
        let exit = self.item.exit_animation().unwrap_or(self.item.animation());
        if exit == Animation::None {
            self.phase = AnimationPhase::Finished;
            return;
        }
        self.phase = AnimationPhase::Exiting;
        self.exit_start = Some(now);
    }
    pub fn active_animation(&self) -> Animation {
        match self.phase {
            AnimationPhase::Exiting => self.item.exit_animation().unwrap_or(self.item.animation()),
            _ => self.item.animation(),
        }
    }
    pub fn tick(&mut self, now: Instant, default_display: Duration) {
        match self.phase {
            AnimationPhase::Entering => {
                let ts = TimeSpec {
                    start: self.enter_start,
                    now,
                    duration: self.enter_duration(),
                };
                if ts.progress() >= 1.0 {
                    self.phase = AnimationPhase::Dwelling;
                    self.dwell_start = Some(now);
                }
            }
            AnimationPhase::Dwelling => {
                let Some(start) = self.dwell_start else {
                    self.dwell_start = Some(now);
                    return;
                };
                if let Some(duration) = self.effective_display_duration(default_display) {
                    if now.saturating_duration_since(start) >= duration {
                        match self.item.exit_animation().unwrap_or(self.item.animation()) {
                            Animation::None => self.phase = AnimationPhase::Finished,
                            _ => {
                                self.phase = AnimationPhase::Exiting;
                                self.exit_start = Some(now);
                            }
                        }
                    }
                }
            }
            AnimationPhase::Exiting => {
                let start = self.exit_start.get_or_insert(now);
                let ts = TimeSpec {
                    start: *start,
                    now,
                    duration: self.exit_duration(),
                };
                if ts.progress() >= 1.0 {
                    self.phase = AnimationPhase::Finished;
                }
            }
            AnimationPhase::Finished => {}
        }
    }
    pub fn phase_progress(&self, now: Instant) -> f64 {
        match self.phase {
            AnimationPhase::Entering => TimeSpec {
                start: self.enter_start,
                now,
                duration: self.enter_duration(),
            }
            .progress(),
            AnimationPhase::Dwelling => {
                if let Some(start) = self.dwell_start {
                    // Use a 4s default for progress calculation if Manual, so effects still play.
                    let duration = self
                        .effective_display_duration(Duration::from_secs(4))
                        .unwrap_or(Duration::from_secs(4));
                    TimeSpec {
                        start,
                        now,
                        duration,
                    }
                    .progress()
                } else {
                    0.0
                }
            }
            AnimationPhase::Exiting => {
                let start = self.exit_start.unwrap_or(self.enter_start);
                TimeSpec {
                    start,
                    now,
                    duration: self.exit_duration(),
                }
                .progress()
            }
            AnimationPhase::Finished => 1.0,
        }
    }

    /// Calculate loop time (repeating 0.0-1.0) based on loop_period configuration.
    /// Returns None if looping is disabled (loop_period is None or Duration::ZERO).
    ///
    /// The loop cycles from the animation's creation time, so it continues
    /// smoothly across phase transitions without resetting.
    pub fn loop_progress(&self, now: Instant) -> Option<f64> {
        let loop_period = self.item.profile().loop_period?;

        if loop_period == Duration::ZERO {
            return None;
        }

        let elapsed = now.saturating_duration_since(self.created_at);
        let loop_ms = loop_period.as_secs_f64() * 1000.0;
        let elapsed_ms = elapsed.as_secs_f64() * 1000.0;

        if loop_ms <= 0.0 {
            return None;
        }

        // Calculate repeating cycle: elapsed_ms % loop_ms / loop_ms
        let normalized = (elapsed_ms % loop_ms) / loop_ms;
        Some(normalized.clamp(0.0, 1.0))
    }
}

// <FILE>src/state/lifecycle.rs</FILE> - <DESC>Generic lifecycle state machine</DESC>
// <VERS>END OF VERSION: 0.2.0 - 2025-12-23</VERS>
