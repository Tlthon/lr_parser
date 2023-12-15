use std::collections::HashMap;
use crate::{syntax::{Terminal, Variable, MixedChar, self, Rule}, itemset::ItemSets};

mod display;
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

pub enum Action{
    Accept,
    Reject,
    Shift(usize),
    Reduce(Variable, usize),
}

pub struct StateMachine{
    pub states: Vec<State>,
}

impl StateMachine {    
    pub fn display<'a>(&'a self, itemset: &'a ItemSets) -> display::StateMachineDisplay<'a> {
        display::StateMachineDisplay::new(&self, &itemset)
    }

    pub fn reduce_state(&self, index: usize, variable: Variable) -> usize {
        *(&self.states[index].check_variable(&variable).unwrap_or(0))
    } 

    pub fn next(&self, index: usize, rest: Option<Terminal>) -> Action {
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
    pub fn from_itemset(sets: &ItemSets) -> Self {
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
                    println!("Found reduce-reduce conflict in state {} between rule {} and {}", id, current_rule, rule)
                }
                machine.states[id].reduce.insert(follow, rule);
            }
        }
        machine
    }
}