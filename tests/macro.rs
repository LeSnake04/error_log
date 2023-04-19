#![cfg(feature = "errors")]
use std::num::ParseIntError;

use error_log::{return_err, return_ok, try_add, try_merge, ErrorLog};
use pretty_assertions::assert_eq;

#[test]
fn macros() {
    let out = try_add();
    assert_eq!(out.entries().len(), 1);
    assert!(out.ok().is_none());
    let out2 = try_merge();
    assert_eq!(out2.entries().len(), 1);
    assert!(out2.ok().is_none());
    let out3 = return_ok();
    assert_eq!(out3.ok(), &Some("ok".to_string()));
    assert!(out3.entries().is_empty());
    let out4 = return_err();
    assert_eq!(out4.entries().len(), 1);
    assert!(out4.ok().is_none());
}

pub fn return_ok() -> ErrorLog<String, ParseIntError> {
    let mut err_log = ErrorLog::new();
    return_ok!("ok", err_log);
}

pub fn return_err() -> ErrorLog<String, ParseIntError> {
    let mut err_log = ErrorLog::new();
    let err = "abc".parse::<i32>().err().unwrap();
    return_err!(err, err_log);
}

pub fn try_add() -> ErrorLog<String, ParseIntError> {
    let mut err_log = ErrorLog::new();
    try_add!("abc".parse::<i32>(), err_log);
    err_log
}
pub fn try_add_ok() -> ErrorLog<String, ParseIntError> {
    let mut err_log = ErrorLog::new();
    try_add!("12".parse::<i32>(), err_log);
    err_log
}

pub fn try_merge() -> ErrorLog<i32, ParseIntError> {
    let mut err_log = ErrorLog::new();
    try_merge!("abc".parse::<i32>(), err_log);
    err_log
}
pub fn try_merge_ok() -> ErrorLog<i32, ParseIntError> {
    let mut err_log = ErrorLog::new();
    try_merge!("12".parse::<i32>(), err_log);
    err_log
}
