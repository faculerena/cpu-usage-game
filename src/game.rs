use std::fs::File;
use std::io;
use std::io::Write;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::{FILENAME, items};
use crate::items::*;

#[derive(Serialize, Deserialize)]
pub struct GameStorage {
    pub(crate) coins: BigUint,
    rate_of_income: BigUint,
    pub(crate) rate_of_slowdown: u32,
    pub(crate) items: Items,
    pub(crate) last_item_cost: BigUint,
    multiplicatives: BigUint,
    additives: BigUint,
}

impl GameStorage {
    pub fn new(rate_of_slowdown: u32) -> Self {
        Self {
            coins: BigUint::from(0u8),
            rate_of_income: BigUint::from(1u8),
            rate_of_slowdown,
            items: items::Items::new(),
            last_item_cost: BigUint::from(0u32),
            multiplicatives: BigUint::from(1u32),
            additives: BigUint::from(0u32),
        }
    }
    pub fn percent_rate(&self, cpu_usage: f32) -> BigUint {
        &self.rate_of_income * (((cpu_usage * 100f32).round()) as u32) / 100u32
    }

    fn roi(&self) -> BigUint {
        let v = (&self.rate_of_income + &self.additives) * &self.multiplicatives;
        v
    }
    pub fn update(&mut self, cpu_usage: f32) -> (BigUint, BigUint) {
        let rate = &self.percent_rate(cpu_usage);
        let roi = &self.roi();
        self.coins += (rate * roi);

        (rate.clone(), roi.clone())
    }
    pub fn buy(&mut self, item: &Item) -> Result<(), GameError> {
        let cost = item.description().cost;
        if self.coins < cost {
            return Err(GameError::NotEnoughCoins);
        }

        self.last_item_cost.clone_from(&cost);
        self.coins -= &cost;
        self.set_buffs(&item);
        self.items.push(item.clone());
        Ok(())
    }

    pub fn save(&self) -> io::Result<()> {
        let json_data = serde_json::to_string_pretty(self)?;

        let mut file = File::create(FILENAME)?;

        file.write_all(json_data.as_bytes())?;
        println!("Data saved to {}", FILENAME);
        Ok(())
    }
    fn set_buffs(&mut self, item: &Item) {
        match item.description().buff {
            Buff::Additive(add) => self.additives += &add,
            Buff::Multiplicative(mul) => self.multiplicatives *= &mul,
        }
    }
}

pub(crate) enum GameError {
    NotEnoughCoins,
}
