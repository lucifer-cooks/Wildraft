use crate::comp;
use serde::{Deserialize, Serialize};
use specs::{Component, DerefFlaggedStorage};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Thirst {
    current: u32,
    base_max: u32,
    maximum: u32,
    regen_rate: f32,
}

impl Thirst {
    const MAX_SCALED_THIRST: u32 = (Self::MAX_THIRST as u32) * Self::SCALING_FACTOR_INT;
    const MAX_THIRST: u16 = u16::MAX - 1;
    const SCALING_FACTOR_FLOAT: f32 = 256.0;
    const SCALING_FACTOR_INT: u32 = 256;

    pub fn current(&self) -> f32 { self.current as f32 / Self::SCALING_FACTOR_FLOAT }

    pub fn base_max(&self) -> f32 { self.base_max as f32 / Self::SCALING_FACTOR_FLOAT }

    pub fn maximum(&self) -> f32 { self.maximum as f32 / Self::SCALING_FACTOR_FLOAT }

    pub fn fraction(&self) -> f32 { self.current() / self.maximum().max(1.0) }

    pub fn new(base_max_f: f32) -> Self {
        let base =
            (base_max_f * Self::SCALING_FACTOR_FLOAT).clamp(0.0, Self::MAX_THIRST as f32) as u32;
        Thirst {
            current: base,
            base_max: base,
            maximum: base,
            regen_rate: 0.0,
        }
    }

    /// Gradual decrease over time (thirst drains faster in heat/action)
    pub fn drain(&mut self, rate: f32, dt: f32) {
        if self.current > 0 {
            self.change_by(-rate * dt);
        }
    }

    /// Regeneration (drinking restores)
    pub fn regen(&mut self, amount: f32) { self.change_by(amount); }

    /// Slow natural regen only near water sources (minimal)
    pub fn tick_regen(&mut self, dt: f32) {
        if self.current < self.base_max {
            self.change_by(0.002 * dt);
        }
    }

    pub fn change_by(&mut self, change: f32) {
        self.current = (((self.current() + change).clamp(0.0, Self::MAX_THIRST as f32)
            * Self::SCALING_FACTOR_FLOAT) as u32)
            .min(self.maximum);
    }

    pub fn needs_water(&self) -> bool { self.current() < self.base_max() * 0.25 }

    pub fn is_critical(&self) -> bool { self.current() < self.base_max() * 0.1 }

    pub fn refresh(&mut self) { self.current = self.base_max; }
}

impl Component for Thirst {
    type Storage = DerefFlaggedStorage<Self, specs::VecStorage<Self>>;
}
