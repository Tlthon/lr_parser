use std::fmt::Display;
use crate::syntax::Rule;
use super::super::item_lookahead::Item;
use super::DOT;

pub struct ItemDisplay<'a> {
    pub(in crate::itemset) item: &'a Item,
    pub(in crate::itemset) rules: &'a [Rule]
}

impl Display for ItemDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut strlen = 6;
        write!(f, "[{:6} -> ", self.rules[self.item.rule_number].clause)?;
        if 0 == self.item.dot() {
            write!(f, "{} ",DOT)?;
            strlen += 2;
        }
        let len = self.rules[self.item.rule_number].output.data.len();
        for (i, character) in self.rules[self.item.rule_number].output.data.iter().enumerate() {
            write!(f, "{}", character)?;
            strlen += character.display_len();

            if i + 1 < len || i + 1 == self.item.dot() {
                write!(f, " ")?;
                strlen += 1;
            }
            if i + 1 == self.item.dot() {
                write!(f, "{}", DOT)?;
                strlen += 1;
                if i + 1 < len {
                    write!(f, " ")?;
                    strlen += 1;
                }
            }
        }
        if let Some(width) = f.width() {
            write!(f, ", {:>width$}]", self.item.follow(), width = width - strlen)?;
        }else {
            write!(f, ", {}]", self.item.follow())?;
        }
        Ok(())
    }
}

