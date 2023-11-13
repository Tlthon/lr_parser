use crate::{parsingtable::StateMachine, syntax::Rule, parsing::ParsingProcess, itemset::ItemSets};
pub mod itemset;
pub mod syntax;
mod parsingtable;
mod parsing;
pub mod ruledepend;

fn main() {

    let mut itemset = ItemSets::new();
    {
        let mut rule = Rule::new(syntax::END_VARIABLE);
        rule.add_variable('E');
        rule.add_terminal(syntax::END_TERMINAL);
        itemset.add_rule(rule);
    }
    {
        let mut rule = Rule::new('E');
        rule.add_variable('E');
        rule.add_terminal('*');
        rule.add_variable('B');
        itemset.add_rule(rule);
    }
    {
        let mut rule = Rule::new('E');
        rule.add_variable('E');
        rule.add_terminal('+');
        rule.add_variable('B');
        itemset.add_rule(rule);
    }
    {
        let mut rule = Rule::new('E');
        rule.add_variable('B');
        itemset.add_rule(rule);
    }
    {
        let mut rule: Rule = Rule::new('B');
        rule.add_terminal('1');
        itemset.add_rule(rule);
    }
    {
        let mut rule = Rule::new('B');
        rule.add_terminal('0');
        itemset.add_rule(rule);
    }
    {
        let mut rule = Rule::new('E');
        rule.add_terminal('(');
        rule.add_variable('E');
        rule.add_terminal(')');
        itemset.add_rule(rule);
    }



    itemset.generate_next();

    println!("{}",itemset);

    let machine = StateMachine::from_itemset(itemset);
    println!("{}", machine);

    let input_vec: Vec<char> = format!("(1){}",syntax::END_TERMINAL).chars().collect();
    let parser = ParsingProcess::new(&input_vec);
    let g = getch_rs::Getch::new();
    let mut history = vec![parser];
    loop {
        // println!("{}", parser.display(&machine));
        for parser in &history {
            println!("{}", parser.display(&machine));
        }
        
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
