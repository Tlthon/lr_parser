use crate::{parsingtable::StateMachine, syntax::{MixedString, TerminalString}};

pub fn solving(input: &[char], machine: StateMachine) {
    let input = TerminalString::from(input);
    let mut string_index:usize = 0;
    let mut state_index:usize = 0;
    let mut output: MixedString = MixedString::new();
    let mut stack: Vec<usize> = vec![0];

    loop{
        let action = machine.next(state_index, input.get(string_index).map(Into::into));
        match action {
            crate::parsingtable::Action::Accept => {
                println!("{} {:?} {:?} {}",state_index ,output ,&input[string_index..], "Accept");
                return;

            },
            crate::parsingtable::Action::Reject => {
                println!("{} {:?} {:?} {}",state_index ,output ,&input[string_index..], "Reject");
                return;
            },
            crate::parsingtable::Action::Shift(next) => {
                println!("{} {:?} {:?} {}",state_index ,output ,&input[string_index..], "Shift");
                output.push_terminal(input.get(string_index).unwrap());
                string_index+=1;
                state_index = next;
                stack.push(state_index);
            },
            crate::parsingtable::Action::Reduce(variable, pop_count) => {
                println!("{} {:?} {:?} {}",state_index ,output ,&input[string_index..], "Reduce");
                for _ in 0..pop_count{
                    output.pop();
                    stack.pop();
    
                }
                output.push_variable(variable);
                state_index = machine.reduce_state(*stack.last().unwrap(), variable);
                stack.push(state_index);
            }
        }
    }
}