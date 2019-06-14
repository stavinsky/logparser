use std::collections::HashMap;
use nom::IResult;
use nom::error::{ParseError, VerboseError};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while, escaped_transform};
use nom::sequence::{preceded, separated_pair, delimited};
use nom::combinator::{map, opt};
use nom::number::complete::float;
use nom::multi::many0;
use crate::types::Value;
use nom::multi::separated_list;
fn comment<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, &'a str, E> {
    preceded(tag("#"), is_not("\n"))(input)
}
pub fn s<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
  let chars = " \t\r\n";
  take_while(move |c| chars.contains(c))(i)
}
pub fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let (i, _) = opt(preceded(s, comment))(i)?;
    s(i)
}
pub fn snake_case<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, String, E>{
    map(
        take_while(
            move |s:char|  "_".contains(s) || s.is_alphabetic()
        ),
        String::from
    )(input)
}


fn boolean<'a ,E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, bool, E>
{
    alt((
        map(tag("true"), |_| true),
        map(tag("false"), |_| false),
    ))(input)
}
pub fn string<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, String, E> {
    delimited(
        tag("\""),
        map(
            escaped_transform(
                is_not("\"\t\r\n\\"), '\\',
                |i:&str| alt!(i,
                    tag!("r") => { |_| "\r" }
                    | tag!("n") => { |_| "\n" }
                    | tag!("t") => { |_| "\t" }
                    | tag!("\\") => { |_| "\\" }
                    | tag!("\"") => { |_| "\"" }
                    | tag!("\'") => { |_| "\'" }
                    | tag!("%") => { |_| "\\%" }
                    | tag!(".") => { |_| "\\." }
                    | tag!("s") => { |_| "\\s" }
                    | tag!("d") => { |_| "\\d" }
                    | tag!(":") => { |_| "\\:" }
                    | tag!("(") => { |_| "\\(" }
                    | tag!(")") => { |_| "\\)" }

                )
            ),
            String::from
        ),
        tag("\""),
    )(input)
}
fn list<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Vec<Value>, E> {
    delimited(
        preceded(sp, tag("[")),
        separated_list(preceded(sp, tag(",")), value),
        preceded(sp, tag("]")),
    )(input)
}

pub fn value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, Value, E> {
    preceded(sp, alt((
        map(string, Value::Str),
        map(list, Value::List),
        map(hash, Value::Hash),
        map(boolean, Value::Boolean),
        map(number, Value::Number),
    )))(input)
}

fn hash_entry<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, (String, Value), E>{
    preceded(sp,
        separated_pair(
            preceded(sp, string),
            preceded(sp, tag("=>")),
            preceded(sp, value),
        )
    )(input)
}

fn hash<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&str, HashMap<String, Value>, E> {
    preceded(sp, map(
        delimited(
            preceded(sp, tag("{")),
            preceded(sp, many0(hash_entry)),
            preceded(sp, tag("}"))
        ),
        |v| {
            let mut h = HashMap::new();
            for (key, value) in v {
                h.insert(key, value);
            }
            h
        }
    ))(input)
}

fn number<'a, E: ParseError<&'a str>>(s: &'a str) -> IResult<&str, f32, E> {
    float(s)
}

#[test]
fn test_string_simple() {
    let s = r#""string""#;
    let expected = String::from("string");
    assert_eq!(
        string::<VerboseError<&str>>(&s).unwrap().1,
        expected
    );
}
#[test]
fn test_string_quote() {
    let s = r#""stri\"ng""#;
    let expected = String::from("stri\"ng");
    assert_eq!(
        string::<VerboseError<&str>>(&s).unwrap().1,
        expected
    );
}
#[test]
fn test_string_tab() {
    let s = r#""stri\tng""#;
    let expected = String::from("stri\tng");
    assert_eq!(
        string::<VerboseError<&str>>(&s).unwrap().1,
        expected
    );
}
#[test]
fn test_string_single_quote() {
    let s = r#""stri\'ng""#;
    let expected = String::from("stri\'ng");
    assert_eq!(
        string::<VerboseError<&str>>(&s).unwrap().1,
        expected
    );
}
#[test]
fn test_string_new_line() {
    let s = r#""stri\nng""#;
    let expected = String::from("stri\nng");
    assert_eq!(
        string::<VerboseError<&str>>(&s).unwrap().1,
        expected
    );
}
#[test]
fn test_string_backslash() {
    let s = r#""stri\\ng""#;
    let expected = String::from("stri\\ng");
    assert_eq!(
        string::<VerboseError<&str>>(&s).unwrap().1,
        expected
    );
}

#[test]
fn test_list() {
    let s = r#"[ "test1",  "test2"]"#;
    let mut expected = Vec::new();
    expected.push(Value::Str(String::from("test1")));
    expected.push(Value::Str(String::from("test2")));
    assert_eq!(
        list::<VerboseError<&str>>(&s).unwrap().1,
        expected
    );
}
#[test]
fn test_hash() {
    let s = r#"
{ "mt_mon_connections" => "integer"
"mt_mon_connections" => "integer"
}
"#;
    let mut expected = HashMap::new();
    expected.insert(
        String::from("mt_mon_connections"),
        Value::Str(String::from("integer"))
    );
    expected.insert(
        String::from("mt_mon_connections"),
        Value::Str(String::from("integer"))
    );
    assert_eq!(
        hash::<VerboseError<&str>>(&s).unwrap().1,
        expected
    );
}
#[test]
fn test_bool() {
    let s = "true";
    let expected = true;

    assert_eq!(boolean::<VerboseError<&str>>(&s).unwrap().1, expected);
}

#[test]
fn test_number() {
    let s = String::from("12345.0");
    assert_eq!(number::<VerboseError<&str>>(&s).unwrap().1, 12345.0f32);
}

// TODO:
// base tipes:
// 1. array
// done 2. list
// done 3. bool
// done 4. hash
// done 5. number
// done 6. string
// 7. comments
//
//
