#[macro_export]
/**
Macro to [push_result()][crate::ErrorLog::push_result] and make funktion return given [ErrorLog][crate::ErrorLog] Variable if given result is an error.

Same as
#[cfg_attr(tarpaulin, ignore)]
```
# use error_log::ErrorLog;

# fn run() -> ErrorLog<i32, std::num::ParseIntError> {
    let mut err_log = ErrorLog::<i32, std::num::ParseIntError>::new();
    match "a1".parse::<i32>() {
        Ok(o) => err_log.set_ok(o),
        Err(e) => {
            err_log.push_err(e);
            return err_log;
        }
    };
    # err_log
# }
*/
macro_rules! try_add {
    ($res: expr, $errlog: ident) => {
        match $errlog.push_result($res) {
            Some(o) => o,
            None => return $errlog,
        }
    };
}

#[macro_export]
/**
Macro  to [merge_result()][crate::ErrorLog::merge_result] and makes funktion return given [ErrorLog][crate::ErrorLog] Variable if given result is an error.

Same as
```
# use error_log::ErrorLog;

# fn run() -> ErrorLog<i32, std::num::ParseIntError> {
    let mut err_log = ErrorLog::<i32, std::num::ParseIntError>::new();
    match "a1".parse::<i32>() {
        Ok(o) => err_log.set_ok(o),
        Err(e) => {
            err_log.push_err(e);
            return err_log;
        }
    };
    # err_log
# }
```
*/
macro_rules! try_merge {
    ($res: expr, $errlog: ident) => {
        if !$errlog.merge_result($res) {
            return $errlog;
        }
    };
}

#[macro_export]
/**
Attach error to and return given [ErrorLog][crate::ErrorLog]
*/
macro_rules! return_err {
    ($err: expr, $errlog: ident) => {
        $errlog.push_err(e);
        return $errlog;
    };
}

#[macro_export]
/**
Set ok value of and return given [ErrorLog][crate::ErrorLog]


*/
macro_rules! return_ok {
    ($err: expr, $errlog: ident) => {
        $errlog.set_ok(e);
        return $errlog;
    };
}
