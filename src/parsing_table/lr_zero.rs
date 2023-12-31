use std::collections::HashMap;
use crate::parsing_table::{Action, display};
use crate::syntax;
use crate::syntax::{MixedChar, Rule, Terminal, Variable};

#[derive(Clone)]
pub struct State{
    pub next: HashMap<MixedChar, usize>,
    pub reduce: Option<Rule>
}

impl State {
    fn new() -> Self {
        Self {
            next: HashMap::new(),
            reduce: None
        }
    }
    fn check_terminal(&self, terminal: &Terminal) -> Option<usize>{
        self.next.get(&MixedChar::Terminal(*terminal)).copied()
    }
    fn check_variable(&self, variable: &Variable) -> Option<usize>{
        self.next.get(&MixedChar::Variable(*variable)).copied()
    }
}

pub struct StateMachine{
    pub states: Vec<State>,
}

impl<'a> super::IStateMachine<'a> for StateMachine {
    type MachineDisplay = display::lr_zero::StateMachineDisplay<'a>;
    type ItemSets = crate::itemset::LRZeroItemSets;

    fn display(&'a self, itemset: &'a Self::ItemSets) -> Self::MachineDisplay{
        Self::MachineDisplay::new(&self, &itemset)
    }

    fn reduce_state(&self, index: usize, variable: Variable) -> usize {
        self.states[index].check_variable(&variable).unwrap_or(0)
    }

    fn next_action(&self, index: usize, rest: Option<Terminal>) -> Action {
        let cur_state = &self.states[index];
        let current = rest.unwrap_or(Terminal::epsilon());
        if let Some(next) = cur_state.check_terminal(&current) {
            return Action::Shift(next);
        }
        if let Some(rule) = &cur_state.reduce{
            let clause = rule.clause;
            let reduce_state = rule.len();
            if clause.symbol == syntax::END_VARIABLE {
                return Action::Accept;
            }
            return Action::Reduce(clause, reduce_state)
        }

        return Action::Reject;
    }
    fn from_itemset(sets: &Self::ItemSets) -> Self {
        let mut machine = Self{
            states: vec![State::new(); sets.sets.len()]
        };
        for (current_state, next_states) in sets.ordering_map.iter().enumerate() {
            next_states.iter().for_each(|(k, v)| {
                machine.states[current_state].next.insert(*k, *v);
            })
        }

        for (id, set) in sets.sets.iter().enumerate() {
            for rule in set.reduce(&sets.rules) {
                if let Some(current_rule) = &machine.states[id].reduce{
                    println!("Found reduce-reduce conflict in state {} between rule {} and {}", id, current_rule, &rule);
                    continue;
                }
                machine.states[id].reduce = Some(rule.clone());
            }
        }
        machine
    }
}