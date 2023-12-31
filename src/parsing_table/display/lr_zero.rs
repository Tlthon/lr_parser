
use std::fmt::Display;
use crate::{itemset::LRZeroItemSets};
use crate::syntax;
use crate::parsing_table::lr_zero::{State, StateMachine};
use crate::itemset::{Item as _, ItemSet, ItemSets};


pub struct StateMachineDisplay<'a> {
    states: &'a [State],
    sets: &'a LRZeroItemSets,
}

impl<'a> StateMachineDisplay<'a> {
    pub fn new(machine: &'a StateMachine, sets: &'a LRZeroItemSets) -> Self {
        Self { states: &machine.states, sets }
    }
}

impl<'a> Display for StateMachineDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, (state, itemset)) in self.states.iter().zip(&self.sets.sets).enumerate(){
            write!(f, "state {}\n", index)?;
            for item in &itemset.items {
                if !item.kernel() {
                    continue;
                }
                f.write_str("    ")?;
                item.display(&self.sets.rules).fmt(f)?;
                f.write_str("\n")?;
            }
            write!(f,"\n")?;
            for(requirement, next_id) in state.next.iter() {
                use crate::syntax::MixedChar::{Terminal, Variable};
                match requirement {
                    Terminal(t) => write!(f, "    {:4}shift {}\n", t ,next_id),
                    Variable(v) => write!(f, "    {:4}goto {}\n", v ,next_id),
                }?;
            }
            if state.reduce.is_some() && state.next.len() != 0 {
                write!(f,"\n")?;
            }
            for rule in &state.reduce {
                for next_state_id in state.next.values() {
                    write!(f, "    shift-reduce conflict \n")?;
                    write!(f, "        favor shift({}) over reduce({})\n", next_state_id, rule)?;
                    continue;
                }

                for (rule1, rule2) in itemset.reduce_reduce_conflict(self.sets.rules()) {
                    write!(f, "    reduce-reduce between rule {}and {}\n", rule1, rule2)?;
                    write!(f, "        Favor rule {}over rule {}\n", rule1, rule2)?;
                }
                let reduced_var = rule.clause;
                if reduced_var.symbol == syntax::END_VARIABLE{
                    write!(f, "    accept\n")?;
                    continue;
                }
                write!(f, "    reduce {}\n", rule)?;
            }

        }
        Ok(())
    }
}

