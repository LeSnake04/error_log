#![cfg(test)]
#![cfg(feature = "errors")]
use std::num::ParseIntError;

use error_log::{try_add, try_merge, ErrorLog};
#[test]
fn main() {
    let out = run();
    assert!(!out.entries().is_empty());
    assert!(out.ok().is_none());
    let out2 = run_merge();
    assert!(out2.ok().is_none())
}

pub fn run() -> ErrorLog<String, ParseIntError> {
    let mut err_log = ErrorLog::new();
    try_add!("abc".parse::<i32>(), err_log);
    err_log
}

pub fn run_merge() -> ErrorLog<i32, ParseIntError> {
    let mut err_log = ErrorLog::new();
    try_merge!("abc".parse::<i32>(), err_log);
    err_log
}
