
use std::fmt::Display;
use crate::itemset::{ItemSet as _, ItemSets as _, LROneItemSets};
use crate::parsing_table::lr_one::{State, StateMachine};
use crate::syntax::MixedChar;
use crate::syntax;
use crate::itemset::Item as _;

pub struct StateMachineDisplay<'a> {
    states: &'a [State],
    sets: &'a LROneItemSets,
}

impl<'a> StateMachineDisplay<'a> {
    pub fn new(machine: &'a StateMachine, sets: &'a LROneItemSets) -> Self {
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
                if item.kernel() {
                    f.write_str("*")?;
                }
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
            if state.reduce.len() != 0 && state.next.len() != 0 {
                write!(f,"\n")?;
            }
            for (follow, rule) in &state.reduce {
                if let Some(next_state_id) = state.next.get(&(MixedChar::from(*follow))) {
                    write!(f, "    shift-reduce conflict on {}\n", follow)?;
                    write!(f, "        favor shift({}) over reduce({})\n", next_state_id, rule)?;

                    continue;
                }
                for (rule1, rule2) in itemset.reduce_reduce_conflict(self.sets.rules()) {
                    write!(f, "    reduce-reduce between rule {}and {}\n", rule1, rule2)?;
                    write!(f, "        Favor rule {}over rule {}\n", rule1, rule2)?;
                }

                let reduced_var = rule.clause;
                if reduced_var.symbol == syntax::END_VARIABLE{
                    write!(f, "    {:4}accept\n", follow)?;
                    continue;
                }
                write!(f, "    {:4}reduce {}\n", follow ,rule)?;
            }

        }
        Ok(())
    }
}

