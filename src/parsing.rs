use crate::{parsingtable::StateMachine, syntax::{MixedString, TerminalString}};

pub struct ParsingProcess{
    input: TerminalString,
    string_index: usize,
    state_index: usize,
    output: MixedString,
    stack: Vec<usize>
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
                println!("{} {:?} {:?} {}",self.state_index ,self.output ,&self.input[self.string_index..], "Accept");
                return Some(true);

            },
            crate::parsingtable::Action::Reject => {
                println!("{} {:?} {:?} {}",self.state_index ,self.output ,&self.input[self.string_index..], "Reject");
                return Some(false);
            },
            crate::parsingtable::Action::Shift(next) => {
                println!("{} {:?} {:?} {}",self.state_index ,self.output ,&self.input[self.string_index..], "Shift");
                self.output.push_terminal(self.input.get(self.string_index).unwrap());
                self.string_index+=1;
                self.state_index = next;
                self.stack.push(self.state_index);
            },
            crate::parsingtable::Action::Reduce(variable, pop_count) => {
                println!("{} {:?} {:?} {}",self.state_index ,self.output ,&self.input[self.string_index..], "Reduce");
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
}