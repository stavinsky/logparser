pub mod parsers;
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


#[derive(Debug, PartialEq)]
struct Plugin {
    params: Vec<Param>,
    name: String,
}
#[derive(Debug, PartialEq)]
struct ConditionExpression {
    raw_condition: String
}
#[derive(Debug, PartialEq)]
struct ElseIf {
   condition: ConditionExpression,
   block: Block,
}
#[derive(Debug, PartialEq)]
struct Condition{
    condition: ConditionExpression,
    block_if: Block,
    block_else_if: Vec<ElseIf>,
    block_else: Block,
}
#[derive(Debug, PartialEq)]
enum Unit {
    Plugin(Plugin),
    Condition(Condition),
}
type Block = Vec<Unit>;

type Param = (String, Value);

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
    map(
        tuple((
            preceded(sp, snake_case),
            preceded(sp, params)
        )),
        |t| Plugin{name: t.0, params: t.1}
    )(input)
}
fn unit(input: &str) -> IResult<&str, Unit> {
    alt((
        map(plugin, Unit::Plugin),
        map(condition, Unit::Condition),

    ))(input)

}
fn block(input: &str) -> IResult<&str, Block> {
    map(
        delimited (
            preceded(sp, tag("{")),
            preceded(sp, many0(unit)),
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
            preceded(sp, tag("if")),
            preceded(sp, take_until("{")),
            preceded(sp, block),
            preceded(sp, many0(block_else_if)),
            preceded(sp, opt(block_else))

        )),
        |r| {
            Condition{
                condition: ConditionExpression {
                    raw_condition: String::from(r.1),
                },
                block_if: r.2,
                block_else_if: r.3,
                block_else: r.4.unwrap_or_default(),
            }

        }
    )(input)

}
fn block_else(input: &str) -> IResult<&str, Block> {
    map(
        tuple((
            preceded(sp, tag("else")),
            preceded(sp, block)
        )),
        |r| r.1
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
        |r| ElseIf {
            condition: ConditionExpression{
                raw_condition: r.2.to_owned()
            },
            block: r.3

        }
    )(input)
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
        Plugin {
            name: String::from("grok"),
            params: expected
        }
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
