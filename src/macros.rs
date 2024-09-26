macro_rules! writeln_to_handle_if_not_empty {
    ($handle:expr, $entry:expr, $value:expr) => {
        match $value {
            value if !value.to_string().is_empty() => {
                writeln!($handle, "\x1B[0;35m   {}\x1B[0m ~ {}", $entry, value).unwrap();
            }
            _ => {}
        }
    };
}

macro_rules! get_env_var {
    ($var:expr) => {
        std::env::var($var).unwrap_or_else(|_| String::new())
    };
}
