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
    itemset.add_from_string("E:T");
    itemset.add_from_string("E:(E)");
    itemset.add_from_string("T:n");
    itemset.add_from_string("T:T+n");

    // itemset.add_from_string("N:MN");
    // itemset.add_from_string("N:");

    // itemset.add_from_string("M:0");
    // itemset.add_from_string("M:1");
    // itemset.add_from_string("M:2");
    // itemset.add_from_string("M:3");
    // itemset.add_from_string("M:4");
    // itemset.add_from_string("M:5");
    // itemset.add_from_string("M:6");
    // itemset.add_from_string("M:7");
    // itemset.add_from_string("M:8");
    // itemset.add_from_string("M:9");

    itemset.generate_next();

    // println!("{}",itemset);

    let machine = StateMachine::from_itemset(&itemset);

    println!("{}", machine.display(&itemset));
    print!("\nTaking input\n");
    let mut line = std::io::stdin().lines().next().unwrap().unwrap();
    line.push(syntax::END_TERMINAL);
    let input_vec: Vec<char> = line.chars().collect();
    let parser = ParsingProcess::new(&input_vec);
    let g = getch_rs::Getch::new();
    let mut history = vec![parser];
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    loop {
        // println!("{}", parser.display(&machine));
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

        println!("Press right arrow to view next step, left arrow to go back 1 step");
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
            _ => return,
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    }
}
