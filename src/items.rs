use core::fmt::Result;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::items::Item::{Default, Item1, Item2};

#[derive(Serialize, Deserialize, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Item {
    Item1,
    Item2,
    Default,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct Items(HashMap<Item, BigUint>);

pub struct ItemDescription {
    pub(crate) name: String,
    pub(crate) cost: BigUint,
    pub(crate) buff: Buff,
}

pub enum Buff {
    Additive(BigUint),
    Multiplicative(BigUint),
}

impl Item {
    pub fn description(&self) -> ItemDescription {
        match self {
            Self::Item1 => ItemDescription {
                name: String::from("Item1"),
                cost: BigUint::from(10u32),
                buff: Buff::Additive(BigUint::from(5u32)),
            },
            Self::Item2 => ItemDescription {
                name: String::from("Item2"),
                cost: BigUint::from(20u32),
                buff: Buff::Multiplicative(BigUint::from(2u32)),
            },
            Self::Default => ItemDescription {
                name: String::from("Default"),
                cost: BigUint::from(0u32),
                buff: Buff::Additive(BigUint::from(0u32)),
            },
        }
    }
}

impl Items {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn get(&self, item: Item) -> Option<&BigUint> {
        self.0.get(&item)
    }

    pub fn insert(&mut self, item: Item, value: BigUint) {
        self.0.insert(item, value);
    }
}

impl From<u16> for Item {
    fn from(value: u16) -> Self {
        match value {
            65 => Item1,
            66 => Item2,
            _ => Default,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.description().name)
    }
}

impl Display for Items {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut first = true;
        for (item, quantity) in &self.0 {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{item}: {quantity}")?;
        }
        Ok(())
    }
}
