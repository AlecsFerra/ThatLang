#[macro_export]
macro_rules! result_propagate_failure_to_result {
    ($maybe_error:expr) => {{
        match $maybe_error {
            Err(msg) => return Err(msg),
            Ok(expr) => expr
        }
    }};
}


#[macro_export]
macro_rules! option_propagate_failure_to_result {
    ($maybe_error:expr) => {{
        match $maybe_error {
            Some(err) => return err,
            _ => ()
        }
    }}
}

#[macro_export]
macro_rules! result_propagate_failure_to_option {
    ($maybe_error:expr) => {{
        match $maybe_error {
            Ok(t) => t,
            Err(e) => return Some(e)
        }
    }};
    ($maybe_error:expr, $msg:expr) => {{
        match $maybe_error {
            Some(t) => t,
            None => return Some($msg)
        }
    }};
}

#[macro_export]
macro_rules! option_propagate_failure_to_option {
    ($maybe_error:expr) => {{
        match $maybe_error {
            _ => (),
            Some(e) => return Some(e)
        }
    }};
}