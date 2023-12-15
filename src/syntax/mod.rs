mod display;
mod convert;

use std::{char, ops::Index, slice::SliceIndex};

pub const END_TERMINAL: char =  '\u{FDD0}';
pub const END_VARIABLE: char =  '\u{FDD1}';

pub const EPSILON: char = '\u{03B5}';
#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Terminal{
    pub symbol: char
}

impl Terminal {
    pub const fn end() -> Terminal {
        Terminal{symbol: END_TERMINAL}
    }
    pub const fn epsilon() -> Terminal {Terminal{symbol: EPSILON}}
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Variable{
    pub symbol: char
}
impl Variable {
    pub const fn accept() -> Variable {
        Variable{symbol: END_VARIABLE}
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum MixedChar{
    Terminal(Terminal),
    Variable(Variable)
}

#[derive(Clone)]
pub struct MixedString{
    pub data: Vec<MixedChar>
}
impl<I: SliceIndex<[MixedChar]>> Index<I> for MixedString {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}
impl MixedString {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
    pub fn push_variable(&mut self, symbol: Variable) {
        self.data.push(MixedChar::Variable(symbol ));
    }
    pub fn push_terminal(&mut self, symbol: Terminal) {
        self.data.push(MixedChar::Terminal(symbol));
    }
    pub fn pop(&mut self) {
        self.data.pop();
    }
}


#[derive(Clone)]
pub struct TerminalString{
    pub data: Vec<Terminal>
}

impl TerminalString {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
    pub fn push_char(&mut self, symbol: char) {
        self.data.push(Terminal { symbol });
    }
    pub fn push_terminal(&mut self, symbol: Terminal) {
        self.data.push(symbol);
    }
    pub fn pop(&mut self) {
        self.data.pop();
    }
    pub fn get(&self, index: usize) -> Option<Terminal> {
        self.data.get(index).copied()
    }
}

impl<I: SliceIndex<[Terminal]>> Index<I> for TerminalString {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}


#[derive(Clone)]
pub struct Rule{
    pub clause: Variable,
    pub output: MixedString
}

impl Rule {
    pub fn new(start: char) -> Self {
        Self { 
            clause: Variable{symbol: start }, 
            output: MixedString{data: Vec::new()}
        }
    }
    pub fn add_terminal(&mut self,terminal: char){
        self.output.data.push(MixedChar::Terminal(Terminal { symbol: terminal }))
    }

    pub fn add_variable(&mut self,variable: char){
        self.output.data.push(MixedChar::Variable(Variable { symbol: variable }))
    }

    pub fn len(&self) -> usize {
        self.output.data.len()
    }
}

