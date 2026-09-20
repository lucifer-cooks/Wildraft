use crate::comp;
use serde::{Deserialize, Serialize};
use specs::{Component, DerefFlaggedStorage};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct Hunger {
    current: u32,
    base_max: u32,
    maximum: u32,
    regen_rate: f32,
}

impl Hunger {
    const MAX_HUNGER: u16 = u16::MAX - 1;
    const MAX_SCALED_HUNGER: u32 = (Self::MAX_HUNGER as u32) * Self::SCALING_FACTOR_INT;
    const SCALING_FACTOR_FLOAT: f32 = 256.0;
    const SCALING_FACTOR_INT: u32 = 256;

    pub fn current(&self) -> f32 { self.current as f32 / Self::SCALING_FACTOR_FLOAT }

    pub fn base_max(&self) -> f32 { self.base_max as f32 / Self::SCALING_FACTOR_FLOAT }

    pub fn maximum(&self) -> f32 { self.maximum as f32 / Self::SCALING_FACTOR_FLOAT }

    pub fn fraction(&self) -> f32 { self.current() / self.maximum().max(1.0) }

    pub fn new(base_max_f: f32) -> Self {
        let base =
            (base_max_f * Self::SCALING_FACTOR_FLOAT).clamp(0.0, Self::MAX_HUNGER as f32) as u32;
        Hunger {
            current: base,
            base_max: base,
            maximum: base,
            regen_rate: 0.0,
        }
    }

    /// Gradual decrease over time (hunger drains)
    pub fn drain(&mut self, rate: f32, dt: f32) {
        if self.current > 0 {
            self.change_by(-rate * dt);
        }
    }

    /// Regeneration (eating restores)
    pub fn regen(&mut self, amount: f32) { self.change_by(amount); }

    /// Slow regeneration over time (for demo balance)
    pub fn tick_regen(&mut self, dt: f32) {
        if self.current < self.base_max {
            self.change_by(0.003 * dt); // minimal slow regen so not fully persistent
        }
    }

    pub fn change_by(&mut self, change: f32) {
        self.current = (((self.current() + change).clamp(0.0, Self::MAX_HUNGER as f32)
            * Self::SCALING_FACTOR_FLOAT) as u32)
            .min(self.maximum);
    }

    pub fn needs_food(&self) -> bool { self.current() < self.base_max() * 0.25 }

    pub fn is_critical(&self) -> bool { self.current() < self.base_max() * 0.1 }

    pub fn refresh(&mut self) { self.current = self.base_max; }
}

impl Component for Hunger {
    type Storage = DerefFlaggedStorage<Self, specs::VecStorage<Self>>;
}
