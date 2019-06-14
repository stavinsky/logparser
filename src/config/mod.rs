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
use crate::config::types::{
    Plugin,
    Condition,
    ConditionExpression,
    Statement,
    Block,
    Param,
    Section,
    ElseIf,
};

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
fn plugin(input: &str) -> IResult<&str, Plugin>{
    let (input, (name, params)) = tuple((
            preceded(sp, snake_case),
            preceded(sp, params)))(input)?;
    Ok((input, Plugin::new(name, params)))
}
fn statement(input: &str) -> IResult<&str, Statement> {
    alt((
        map(plugin, Statement::Plugin),
        map(condition, Statement::Condition),

    ))(input)

}
fn block(input: &str) -> IResult<&str, Block> {
    map(
        delimited (
            preceded(sp, tag("{")),
            preceded(sp, many0(statement)),
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
fn condition(input: &str) -> IResult<&str, Condition> {
    map(
        tuple((
            preceded(sp, tag("if")), // r.0
            preceded(sp, take_until("{")), // r.1 if expression
            preceded(sp, block), // r.2 block_if
            preceded(sp, many0(block_else_if)), // r3. block_if_else
            preceded(sp, opt(block_else)) // r.4 block_if_else

        )),
        |r| Condition::new(r.1, r.2, r.3, r.4.unwrap_or_default())
    )(input)

}
fn block_else(input: &str) -> IResult<&str, Block> {
    preceded(preceded(sp, tag("else")),
        preceded(sp, block)
    )(input)
}
fn block_else_if(input: &str) -> IResult<&str, ElseIf> {
    map(
        tuple((
            preceded(sp, tag("else")),
            preceded(sp, tag("if")),
            preceded(sp, take_until("{")),
            preceded(sp, block),
        )),
        |r| ElseIf::new(r.2, r.3)
    )(input)
}
fn read_section (input: &str) -> IResult<&str, Section>{
    preceded(sp, alt((
        map(preceded(tag("input"), block), Section::Input),
        map(preceded(tag("filter"), block), Section::Filter),
        map(preceded(tag("output"), block), Section::Output),

    )))(input)
}
fn read_config(input: &str) -> IResult<&str, Vec<Section>>{
    many0(read_section)(input)
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
    let s = r#"
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
    assert_eq!(
        plugin(&s).unwrap().1,
        Plugin::new(String::from("grok"), expected)
    );
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
let res = condition(s).unwrap();
std::dbg!(res);

}
#[test]
fn test_read_config(){

    use std::fs;
    let s = fs::read_to_string("test.conf").unwrap();
    let r = read_section(&s).unwrap().1;
    std::dbg!(r);
}
// todo:
// filter ->
//          if conditions {
//              plugin
//          }
//          plugin ->
//                   params: basic-types
//
//
//
