#![allow(dead_code)]

extern crate ferris_says;

use std::io::{BufWriter, stdout};
use ferris_says::say;

mod rustest;

fn test1() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();
    let mut write = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut write).unwrap();
    println!("Hello, world!");
}

fn main() {
    // test1();
    // rustest::test_structs::test_object();
    println!("==========test format==========");
    // rustest::test_structs::test_format();
    // rustest::test_structs::test_struct();
    // rustest::test_number::test_int();
    // rustest::test_enum::test_enum();
    // rustest::test_collections::test_select();
    // rustest::test_collections::test_hash_map();
    // rustest::test_error::test_error_handle();
    rustest::test_collections::test_list();
}
