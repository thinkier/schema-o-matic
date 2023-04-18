#[macro_use]
extern crate argh;
#[macro_use]
extern crate serde_derive;

use std::fs;
use std::io::{Read, stdin};
use serde_json::Value;
use crate::imp::json_schema::ClearDefault;
use crate::model::cli::CliArgs;
use crate::model::json_schema::{JsonSchema};

pub mod imp;
pub mod model;

fn main() {
    let args: CliArgs = argh::from_env();

    let read = {
        if let Some(path) = &args.input {
            fs::read_to_string(path).expect("Unable to read file")
        } else {
            let mut str = String::new();
            stdin().read_to_string(&mut str).expect("Unable to read stdin");

            str
        }
    };

    let object: Value = serde_json::from_str(&read).expect("Unable to parse input as json");

    let mut schema = JsonSchema::from_value(&object);

    if !args.show_default {
        schema.clear_default();
    }

    let schema_json = serde_json::to_string_pretty(&schema).unwrap();

    if let Some(dest) = args.output {
        fs::write(dest, schema_json).expect("Unable to write to file");
    } else {
        println!("{}", schema_json);
    }
}
