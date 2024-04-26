use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::{self, Write},
    ops::Add,
};

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::items::{Buff, Item, Items};

#[derive(Serialize, Deserialize)]
pub struct GameStorage {
    pub(crate) coins: BigUint,
    rate_of_income: BigUint,
    pub(crate) rate_of_slowdown: u32,
    pub(crate) items: Items,
    pub(crate) last_item_cost: BigUint,
    multiplicatives: BigUint,
    additives: BigUint,
    pub(crate) last_bought: Option<String>,
}

impl GameStorage {
    pub fn new(rate_of_slowdown: u32) -> Self {
        Self {
            coins: BigUint::from(0u8),
            rate_of_income: BigUint::from(1u8),
            rate_of_slowdown,
            items: Items::new(),
            last_item_cost: BigUint::from(0u32),
            multiplicatives: BigUint::from(1u32),
            additives: BigUint::from(0u32),
            last_bought: None,
        }
    }
    pub fn percent_rate(&self, cpu_usage: f32) -> BigUint {
        &self.rate_of_income * (((cpu_usage * 100f32).round()) as u32) / 100u32
    }

    fn roi(&self) -> BigUint {
        (&self.rate_of_income + &self.additives) * &self.multiplicatives
    }
    pub fn update(&mut self, cpu_usage: f32) -> (BigUint, BigUint) {
        let rate = &self.percent_rate(cpu_usage);
        let roi = &self.roi();
        self.coins += rate * roi;

        (rate.clone(), roi.clone())
    }
    pub fn buy(&mut self, item: &Item) -> Result<(), GameError> {
        let cost = item.description().cost;
        if self.coins < cost {
            return Err(GameError::NotEnoughCoins {
                have: &self.coins,
                expect: cost,
            });
        }

        let zero = BigUint::default();

        self.last_item_cost.clone_from(&cost);
        self.coins -= &cost;
        self.set_buffs(item);
        let old = &self.items.get(*item).unwrap_or(&zero);
        self.items.insert(*item, old.add(1u16));
        Ok(())
    }

    pub fn save(&self, filename: &String) -> io::Result<()> {
        let json_data = serde_json::to_string_pretty(self)?;

        let mut file = File::create(filename)?;

        file.write_all(json_data.as_bytes())?;
        println!("Data saved to {filename}");
        Ok(())
    }
    fn set_buffs(&mut self, item: &Item) {
        match item.description().buff {
            Buff::Additive(add) => self.additives += &add,
            Buff::Multiplicative(mul) => self.multiplicatives *= &mul,
        }
    }
}

pub enum GameError<'err> {
    NotEnoughCoins {
        have: &'err BigUint,
        expect: BigUint,
    },
}

impl Display for GameError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::NotEnoughCoins { have, expect } => {
                write!(
                    f,
                    "Not enough coins. You have {have}, but you need {expect}."
                )
            }
        }
    }
}
