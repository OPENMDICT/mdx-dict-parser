#![deny(clippy::all)]
use mdict_parser::{parser, mdict::Record};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn parse(){
  // let input = include_bytes!("../dictionary.mdx");
  // let dict = parser::parse(input);

  // // iter dictionary entries
  // for key in dict.keys() {
  //   println!("{:?}", item)
  // }

  // // iter all dictionary records
  // for item in dict.items() {
  //   println!("{:?}", item)
  // }
}
