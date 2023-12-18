// use std::fmt::Display;
// use crate::itemset::Item as _;
// use super::super::lr_one::ItemSets;
//
// impl Display for ItemSets {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for (number, item_set) in self.sets.iter().enumerate() {
//             write!(f, "Item set {}\n",number)?;
//             for item in &item_set.items {
//                 item.display(&self.rules).fmt(f)?;
//                 if item.kernel() {
//                     write!(f, "*")?;
//                 }
//                 write!(f, "\n")?;
//             }
//         }
//         Ok(())
//     }
// }