use std::{char, fmt::{Display, Debug}, ops::Index, slice::SliceIndex};

pub const END_TERMINAL: char =  '\u{FDD0}';
pub const END_VARIABLE: char =  '\u{FDD1}';

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Terminal{
    pub symbol: char
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.symbol == END_TERMINAL {
            return write!(f, "EOF");
            
        }
        write!(f, "{}", self.symbol)
    }
}
impl Debug for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.symbol == END_TERMINAL {
            return write!(f, "EOF");
            
        }
        write!(f, "{}", self.symbol)
    }
}

impl From<&char> for Terminal {
    fn from(value: &char) -> Self {
        Self { symbol: *value }
    }
}


#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Variable{
    pub symbol: char
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.symbol == END_VARIABLE {
            return write!(f, "ACCEPT");
            
        }
        write!(f, "{}", self.symbol)
    }
}
impl Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.symbol == END_VARIABLE {
            return write!(f, "ACCEPT");
            
        }
        write!(f, "{}", self.symbol)
    }
}



#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
pub enum MixedChar{
    Terminal(Terminal),
    Variable(Variable)
}

impl MixedChar {
    pub fn try_variable(self) -> Option<Variable> {
        match self {
            MixedChar::Terminal(_) => None,
            MixedChar::Variable(v) => Some(v),
        }
    }
}
impl From<Variable> for MixedChar {
    fn from(value: Variable) -> Self {
        MixedChar::Variable(value)
    }
}

impl Display for MixedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MixedChar::Terminal(t) => write!(f, "{}", t),
            MixedChar::Variable(v) => write!(f, "{}", v),
        }
    }
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

impl Display for MixedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for character in self.data.iter() {
            write!(f, "{}", character)?
        }
        return Ok(());
    }
}
impl Debug for MixedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for character in self.data.iter() {
            if first != true {
                write!(f, ", ")?
            }
            first = false;
            write!(f, "'{}'", character)?
        }
        write!(f, "]")?;
        return Ok(());
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

impl From<&[char]> for TerminalString {
    fn from(value: &[char]) -> Self {
        Self{
            data: value.iter().map(|symbol| Terminal{ symbol: *symbol }).collect()
        }
    }
}

impl Display for TerminalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for character in self.data.iter() {
            write!(f, "{}", character)?
        }
        return Ok(());
    }
}
impl Debug for TerminalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for character in self.data.iter() {
            if first != true {
                write!(f, ", ")?
            }
            first = false;
            write!(f, "'{}'", character)?
        }
        write!(f, "]")?;
        return Ok(());
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

pub struct Rules{
    pub rules: Vec<Rule>
}