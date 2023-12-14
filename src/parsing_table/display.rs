use std::fmt::Display;
use crate::{itemset::ItemSets, syntax};
use super::{State, StateMachine};

pub struct StateMachineDisplay<'a> {
    states: &'a [State],
    sets: &'a ItemSets,
}

impl<'a> StateMachineDisplay<'a> {
    pub(super) fn new(machine: &'a StateMachine, sets: &'a ItemSets) -> Self {
        Self { states: &machine.states, sets }
    }
}

impl<'a> Display for StateMachineDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, state) in self.states.iter().enumerate(){


            write!(f, "state {}\n", index)?;
            for item in &self.sets.sets[index].items {
                if !item.kernel() {
                    continue;
                }
                write!(f, "    {}\n", item.display(&self.sets.rules))?;
            }
            write!(f,"\n")?;

            for (follow, rule) in &state.reduce {
                let reduced_var = rule.clause;
                if reduced_var.symbol == syntax::END_VARIABLE{
                    write!(f, "    {:3} accept\n", follow)?;
                    continue;
                }
                write!(f, "    {:3} reduce {}\n", follow ,rule)?;
            }
            if state.reduce.len() != 0 && state.next.len() != 0 {
                write!(f,"\n")?;
            }

            for(requirement, next_id) in state.next.iter() {
                use crate::syntax::MixedChar::{Terminal, Variable};
                match requirement {
                    Terminal(t) => write!(f, "    {:3} shift {}\n", t ,next_id),
                    Variable(v) => write!(f, "    {:3} goto {}\n", v ,next_id),
                }?;
            }
        }
        Ok(())

    }
}

