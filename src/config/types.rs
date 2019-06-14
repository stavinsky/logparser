use crate::types::Value;


#[derive(Debug, PartialEq)]
pub struct Plugin {
    params: Vec<Param>,
    name: String,
}
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
#[derive(Debug, PartialEq)]
pub enum Statement {
    Plugin(Plugin),
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
impl Plugin {
    pub fn new(name: String, params: Vec<Param>) -> Self {
        Plugin{ name, params }
    }
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
