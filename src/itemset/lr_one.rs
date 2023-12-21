use std::collections::{HashMap, HashSet};
use crate::rule_depend::RuleGraph;
use crate::{first_follow, syntax};
use crate::itemset::item_lookahead::ItemSet;
use crate::syntax::{MixedChar, Rule};
use crate::syntax::Variable;

// mod display;

pub struct ItemSets {
    pub sets: Vec<ItemSet>,
    pub rules: Vec<Rule>,
    pub ordering_map: Vec<Vec<(MixedChar, usize)>>,
}

impl super::ItemSets<'_> for ItemSets {
    type Item = super::item_lookahead::Item;
    type ItemSet = ItemSet;
    fn item_sets(&self) -> &[Self::ItemSet] { &self.sets }
    fn rules(&self) -> &[Rule] { &self.rules }
    fn ordering_map(&self) -> &[Vec<(MixedChar, usize)>] {
        self.ordering_map.as_slice()
    }
}

impl super::LookaheadItemSets<'_> for ItemSets {}

impl ItemSets {
    pub fn new(last_variable: char) -> Self {
        let mut first_rule = Rule::new(syntax::END_VARIABLE);
        first_rule.add_variable(last_variable);
        Self { rules: vec![first_rule], sets: Vec::default(), ordering_map: Vec::default() }
    }
    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn add_from_string(&mut self, string_rule: &str) -> bool{
        let Ok(rule) = string_rule.try_into() else {
            return false
        };
        self.rules.push(rule);
        true
    }

    pub fn clear(&mut self) {
        self.sets = Vec::default();
        self.ordering_map = Vec::default();
    }

    pub fn generate_next(&mut self){
        let mut itemmaps = HashMap::new();
        let rule_graph = RuleGraph::new(self.rules.clone());
        let mut first_item = ItemSet::new();
        let mut index = 0;
        first_item.add_kernel(&self.rules[0], 0, 0, &[syntax::Terminal::end()]);
        let first = first_follow::First::from_rule(&self.rules);
        let follows = first_follow::Follow::new(&first, &self.rules);
        first_item.add_non_kernel(&rule_graph, &self.rules, &follows, None);
        self.sets.push(first_item);
        loop {
            let Some(cur_item) = self.sets.get(index).cloned() else {
                break;
            };
            let symbols = & mut cur_item.symbols.clone();
            let mut next_val = Vec::new();
            for transition_char in symbols.iter(){
                let new_itemset: Option<ItemSet> = cur_item.transitions(*transition_char, &self.rules);
                if let Some(mut new_itemset) = new_itemset {
                    new_itemset.add_non_kernel(&rule_graph, &self.rules, &follows, Some(&cur_item));
                    if let Some(new_index) = itemmaps.get(&new_itemset) {
                        next_val.push((transition_char.clone(), *new_index));
                        continue;
                    }
                    itemmaps.insert(new_itemset.clone(), self.sets.len());
                    next_val.push((transition_char.clone(), self.sets.len()));
                    self.sets.push(new_itemset);
                }
            }
            self.ordering_map.push(next_val);
            index += 1;
        }
    }

    pub fn generate_lalr(&mut self){
        use crate::itemset::item_no_lookahead::ItemSet as itemset_no_lookahead;
        let mut itemmaps = HashMap::new();
        let mut kernel_maps:HashMap<itemset_no_lookahead, usize> = HashMap::default();
        let mut same_kernels = vec![];
        let mut prev = vec![];
        let mut sets = vec![];

        let rule_graph = RuleGraph::new(self.rules.clone());
        let mut index = 0;
        let mut ordering_map = vec![];

        let first = first_follow::First::from_rule(&self.rules);
        let follows = first_follow::Follow::new(&first, &self.rules);
        let mut track_kernel = |kernels: itemset_no_lookahead, index: usize| {
            {
                if let Some(previous) = kernel_maps.get(&kernels) {
                    same_kernels.push((index, *previous));
                }
                kernel_maps.insert(kernels, index);
            }
        };

        {
            let mut first_item = ItemSet::new();
            first_item.add_kernel(&self.rules[0], 0, 0, &[syntax::Terminal::end()]);
            track_kernel((&first_item).into(), 0);
            first_item.add_non_kernel(&rule_graph, &self.rules, &follows, None);
            sets.push(first_item);
            prev.push(None)
        }

        while let Some(cur_item) = sets.get(index).cloned() {

            let symbols = &mut cur_item.symbols.clone();
            let mut next_val = HashMap::new();
            for transition_char in symbols.iter() {
                if let Some(mut new_itemset) = cur_item.transitions(*transition_char, &self.rules) {
                    if let Some(matched_index) = itemmaps.get(&new_itemset) {
                        next_val.insert(*matched_index, *transition_char);
                        continue;

                    }
                    let new_index = sets.len();
                    itemmaps.insert(new_itemset.clone(), new_index);

                    track_kernel((&new_itemset).into(), new_index);
                    new_itemset.add_non_kernel(&rule_graph, &self.rules, &follows, Some(&cur_item));
                    sets.push(new_itemset);

                    prev.push(Some(index));
                    next_val.insert(new_index, *transition_char);
                }
            }
            ordering_map.push(next_val);
            index += 1;
        }
        let mut translation = vec![];
        let mut unuseds = HashSet::new();
        let mut j = 0;

        for (outer, into) in same_kernels {
            unuseds.insert(outer);
            while translation.len() < outer {
                translation.push(j);
                j+=1;
            }
            translation.push(translation[into]);
            let old_itemset = sets[outer].clone();
            sets[into].merge(old_itemset);
        }
        while translation.len() < sets.len() {
            translation.push(j);
            j += 1;
        }
        let mut temp = vec![];
        for (id, ordering_map) in ordering_map.iter().enumerate() {
            if unuseds.contains(&id) {
                continue;
            }

            self.ordering_map.push(ordering_map.iter().map(|(index, j)|
                (*j,translation[*index])).collect()
            );

            temp.push(ordering_map.iter().map(|(index, j)| (*j,*index)).collect::<Vec<_>>());
        }

        for (id,set) in sets.iter().enumerate() {
            if unuseds.contains(&id) {
                continue;
            }
            self.sets.push(set.clone())
        }
    }

}
