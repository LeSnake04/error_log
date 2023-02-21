use std::num::ParseIntError;

use error_log::ErrorLog;

#[test]
fn main() {
    let out = run();
    assert_eq!(out.ok(), &Some(123));
    assert!(!out.errors().is_empty());
    // Get Number or if no Ok value set, output errors, then panic.
    let num: i32 = out.unwrap_or_display();
    println!("Out: {}", num);
}

fn run() -> ErrorLog<i32, ParseIntError> {
    let mut err_log = ErrorLog::new();
    // Add the potentital error from a funktion
    err_log.push_result(task());
    assert_eq!(err_log.errors().len(), 1);
    err_log += task(); // Shorthand
    assert_eq!(err_log.errors().len(), 2);
    // If the Result is a Ok value, set the Errorlog
    err_log.merge_result("ab123".parse::<i32>());
    assert_eq!(err_log.ok(), &None);
    // Shorthand
    err_log *= "42".parse::<i32>();
    assert_eq!(err_log.ok(), &Some(42));
    assert_eq!(err_log.errors().len(), 3);
    // Set Ok Value.
    err_log.set_ok(23);
    assert_eq!(err_log.ok(), &Some(23));
    *err_log = Some(123);
    assert_eq!(err_log.ok(), &Some(123));
    assert_eq!(err_log.errors().len(), 3);
    err_log
}

fn task() -> Result<(), ParseIntError> {
    let num: i32 = "a17".parse()?;
    println!("{}", num * 2);
    Ok(())
}

#[test]
fn manuelly_add_no_std_error() {
    let mut err_log = ErrorLog::<i32, String>::new();
    err_log += String::from("Manually added error");
    assert_eq!(err_log.errors().len(), 1);
    assert!(err_log.ok_or_display().is_none());
}
