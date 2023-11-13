use std::{collections::HashMap, fmt::Display};
use crate::{syntax::{Terminal, Variable, MixedChar, self}, itemset::ItemSets};

#[derive(Clone)]
pub struct State{ 
    pub next: HashMap<MixedChar, usize>,
    pub reduce: Option<(Variable, usize)>
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

pub enum Action{
    Accept,
    Reject,
    Shift(usize),
    Reduce(Variable, usize),
}

pub struct StateMachine{
    pub states: Vec<State>,
}

impl Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, state) in self.states.iter().enumerate(){
            write!(f, "{}: ", index)?;
            if let Some((reduced_var, _reduce_state)) = state.reduce {
                if reduced_var.symbol == syntax::END_VARIABLE{
                    write!(f, "accept\n")?;
                    continue;
                }
                write!(f, "reduce: {}\n", reduced_var)?;
                continue;
            }
            if !state.next.is_empty() {
                write!(f, "shift: ")?;
            }
            for (requirement, stateid) in state.next.iter(){
                write!(f, "({}, {}) ", requirement, stateid)?;
            }

            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl StateMachine {    
    pub fn reduce_state(&self, index: usize, variable: Variable) -> usize {
        *(&self.states[index].check_variable(&variable).unwrap_or(0))
    } 

    pub fn next(&self, index: usize, rest: Option<Terminal>) -> Action {
        let cur_state = &self.states[index];
        if let Some((reduceval, reduce_state)) = cur_state.reduce{
            if reduceval.symbol == syntax::END_VARIABLE {
                return Action::Accept;
            }
            return Action::Reduce(reduceval, reduce_state)
        }
        let Some(current) = rest else { 
            // If there is no next character, reject
            return Action::Reject;
        };
        let Some(next) = cur_state.check_terminal(&current) else { 
            // If the next character do not lead anywhere, reject
            return Action::Reject;
        };
        return Action::Shift(next);
    }
    pub fn from_itemset(sets: ItemSets) -> Self {
        let mut machine = Self{
            states: vec![State::new(); sets.itemsets.len()]
        };
        for (current_state, next_states) in sets.ordering_map.iter().enumerate() {
            next_states.iter().for_each(|(k, v)| {
                machine.states[current_state].next.insert(*k, *v);
            })
        }

        for (id, set) in sets.itemsets.iter().enumerate() {
            if let Some((dot, variable)) = set.reduce(&sets.rules) {
                machine.states[id].reduce = Some((variable, dot));
            }
        }
        machine
    }
}