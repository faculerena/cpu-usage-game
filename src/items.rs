use std::fmt::{Display, Formatter};

use num_bigint::BigUint;

use crate::items::Item::{DefaultItem, Item1, Item2};

pub enum Item {
    Item1,
    Item2,
    DefaultItem,
}

pub struct Items(Vec<Item>);

pub struct ItemDescription {
    name: String,
    cost: BigUint,
    buff: Buff,
}

pub enum Buff {
    Additive(BigUint),
    Multiplicative(BigUint),
}

impl Item {
    pub fn description(&self) -> ItemDescription {
        match self {
            Item::Item1 => ItemDescription {
                name: String::from("Item1"),
                cost: BigUint::from(10u32),
                buff: Buff::Additive(BigUint::from(5u32)),
            },
            Item::Item2 => ItemDescription {
                name: String::from("Item2"),
                cost: BigUint::from(20u32),
                buff: Buff::Multiplicative(BigUint::from(2u32)),
            },
            Item::DefaultItem => ItemDescription {
                name: String::from("Default"),
                cost: BigUint::from(0u32),
                buff: Buff::Additive(BigUint::from(0u32)),
            },
        }
    }
}

impl Items {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, item: Item) {
        self.0.push(item)
    }
}

impl From<u16> for Item {
    fn from(value: u16) -> Self {
        match value {
            65 => Item1,
            66 => Item2,
            _ => DefaultItem
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Item1 => { write!(f, "{}", stringify!(Item1)) }
            Item2 => { write!(f, "{}", stringify!(Item2)) }
            DefaultItem => { write!(f, "{}", stringify!(DefaultItem)) }
        }
    }
}

impl Display for Items {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let items_str: Vec<String> = self.0.iter().map(|item| format!("{}", item)).collect();
        write!(f, "[{}]", items_str.join(", "))
    }
}