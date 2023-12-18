mod lr_one;
mod lr_zero;

mod item_no_lookahead;
mod item_lookahead;

mod display;

use std::fmt::Display;
pub use lr_one::ItemSets as LROneItemSets;
pub use lr_zero::ItemSets as LRZeroItemSets;
use crate::syntax::{MixedChar, Rule, Terminal};

pub trait Item <'display>{
    type Display;
    fn shift(&self) -> Self;
    fn symbol(&self, rules: &[Rule]) -> Option<MixedChar>;
    fn is_end(&self, rules: &[Rule]) -> bool;
    fn display (&'display self, rules: &'display [Rule]) -> Self::Display;
    fn dot(&self) -> usize;
    fn kernel(&self) -> bool;
}

pub trait ItemSet <'item_iterator>
{
    type Item: Item<'item_iterator> + 'item_iterator ;
    type ItemIterator: Iterator<Item = &'item_iterator Self::Item>;
    fn items(&'item_iterator self) -> Self::ItemIterator;
}

pub trait ItemSets<'a>  {
    type Item:  Item<'a> + 'a;
    type ItemSet: ItemSet <'a, Item = Self::Item>;

    fn item_sets(&self) -> &[Self::ItemSet];
    fn rules(&self) -> &[Rule];
}