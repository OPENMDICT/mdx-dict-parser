#![deny(clippy::all)]

use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate napi_derive;
use mdict_parser::parser;

#[napi(constructor)]
pub struct DictRecord {
  pub word: String,
  pub definition: String,
}

#[napi]
fn parse_mdict(file: String) -> Vec<DictRecord> {
  let mut file = File::open(file).expect("Unable to open file");
  let mut bytes = Vec::new();
  file.read_to_end(&mut bytes).expect("Unable to read file");

  let dict = parser::parse(&bytes);

  let mut records: Vec<DictRecord> = Vec::new();

  for item in dict.items() {
    let record = DictRecord {
      word: item.key.to_string(),
      definition: item.definition.to_string(),
    };
    records.push(record);
  }

  return records;
}
