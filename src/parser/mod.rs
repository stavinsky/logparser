pub mod parsers;
pub mod types;
use crate::types::Value;
use parsers::{value, sp, snake_case};
use std::collections::HashMap;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{ preceded, separated_pair, delimited, tuple};
use nom::bytes::complete::{tag, take_while};
use nom::multi::many0;
use nom::bytes::complete::take_until;
use nom::combinator::opt;
use nom::error::{ParseError, VerboseError};
use nom::branch::alt;
use crate::parser::types::{
//    Plugin,
    Condition,
    ConditionExpression,
    Statement,
    Block,
    Param,
    Section,
    ElseIf,
    Parser,
};
use crate::plugins::PluginRegistry;
use crate::plugins::Plugin;
fn param_entry(input: &str) -> IResult<&str, Param>{
    separated_pair(
        preceded(sp, snake_case),
        preceded(sp, tag("=>")),
        preceded(sp, value)
    )(input)
}
fn params(input: &str) -> IResult<&str, Vec<Param>>{
    delimited(
        preceded(sp, tag("{")),
        preceded(sp, many0(param_entry)),
        preceded(sp, tag("}")),
    )(input)

}

fn plugin(r: PluginRegistry) -> impl Fn(&str) -> IResult<&str, Box<dyn Plugin>> {
    move |input: &str| {
        use nom::Err;
        use nom::error::ErrorKind;
        let (input, (name, params)) = tuple((
                preceded(sp, snake_case),
                preceded(sp, params)))(input)?;
        if let Some(plugin) = r.new_plugin(name, params){
            Ok((input, plugin))
        }
        else {
            Err(Err::Failure((input, ErrorKind::Tag)))
        }
    }
}
fn statement(r: PluginRegistry) -> impl Fn(&str)-> IResult<&str, Statement> {
    move|input: &str| {
        alt((
            map(plugin(r.clone()), Statement::Plugin),
            map(condition(r.clone()), Statement::Condition),
        ))(input)
    }

}
fn block(r: PluginRegistry) -> impl Fn (&str)-> IResult<&str, Block> {
    move |input: &str| {
        map(
            delimited (
                preceded(sp, tag("{")),
                preceded(sp, many0(statement(r.clone()))),
                preceded(sp, tag("}")),
            ), |plugins| {
                let mut block = Block::new();
                for plugin in plugins {
                    block.push(plugin)
                }
                block
            }
        )(input)
    }
}
fn condition(r: PluginRegistry) -> impl Fn(&str)-> IResult<&str, Condition> {
    move |input: &str| {
        map(
            tuple((
                preceded(sp, tag("if")), // r.0
                preceded(sp, take_until("{")), // r.1 if expression
                preceded(sp, block(r.clone())), // r.2 block_if
                preceded(sp, many0(block_else_if(r.clone()))), // r3. block_if_else
                preceded(sp, opt(block_else(r.clone()))) // r.4 block_if_else

            )),
            |r| Condition::new(r.1, r.2, r.3, r.4.unwrap_or_default())
        )(input)
    }
}
fn block_else(r: PluginRegistry) -> impl Fn(&str)-> IResult<&str, Block> {
    move |input: &str|
    preceded(preceded(sp, tag("else")),
        preceded(sp, block(r.clone()))
    )(input)
}
fn block_else_if(r: PluginRegistry) -> impl Fn(&str) -> IResult<&str, ElseIf> {
    move |input: &str| {
        map(
            tuple((
                preceded(sp, tag("else")),
                preceded(sp, tag("if")),
                preceded(sp, take_until("{")),
                preceded(sp, block(r.clone())),
            )),
            |r| ElseIf::new(r.2, r.3)
        )(input)
    }
}
fn read_section (r: PluginRegistry) -> impl Fn(&str)-> IResult<&str, Section>{
    move |input: &str| {
        preceded(sp, alt((
            map(preceded(tag("input"), block(r.clone())), Section::Input),
            map(preceded(tag("filter"), block(r.clone())), Section::Filter),
            map(preceded(tag("output"), block(r.clone())), Section::Output),

        )))(input)
    }
}
pub fn read_config(input: &str) -> IResult<&str, Parser>{
    let r = PluginRegistry::new();
    let mut parser = Parser::new();
    let (input, sections) = many0(read_section(r))(input)?;
    for section in sections {
        parser.add_section(section);
    }
    Ok((input, parser))
}
#[test]
fn test_param_entry(){
    let s = r#"
      add_field => {
        "mt_full_date" => "%{mt_date} %{mt_time}"
      }
    "#;
    let mut inner_hash= HashMap::new();
    inner_hash.insert(
        String::from("mt_full_date"),
        Value::Str(String::from("%{mt_date} %{mt_time}"))
    );
    let expected = (
        String::from("add_field"),
        Value::Hash(inner_hash),
    );
    assert_eq!(
        param_entry(&s).unwrap().1,
        expected
    );

}

#[test]
fn test_plugin(){
    let _s = r#"
grok {
    match => {
        "message" => "max. memory block: %{INT:mt_mon_max_memory_block} kb process cpu: %{INT:mt_mon_process_cpu}\%  net in: %{INT:mt_mon_network_in} Kbyte/s  net out: %{INT:mt_mon_network_out} Kbyte/s"
    }
    overwrite => "message"
}
"#;
    let mut expected = Vec::new();
    let mut internal_value = HashMap::new();
    internal_value.insert(
        String::from("message"),
        Value::Str(String::from("max. memory block: %{INT:mt_mon_max_memory_block} kb process cpu: %{INT:mt_mon_process_cpu}\\%  net in: %{INT:mt_mon_network_in} Kbyte/s  net out: %{INT:mt_mon_network_out} Kbyte/s")));
    expected.push((
        String::from("match"),
        Value::Hash(internal_value)
    ));
    expected.push((
        String::from("overwrite"),
        Value::Str(String::from("message"))
    ));
//    assert_eq!(
//        plugin(&s).unwrap().1,
//        Plugin::new(String::from("grok"), expected)
//    );
//    TODO: Fix This
}
#[test]
fn test_if_block() {
   let s = r#"
if EXPRESSION1 {
    mutate {
        convert => {
            "dl_exec_time_ms1" => "integer"
        }
    }
} else if EXPRESSION2 {
    mutate {
        convert => {
            "dl_exec_time_ms2" => "integer"
        }
    }
    if Expression4 {
        mutate {
            convert => {
                "folded_if" => "integer"
            }
        }


    }
} else {
    mutate {
        convert => {
            "dl_exec_time_ms3" => "integer"
        }
    }
}
"#;
    use std::rc::Rc;
    use crate::plugins::Mutate;
    let r = PluginRegistry::new();
    r.register_plugin("mutate", Rc::new(|params|Mutate::new(params)));
    let res = condition(r)(s).unwrap();
    std::dbg!(res);

}
#[test]
fn test_read_config(){
    use std::rc::Rc;
    use crate::plugins::{Mutate, Grok, Date};
    let r = PluginRegistry::new();
    r.register_plugin("mutate", Rc::new(|params|Mutate::new(params)));
    r.register_plugin("grok", Rc::new(|params|Grok::new(params)));
    r.register_plugin("date", Rc::new(|params|Date::new(params)));

    use std::fs;
    let s = fs::read_to_string("test.conf").unwrap();
    let r = read_section(r)(&s).unwrap().1;
    std::dbg!(r);
}

