use getch_rs::Getch;
use prettytable::{Table, Row, Cell};

use crate::{parsing_table::StateMachine, syntax::Rule, parsing::ParsingProcess, itemset::ItemSets};
pub mod itemset;
pub mod syntax;
mod parsing_table;
mod parsing;
pub mod rule_depend;
pub mod first_follow;
mod tarjan;
mod map_set;
fn main() {

    let mut itemset = ItemSets::new();
    {
        let mut rule = Rule::new(syntax::END_VARIABLE);
        rule.add_variable('E');
        rule.add_terminal(syntax::END_TERMINAL);
        itemset.add_rule(rule);
    }
    itemset.add_from_string("E:E+M");
    itemset.add_from_string("E:M");
    itemset.add_from_string("M:M*P");
    itemset.add_from_string("M:P");
    itemset.add_from_string("P:n");
    itemset.add_from_string("P:(E)");
    edit_rule(&mut itemset);

    let machine = StateMachine::from_itemset(&itemset);

    // println!("{:18}", itemset);
    println!("{:20}", machine.display(&itemset));
    print!("\nTaking input\n");
    let mut line = std::io::stdin().lines().next().unwrap().unwrap();
    line.push(syntax::END_TERMINAL);
    let input_vec: Vec<char> = line.chars().collect();
    let parser = ParsingProcess::new(&input_vec);
    let g = getch_rs::Getch::new();
    let mut history = vec![parser];
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    run_parsing(&machine, history);
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn edit_rule(itemset: &mut ItemSets) {
    clear_screen();
    loop {
        println!("Press s to save, a to add new rule, p to print the resulting itemset");
        let Ok(key_press) = getch_rs::Getch::new().getch() else {break};
        match key_press {
            getch_rs::Key::Char('s') => {
                itemset.clear();
                itemset.generate_next();
                return;
            },
            getch_rs::Key::Char('p') => {
                for rule in &itemset.rules {
                    println!("{}", rule);
                }
            },
            getch_rs::Key::Char('a')=> {
                loop {
                    println!("Please type in new rule (format is Clause->MixedString)");
                    let Some(Ok(line)) = std::io::stdin().lines().next() else {clear_screen();continue };
                    let success = itemset.add_from_string(&line);
                    if success { break; }
                    clear_screen();
                }
            }
            _ => {
                clear_screen();
            }
        }


    }
}

fn run_parsing(machine: &StateMachine, mut history: Vec<ParsingProcess>) {
    let g = getch_rs::Getch::new();
    loop {
        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Step"),
            Cell::new("State"),
            Cell::new("Stack"),
            Cell::new("Output"),
            Cell::new("Input"),
            Cell::new("Action"),

        ]));
        for (step, parser) in history.iter().enumerate() {
            table.add_row(parser.display(&machine).get_row(step));
        }

        table.printstd();

        println!("Press right arrow to view next step, left arrow to go back 1 step, down arrow to exit, up arrow to reset");
        let Ok(key_press) = g.getch() else {break};
        match key_press {
            getch_rs::Key::Right => {
                let Some(next_step) = history.last().unwrap().get_next(&machine) else {
                    break;
                };
                history.push(next_step);
            },
            getch_rs::Key::Left => {
                if history.len() > 1 {
                    history.pop();
                }
            },
            getch_rs::Key::Down=> {
                return;
            }
            getch_rs::Key::Up=> {
                while history.len() > 1 {
                    history.pop();
                }
            }

            _ => continue
        }
        clear_screen();

    }

}