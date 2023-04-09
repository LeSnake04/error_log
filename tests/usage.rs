use error_log::ErrorLog;
use pretty_assertions::assert_eq;
use std::num::ParseIntError;

#[test]
fn usage() {
    let out = run();
    assert_eq!(out.ok(), &Some(123));
    assert!(!out.entries().is_empty());
    // Get Number or if no Ok value set, output errors, then panic.
    let num: i32 = out.display_unwrap();
    println!("Out: {}", num);
}

fn run() -> ErrorLog<i32, ParseIntError> {
    let mut err_log = ErrorLog::new();
    // Add the potentital error from a funktion
    err_log.push_result(task());
    assert_eq!(err_log.entries().len(), 1);
    #[cfg(feature = "helper-traits")]
    {
        err_log += task(); // Shorthand
        assert_eq!(err_log.entries().len(), 2);
    }
    // If the Result is a Ok value, set the Errorlog, e
    err_log.merge_result("ab123".parse::<i32>());
    assert_eq!(err_log.ok(), &None);
    #[cfg(feature = "helper-traits")]
    {
        err_log *= "42".parse::<i32>(); // Shorthand
        err_log *= "a42".parse::<i32>(); // Shorthand
        assert_eq!(err_log.ok(), &Some(42));
        assert_eq!(err_log.entries().len(), 3);
    }
    // Set Ok Value.
    err_log.set_ok(123);
    assert_eq!(err_log.ok(), &Some(123));
    #[cfg(feature = "helper-traits")]
    {
        *err_log = Some(123);
        assert_eq!(err_log.ok(), &Some(123));
        assert_eq!(err_log.entries().len(), 2);
    }
    #[cfg(not(feature = "helper-traits"))]
    assert_eq!(err_log.entries().len(), 2);
    err_log
}

fn task() -> Result<(), ParseIntError> {
    let num: i32 = "a17".parse()?;
    println!("{}", num * 2);
    Ok(())
}

#[test]
fn manuelly_add_no_std_error() {
    #[allow(unused_mut)]
    let mut err_log = ErrorLog::<i32, String>::new();
    #[cfg(feature = "helper-traits")]
    {
        err_log += String::from("Manually added error");
        assert_eq!(err_log.entries().len(), 1);
    }
    assert!(err_log.display_ok().is_none());
}
