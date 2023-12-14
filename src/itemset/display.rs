use std::fmt::Display;

use crate::syntax::Rule;

use super::{Item, DOT};

pub struct ItemDisplay<'a> {
    pub(super) item: &'a Item,
    pub(super) rules: &'a [Rule]
}

impl Display for ItemDisplay<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if ! self.item.kernel{
            write!(f, "+ ")?;
        }
        write!(f, "{} -> ", self.rules[self.item.rule_number].clause)?;
        if 0 == self.item.dot {
            write!(f, "{} ",DOT)?;
        }
        write!(f, "[")?;
        let len = self.rules[self.item.rule_number].output.data.len();
        for (i, character) in self.rules[self.item.rule_number].output.data.iter().enumerate() {
            write!(f, "{}",character)?;
            if i + 1 < len || i+1 == self.item.dot{
                write!(f, " ")?;
            }

            if i + 1 == self.item.dot{
                write!(f, "{}",DOT)?;
                if i + 1 < len {
                    write!(f, " ")?;
                }
            }
        }

        write!(f, "], {}", self.item.follow)
        // Ok(())
    }
}
