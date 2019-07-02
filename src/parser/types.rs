use crate::types::Value;
use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

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


pub type Func = Rc<dyn Fn(Vec<Param>) ->()> ;
#[derive(Clone)]
pub struct PluginRegistry {
    plugins: Rc<RwLock<HashMap<String, Func>>>
}
impl PluginRegistry {
    pub  fn new() -> Self{
        Self{
            plugins: Rc::new(RwLock::new(HashMap::new()))
        }
    }
    pub fn new_plugin(&self, name: String, params: Vec<Param>) -> Option<Plugin> {
        let plugins = self.plugins.read().unwrap();
        if let Some(func) = plugins.get(&name) {
            let func = func.to_owned();
            Some(Plugin {
                name, params, func
            })
        }
        else {
            None
        }
    }
    pub fn register_plugin(&self, name: &str, f: Func) -> (){
        let mut plugins = self.plugins.write().unwrap();
        plugins.insert(name.to_owned(), f.clone());

    }
}
pub struct Plugin {
    params: Vec<Param>,
    name: String,
    func: Func,
}

impl Plugin {
//    pub fn new(name: String, params: Vec<Param>) -> Self {
//        Plugin{ name, params, func: Rc::new(|params| {})}
//    }

}
impl fmt::Debug for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plugin {{ name: {:?}, params: {:?} }}", self.name, self.params)
    }
}
impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.params == other.params
    }
}
