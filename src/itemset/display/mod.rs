use std::fmt::{Display, Formatter};

pub mod item_lookahead;
pub mod item_no_lookahead;
mod lr_one;
mod lr_zero;

pub const DOT: char = '•';

// impl<Item, Set> Display for dyn super::ItemSets<'_, Item=Item, ItemSet=Set> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         for (number, item_set) in self.item_sets().iter().enumerate() {
//             write!(f, "Item set {}\n",number)?;
//             for item in &item_set.items() {
//                 item.display(&self.rules()).fmt(f)?;
//                 if item.kernel() {
//                     write!(f, "*")?;
//                 }
//                 write!(f, "\n")?;
//             }
//         }
//         Ok(())
//
//     }
// }
