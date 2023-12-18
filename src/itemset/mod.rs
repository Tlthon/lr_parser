mod lr_one;
mod lr_zero;

mod item_no_lookahead;
mod item_lookahead;

mod display;

use std::fmt::Display;
pub use lr_one::ItemSets as LROneItemSets;
pub use lr_zero::ItemSets as LRZeroItemSets;
use crate::syntax::{MixedChar, Rule, Terminal};

trait Item <'display>{
    type Display;
    fn shift(&self) -> Self;
    fn symbol(&self, rules: &[Rule]) -> Option<MixedChar>;
    fn is_end(&self, rules: &[Rule]) -> bool;
    fn display (&'display self, rules: &'display [Rule]) -> Self::Display;
    fn dot(&self) -> usize;
    fn kernel(&self) -> bool;
}

trait ItemSet <'item, 'item_iterator: 'item>{
    type Item: Item<'item> ;
    type ItemIterator: Iterator<Item = &'item_iterator Self::Item>;
    fn items(&'item self) -> Self::ItemIterator;
}

trait ItemSets {
    type Item: for<'a> Item<'a> ;
    type ItemSet: for <'a, 'b> ItemSet <'a, 'b>;

    fn item_sets(&self) -> &[Self::ItemSet];
    fn rules(&self) -> &[Rule];
}