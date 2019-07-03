use crate::types::Value;
use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;
use crate::plugins::Plugin;
use crate::plugins::PluginRegistry;
#[derive(Debug, PartialEq)]
pub struct ConditionExpression {
    raw_condition: String
}
#[derive(Debug, PartialEq)]
pub struct ElseIf {
   condition: ConditionExpression,
   block: Block,
}
#[derive(Debug, PartialEq)]
pub struct Condition{
    condition: ConditionExpression,
    block_if: Block,
    block_else_if: Vec<ElseIf>,
    block_else: Block,
}
pub enum Statement {
    Plugin(Box<dyn Plugin>),
    Condition(Condition),
}
pub type Block = Vec<Statement>;

pub type Param = (String, Value);

#[derive(Debug, PartialEq)]
pub enum Section {
    Input(Block),
    Filter(Block),
    Output(Block),
}
pub struct Parser{
    input: Block,
    output: Block,
    filter: Block,

}
impl Condition{
    pub fn new(
        condition_string: &str,
        block_if: Block,
        block_else_if: Vec<ElseIf>,
        block_else: Block
    ) -> Self {
        Condition{
            condition: ConditionExpression{
                raw_condition: String::from(condition_string),
            },
            block_if, block_else, block_else_if,
        }
    }
}

impl ElseIf {
    pub fn new(condition_string: &str, block: Block)-> Self {
        ElseIf{
            condition: ConditionExpression {
                raw_condition: String::from(condition_string),
            },
            block: block,
        }
    }
}
impl Parser {
    pub fn new() -> Self {
        Parser {
            input: Vec::new(),
            output: Vec::new(),
            filter: Vec::new(),
        }
    }
    pub fn add_section(&mut self, section: Section) -> (){
        match section{
            Section::Input(i) => self.input.extend(i),
            Section::Output(o) => self.output.extend(o),
            Section::Filter(f) => self.filter.extend(f),
        }
    }
}



impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Statement::*;
        match self {
            Plugin(p) => write!(f, "Plugin {{ name: {:?} }}", p.kind()),
            Condition(c) => write!(f, "{:?}", c)
        }
    }
}
impl PartialEq for Statement {
    fn eq(&self, other: &Self) -> bool {
        use Statement::*;
        match (self, other) {
            (Plugin(p), Plugin(o)) => p.kind() == o.kind(),
            (Condition(c), Condition(o)) => c==o,
            _ => false,
        }
    }
}
