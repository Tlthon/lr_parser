#![allow(dead_code)]

use std::{env, fs};

use prettytable::{Cell, Row, Table};

use crate::{parsing::ParsingProcess};
use crate::parsing_table::IStateMachine;

pub mod itemset;
pub mod syntax;
mod parsing_table;
mod parsing;
pub mod rule_depend;
mod tarjan;
mod data_structure;
pub mod first_follow;

fn main() {
    let file_path = "rule.txt";
    println!("read rule from file: {file_path}");

    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(input) if input == "lr_zero" => lr_zero(file_path),
        _ => lr_one(file_path),
    }
}

fn lr_zero(file_path: &str) {
    use crate::{itemset::lr_zero::ItemSets, parsing_table::lr_zero::StateMachine};
    let mut itemsets = ItemSets::new('E');

    for line in fs::read_to_string(file_path).unwrap().lines() {
        itemsets.add_from_string(line);
    }

    itemsets.generate_next();
    let machine = StateMachine::from_itemset(&itemsets);
    run_machine(&itemsets, machine);
}


fn lr_one(file_path: &str) {
    use crate::{itemset::lr_one::ItemSets, parsing_table::lr_one::StateMachine};
    let mut itemsets = ItemSets::new('E');

    for line in fs::read_to_string(file_path).unwrap().lines() {
        itemsets.add_from_string(line);
    }

    itemsets.generate_next();
    let machine = StateMachine::from_itemset(&itemsets);
    run_machine(&itemsets, machine);
}

fn run_machine<ItemSets, StateMachine>(itemset: &ItemSets, machine: StateMachine)
    where StateMachine: for<'a> parsing_table::IStateMachine<'a, ItemSets = ItemSets>{
    println!("{:20}", machine.display(&itemset));
    print!("\nTaking input\n");
    let mut line = std::io::stdin().lines().next().unwrap().unwrap();
    line.push(syntax::END_TERMINAL);
    let input_vec: Vec<char> = line.chars().collect();
    let parser = ParsingProcess::new(&input_vec);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    run_parsing(&machine, vec![parser]);

}

fn clear_screen() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn run_parsing<IStateMachine>(machine: &IStateMachine, mut history: Vec<ParsingProcess>)
where IStateMachine: for<'a> parsing_table::IStateMachine<'a>
{
    let g = getch_rs::Getch::new();
    loop {
        clear_screen();
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
            table.add_row(parser.display(machine).get_row(step));
        }

        table.printstd();

        println!("Press right arrow to view next step, left arrow to go back 1 step, down arrow to exit, up arrow to reset");
        let Ok(key_press) = g.getch() else {break};
        match key_press {
            getch_rs::Key::Right => {
                let Some(next_step) = history.last().unwrap().get_next(machine) else {
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


    }

}
