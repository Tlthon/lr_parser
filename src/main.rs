use crate::{parsingtable::StateMachine, syntax::Rule, parsing::solving, itemset::ItemSets};
pub mod itemset;
pub mod syntax;
mod parsingtable;
mod parsing;
pub mod ruledepend;

fn main() {

    let mut itemset = ItemSets::new();
    // let mut itemset = ;
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

    // println!("{}", machine);
    let input_vec: Vec<char> = format!("(1)+1{}",syntax::END_TERMINAL).chars().collect();
    solving(&input_vec, machine)
       
}
