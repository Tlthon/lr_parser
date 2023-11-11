use std::fmt::Display;

use crate::{parsingtable::StateMachine, syntax::{MixedString, TerminalString}};

#[derive(Clone)]
pub struct ParsingProcess{
    input: TerminalString,
    string_index: usize,
    state_index: usize,
    output: MixedString,
    stack: Vec<usize>,
}

impl ParsingProcess {
    pub fn new(input: &[char]) -> ParsingProcess{
        let input = TerminalString::from(input);

        ParsingProcess { input ,string_index: 0, state_index: 0, output: MixedString::new(), stack: vec![0] }
    }
    pub fn run(&mut self, machine: &StateMachine) -> Option<bool>{
        let action = machine.next(self.state_index, self.input.get(self.string_index).map(Into::into));
        match action {
            crate::parsingtable::Action::Accept => {
                return Some(true);

            },
            crate::parsingtable::Action::Reject => {
                return Some(false);
            },
            crate::parsingtable::Action::Shift(next) => {
                self.output.push_terminal(self.input.get(self.string_index).unwrap());
                self.string_index+=1;
                self.state_index = next;
                self.stack.push(self.state_index);
            },
            crate::parsingtable::Action::Reduce(variable, pop_count) => {
                for _ in 0..pop_count{
                    self.output.pop();
                    self.stack.pop();
    
                }
                self.output.push_variable(variable);
                self.state_index = machine.reduce_state(*self.stack.last().unwrap(), variable);
                self.stack.push(self.state_index);
            }
        }
        return None;    
    }

    pub fn get_next(&self, machine: &StateMachine) -> Option<Self>{
        let action = machine.next(self.state_index, self.input.get(self.string_index).map(Into::into));
        let mut next_step = self.clone();
        match action {
            crate::parsingtable::Action::Accept => {
                return None;

            },
            crate::parsingtable::Action::Reject => {
                return None;
            },
            crate::parsingtable::Action::Shift(next) => {
                next_step.output.push_terminal(next_step.input.get(next_step.string_index).unwrap());
                next_step.string_index+=1;
                next_step.state_index = next;
                next_step.stack.push(next_step.state_index);
            },
            crate::parsingtable::Action::Reduce(variable, pop_count) => {
                for _ in 0..pop_count{
                    next_step.output.pop();
                    next_step.stack.pop();
                }
                next_step.output.push_variable(variable);
                next_step.state_index = machine.reduce_state(*next_step.stack.last().unwrap(), variable);
                next_step.stack.push(next_step.state_index);
            }
        }
        return Some(next_step);    
    }


    pub fn display<'a>(&'a self, machine: &'a StateMachine) -> PrintingString {
        let action = machine.next(self.state_index, self.input.get(self.string_index).map(Into::into));
        let next_action = match action {
            crate::parsingtable::Action::Accept => "Accept",
            crate::parsingtable::Action::Reject => "Reject",
            crate::parsingtable::Action::Shift(_) => "Shift",
            crate::parsingtable::Action::Reduce(_, _) => "Reduce",
        };
        PrintingString { process: self, action: next_action }
    }
}

pub struct PrintingString<'temp>{
    process: &'temp ParsingProcess,
    action: &'static str
}

impl Display for PrintingString<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {:?} {:?} {}",self.process.state_index, self.process.stack ,self.process.output ,&self.process.input[self.process.string_index..], self.action)
    }
}