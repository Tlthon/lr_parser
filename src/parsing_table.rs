use std::{collections::HashMap, fmt::Display};
use crate::{syntax::{Terminal, Variable, MixedChar, self, Rule}, itemset::ItemSets};

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

pub enum Action{
    Accept,
    Reject,
    Shift(usize),
    Reduce(Variable, usize),
}

pub struct StateMachine{
    pub states: Vec<State>,
}

mod display {
    use std::fmt::Display;
    use crate::{itemset::ItemSets, syntax};
    use super::{State, StateMachine};

    pub struct StateMachineDisplay<'a> {
        states: &'a [State],
        itemsets: &'a ItemSets,
    }

    impl<'a> StateMachineDisplay<'a> {
        pub(super) fn new(machine: &'a StateMachine, itemsets: &'a ItemSets) -> Self {
            Self { states: &machine.states, itemsets }
        }
    }

    impl<'a> Display for StateMachineDisplay<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {    
            for (index, state) in self.states.iter().enumerate(){


                write!(f, "state {}\n", index)?;
                for item in &self.itemsets.itemsets[index].items {
                    if !item.kernel() {
                        continue;
                    }
                    write!(f, "    {}\n", item.display(&self.itemsets.rules))?;
                }

                if let Some(rule) = &state.reduce {
                    let reduced_var = rule.clause;
                    if reduced_var.symbol == syntax::END_VARIABLE{
                        write!(f, "    accept\n")?;
                        continue;
                    }
                    write!(f, "    reduce: (rule {})\n", rule)?;
                }
                if !state.next.is_empty() {
                    for(requirement, stateid) in state.next.iter() {
                        use crate::syntax::MixedChar::{Terminal, Variable};
                        match requirement {
                            Terminal(t) => write!(f, "    {: <10} shift {}\n", t ,stateid),
                            Variable(v) => write!(f, "    {: <10} goto {}\n", v ,stateid),

                        }?;
                    }
                }
            }
            Ok(())
    
        }
    }
}

impl Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, state) in self.states.iter().enumerate(){
            write!(f, "{}: ", index)?;
            if let Some(rule) = &state.reduce {
                let reduced_var = rule.clause;
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
    pub fn display<'a>(&'a self, itemset: &'a ItemSets) -> display::StateMachineDisplay<'a> {
        display::StateMachineDisplay::new(&self, &itemset)
    }

    pub fn reduce_state(&self, index: usize, variable: Variable) -> usize {
        *(&self.states[index].check_variable(&variable).unwrap_or(0))
    } 

    pub fn next(&self, index: usize, rest: Option<Terminal>) -> Action {
        let cur_state = &self.states[index];
        if let Some(current) = rest { 
            if let Some(next) = cur_state.check_terminal(&current) {
                return Action::Shift(next);
            }
        }
        if let Some(rule) = &cur_state.reduce{
            let reduceval = rule.clause;
            let reduce_state = rule.len(); 
            if reduceval.symbol == syntax::END_VARIABLE {
                return Action::Accept;
            }
            return Action::Reduce(reduceval, reduce_state)
        }
        return Action::Reject;
    }
    pub fn from_itemset(sets: &ItemSets) -> Self {
        let mut machine = Self{
            states: vec![State::new(); sets.itemsets.len()]
        };
        for (current_state, next_states) in sets.ordering_map.iter().enumerate() {
            next_states.iter().for_each(|(k, v)| {
                machine.states[current_state].next.insert(*k, *v);
            })
        }

        for (id, set) in sets.itemsets.iter().enumerate() {
            for rule in set.reduce(&sets.rules) {
                if !machine.states[id].next.is_empty(){
                    println!("Found shift-reduce conflict in state {} from rule {} and shift from {:?}", id, rule, machine.states[id].next.keys())
                    
                }
                if let Some(current_rule) = &machine.states[id].reduce{
                    println!("Found reduce-reduce conflict in state {} between rule {} and {}", id, current_rule, rule)
                }

                machine.states[id].reduce = Some(rule);
            }
        }
        machine
    }
}