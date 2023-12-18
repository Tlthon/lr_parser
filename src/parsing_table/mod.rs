use std::fmt::Display;

use crate::syntax::{Terminal, Variable};

mod display;
pub mod lr_one;
pub mod lr_zero;

pub enum Action{
    Accept,
    Reject,
    Shift(usize),
    Reduce(Variable, usize),
}

pub trait IStateMachine<'a> {
    type MachineDisplay: Display;
    type ItemSets;
    fn display(&'a self, itemset: &'a Self::ItemSets) -> Self::MachineDisplay;
    fn from_itemset(sets: &Self::ItemSets) -> Self;
    fn next_action(&self, index: usize, rest: Option<Terminal>) -> Action;
    fn reduce_state(&self, index: usize, variable: Variable) -> usize;
}