#![allow(dead_code, unused_imports)]

extern crate grok;
//extern crate eval;
extern crate parking_lot;

//use std::io;
//use std::io::BufReader;
//use std::io::BufRead;
//use eval::{Expr, to_value};
use std::sync::{Arc};
mod plugins;
//mod cache;
//mod helper;
//use helper::Helper;
mod parser;
#[macro_use]
extern crate nom;
mod eval;
mod types;
use parser::read_config;
fn main() {


}


// fn main() {
//     let stdin = io::stdin();
//     let reader = BufReader::new(stdin);
//     let helper = Helper::new();
//     let helper = Arc::new(helper);
//     for line in reader.lines().filter_map(Result::ok) {
//         let helper = Arc::clone(&helper);
//         let res = Expr::new(r#"regex_match('[0-9]\t.*DealerLogic.*', str)"#)
//                     .function("regex_match",   move |args| {
//                         let helper = &helper;
//                         Ok(to_value(&helper.regex_match(&args[0], &args[1])))
//                     })
//                     .value("str", line)
//                     .exec();
//         println!("{:?}", res);
//     }
// }
