use std::fmt::Display;

use crate::parsing_table::{IStateMachine};
use crate::syntax::{MixedString, TerminalString};
#[derive(Clone)]
pub struct ParsingProcess{
    input: TerminalString,
    string_index: usize,
    state_index: usize,
    output: MixedString,
    stack: Vec<usize>,
}

impl  ParsingProcess{
    pub fn new(input: &[char]) -> ParsingProcess {
        let input = TerminalString::from(input);

        ParsingProcess { input, string_index: 0, state_index: 0, output: MixedString::new(), stack: vec![0] }
    }
}
impl <'a> ParsingProcess  {
    #[allow(unused)]
    pub fn run<Machine: IStateMachine<'a>>(&mut self, machine: &Machine) -> Option<bool>{
        let action = machine.next_action(self.state_index, self.input.get(self.string_index).map(Into::into));
        match action {
            crate::parsing_table::Action::Accept => {
                return Some(true);
            },
            crate::parsing_table::Action::Reject => {
                return Some(false);
            },
            crate::parsing_table::Action::Shift(next) => {
                self.output.push_terminal(self.input.get(self.string_index).unwrap());
                self.string_index+=1;
                self.state_index = next;
                self.stack.push(self.state_index);
            },
            crate::parsing_table::Action::Reduce(variable, pop_count) => {
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

    pub fn get_next<Machine: IStateMachine<'a>>(&self, machine: &Machine) -> Option<Self>{
        let action = machine.next_action(self.state_index, self.input.get(self.string_index).map(Into::into));
        let mut next_step = self.clone();
        match action {
            crate::parsing_table::Action::Accept => {
                return None;
            },
            crate::parsing_table::Action::Reject => {
                return None;
            },
            crate::parsing_table::Action::Shift(next) => {
                next_step.output.push_terminal(next_step.input.get(next_step.string_index).unwrap());
                next_step.string_index+=1;
                next_step.state_index = next;
                next_step.stack.push(next_step.state_index);
            },
            crate::parsing_table::Action::Reduce(variable, pop_count) => {
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


    pub fn display<Machine: IStateMachine<'a>>(&'a self, machine: &'a Machine) -> PrintingString {
        let action = machine.next_action(self.state_index, self.input.get(self.string_index).map(Into::into));
        let next_action = match action {
            crate::parsing_table::Action::Accept => "Accept",
            crate::parsing_table::Action::Reject => "Reject",
            crate::parsing_table::Action::Shift(_) => "Shift",
            crate::parsing_table::Action::Reduce(_, _) => "Reduce",
        };
        PrintingString { process: self, action: next_action }
    }
}

pub struct PrintingString<'temp>{
    process: &'temp ParsingProcess,
    action: &'static str
}
use prettytable::{Cell, Row};


impl PrintingString<'_> {
    pub fn get_row(&self, index: usize) -> Row{
        Row::new(vec![
            Cell::new(&index.to_string()),
            Cell::new(&format!("{:?}",self.process.state_index)),
            Cell::new(&format!("{:?}",self.process.stack)),
            Cell::new(&format!("{:?}",self.process.output)),
            Cell::new(&format!("{:?}",&self.process.input[self.process.string_index..])),
            Cell::new(self.action),
        ])
        // write!(f, "{} {:?} {:?} {:?} {}",self.process.state_index, self.process.stack ,self.process.output ,&self.process.input[self.process.string_index..], self.action)
    }

}

impl Display for PrintingString<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {:?} {:?} {}",self.process.state_index, self.process.stack ,self.process.output ,&self.process.input[self.process.string_index..], self.action)
    }
}