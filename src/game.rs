use num_bigint::BigUint;

use crate::items;
use crate::items::*;

pub struct GameStorage {
    pub coins: BigUint,
    pub rate_of_income: BigUint,
    pub rate_of_slowdown: u32,
    pub items: Items,
    pub last_item_cost: BigUint,
}

impl GameStorage {
    pub fn new(rate_of_slowdown: u32) -> Self {
        Self {
            coins: BigUint::from(0u8),
            rate_of_income: BigUint::from(1u8),
            rate_of_slowdown,
            items: items::Items::new(),
            last_item_cost: BigUint::from(0),
        }
    }
    pub fn rate(&self, cpu_usage: f32) -> BigUint {
        &self.rate_of_income * (((cpu_usage * 100f32).round()) as u32) / 100u32
    }
    pub fn update(&mut self, cpu_usage: f32) -> (BigUint, BigUint) {
        let rate = &self.rate(cpu_usage);
        let roi = &self.rate_of_income;
        self.coins += rate;

        (rate.clone(), roi.clone())
    }
}
