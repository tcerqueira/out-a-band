#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Copy)]
pub struct Monster {
    pub hp: u32,
    pub x: u32,
    pub y: u32,
}

pub struct Game<M> {
    pub monster_list: Vec<Monster>,
    pub held_items: M,
}

pub trait ItemsAccess {
    fn get_items(&self, k: u32) -> &[u32; 4];
}

impl<M: ItemsAccess> Game<M> {
    pub fn get_monster_info(&self, idx: u32) -> (&Monster, &[u32; 4]) {
        (
            &self.monster_list[idx as usize],
            self.held_items.get_items(idx),
        )
    }
}

impl ItemsAccess for HashMap<u32, [u32; 4]> {
    fn get_items(&self, k: u32) -> &[u32; 4] {
        &self[&k]
    }
}

impl ItemsAccess for BTreeMap<u32, [u32; 4]> {
    fn get_items(&self, k: u32) -> &[u32; 4] {
        &self[&k]
    }
}
