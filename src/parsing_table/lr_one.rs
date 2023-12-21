use std::collections::HashMap;
// use crate::itemset::LROneItemSets;
use crate::parsing_table::{Action};
use crate::parsing_table::display::lr_one::StateMachineDisplay;
use crate::syntax;
use crate::syntax::{MixedChar, Rule, Terminal, Variable};

#[derive(Clone)]
pub struct State{
    pub next: HashMap<MixedChar, usize>,
    pub reduce: HashMap<Terminal, Rule>
}

impl State {
    fn new() -> Self {
        Self {
            next: HashMap::new(),
            reduce: HashMap::new()
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
    type MachineDisplay = StateMachineDisplay<'a>;
    type ItemSets = crate::itemset::LROneItemSets;
    fn display(&'a self, itemset: &'a Self::ItemSets) -> StateMachineDisplay<'a> {
        StateMachineDisplay::new(&self, &itemset)
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
            for (rule, follow) in set.reduce(&sets.rules) {
                if let Some(current_rule) = &machine.states[id].reduce.get(&follow){
                    continue;
                }
                machine.states[id].reduce.insert(follow, rule.clone());
            }
        }
        machine
    }

    fn next_action(&self, index: usize, rest: Option<Terminal>) -> Action {
        let cur_state = &self.states[index];
        let current = rest.unwrap_or(Terminal::epsilon());
        if let Some(next) = cur_state.check_terminal(&current) {
            return Action::Shift(next);
        }
        if let Some(rule) = &cur_state.reduce.get(&current){
            let clause = rule.clause;
            let reduce_state = rule.len();
            if clause.symbol == syntax::END_VARIABLE {
                return Action::Accept;
            }
            return Action::Reduce(clause, reduce_state)
        }

        return Action::Reject;
    }
    fn reduce_state(&self, index: usize, variable: Variable) -> usize {
        self.states[index].check_variable(&variable).unwrap_or(0)
    }

}